import test from 'node:test';
import assert from 'node:assert/strict';
import {DatabaseSync} from 'node:sqlite';
import {readFileSync} from 'node:fs';
import worker, {CANDIDATE, validateOutput, decodeModelOutput} from './worker.mjs';

function fixture() {
  const db = new DatabaseSync(':memory:'); db.exec(readFileSync(new URL('./schema.sql',import.meta.url),'utf8'));
  const packet = {candidate_packet_status:CANDIDATE,field_results:[{field_key:'claim',source_anchors:[],confidence:0.5,answer_status:'PROPOSED'}],open_discoveries:[]};
  const sent=[]; let calls=0;
  const env={
    NERVE_ATOM_TOKEN:'test', AI_MODEL:'test-model',
    DB:{prepare(sql){
      return {bind(...args){
        return {
          async run(){const r=db.prepare(sql).run(...args);return {meta:{changes:Number(r.changes)}};},
          async first(){return db.prepare(sql).get(...args)||null;}
        };
      }};
    }},
    JOBS:{async send(value){sent.push(value);}},
    AI:{async run(){calls++;return {response:JSON.stringify(packet)};}}
  };
  return {env,db,sent,get calls(){return calls;}};
}
const submit = (env,body) => worker.fetch(new Request('https://test/jobs',{method:'POST',headers:{Authorization:'Bearer test'},body:JSON.stringify(body)}),env);
const requestBody = {pass:'builder',request:{authority:CANDIDATE,builder_run_uuid:'fixture-uuid',response_schema:{},preserved_source:{text:'Synthetic test only'}}};
test('decodes both Workers AI and chat completion envelopes',()=>{
 assert.deepEqual(decodeModelOutput({response:'{"value":1}'}),{value:1});
 assert.deepEqual(decodeModelOutput({choices:[{message:{content:'{"value":2}'}}]}),{value:2});
});
function message(id){return {body:{id},acked:false,retried:false,ack(){this.acked=true;},retry(){this.retried=true;}};}
test('authentication fails closed',async()=>{
 const {env}=fixture();assert.equal((await worker.fetch(new Request('https://test/jobs'),env)).status,401);
 env.NERVE_ATOM_TOKEN='';assert.equal((await worker.fetch(new Request('https://test/health'),env)).status,503);
});
test('same snapshot is idempotent, and completed delivery does not run AI again',async()=>{
 const f=fixture();const first=await (await submit(f.env,requestBody)).json();const second=await (await submit(f.env,requestBody)).json();assert.equal(first.id,second.id);
 const a=message(first.id);await worker.queue({messages:[a]},f.env);assert.equal(a.acked,true);assert.equal(f.calls,1);
 const b=message(first.id);await worker.queue({messages:[b]},f.env);assert.equal(b.acked,true);assert.equal(f.calls,1);
 const result=await worker.fetch(new Request('https://test/jobs/'+first.id,{headers:{Authorization:'Bearer test'}}),f.env);
 assert.equal((await result.json()).status,'complete');
});
test('active lease defers duplicate and expired lease can recover',async()=>{
 const f=fixture();const {id}=await (await submit(f.env,requestBody)).json();
 f.db.prepare("UPDATE jobs SET status='running',lease_until=? WHERE id=?").run(Date.now()+50000,id);
 const busy=message(id);await worker.queue({messages:[busy]},f.env);assert.equal(busy.retried,true);assert.equal(f.calls,0);
 f.db.prepare('UPDATE jobs SET lease_until=0 WHERE id=?').run(id);
 const recovered=message(id);await worker.queue({messages:[recovered]},f.env);assert.equal(recovered.acked,true);assert.equal(f.calls,1);
});
test('failed inference retries and becomes visible terminal failure',async()=>{
 const f=fixture();f.env.AI.run=async()=>{throw Error('temporary provider error');};const {id}=await (await submit(f.env,requestBody)).json();
 for(let i=0;i<5;i++){const msg=message(id);await worker.queue({messages:[msg]},f.env);assert.equal(i<4?msg.retried:msg.acked,true);}
 assert.equal(f.db.prepare('SELECT status FROM jobs WHERE id=?').get(id).status,'failed');
});
test('rejects authority escalation, malformed payloads and oversized jobs',async()=>{
 const f=fixture();assert.equal((await submit(f.env,{...requestBody,request:{...requestBody.request,authority:'CANON'}})).status,400);
 assert.equal((await submit(f.env,{...requestBody,padding:'x'.repeat(61000)})).status,413);
 assert.throws(()=>validateOutput({candidate_packet_status:'ADMITTED',field_results:[],open_discoveries:[]},'builder'));
});
test('pipeline audits with original source and preserves application identity',async()=>{
 const f=fixture();const requests=[];
 f.env.AI.run=async(model,input)=>{
  const req=JSON.parse(input.messages[1].content);requests.push(req);
  const packet={candidate_packet_status:CANDIDATE,field_results:[],open_discoveries:[]};
  return {response:JSON.stringify(req.pass==='AUDIT_REPAIR'?{repaired_candidate_packet:packet}:packet)};
 };
 const {id}=await (await submit(f.env,{...requestBody,pass:'pipeline'})).json();
 await worker.queue({messages:[message(id)]},f.env);
 assert.equal(requests.length,2);assert.deepEqual(requests[1].preserved_source,requestBody.request.preserved_source);
 const row=f.db.prepare('SELECT result_json FROM jobs WHERE id=?').get(id);
 assert.equal(JSON.parse(row.result_json).builder_run_uuid,'fixture-uuid');
});
