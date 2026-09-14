import assert from 'node:assert/strict';
import {CANDIDATE} from './worker.mjs';
const request={
 authority:CANDIDATE, builder_run_uuid:crypto.randomUUID(), pass:'BUILDER',
 global_prompt:'Proposals only. No canon admission.',builder_prompt:'Extract exactly one claim from this synthetic fixture.',
 prompt_versions:{global:'synthetic-smoke-v1'},preserved_source:{text:'A triangle has three sides.',coordinates:'synthetic fixture, line 1'},
 field_prompts:{claim:{question:'What does the source state?',expected_type:'string'}},
 response_schema:{builder_run_uuid:'APPLICATION_GENERATED_UUID',candidate_packet_status:CANDIDATE,field_results:[{field_key:'claim',enabled:true,suggested_value:'',allowed_values:[],rationale:'',source_anchors:['synthetic fixture, line 1'],confidence:0.9,warnings:[],answer_status:'PROPOSED'}],open_discoveries:[]}
};
const response=await fetch('http://127.0.0.1:7861/jobs',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({pass:'pipeline',request})});
const submitted=await response.json();assert.equal(response.status,202,JSON.stringify(submitted));
console.log('Submitted synthetic pipeline:',submitted.id);
let previous='';
for(let attempt=0;attempt<100;attempt++){
 const current=await (await fetch('http://127.0.0.1:7861/jobs/'+submitted.id)).json();
 if(current.status!==previous){console.log('Status:',current.status);previous=current.status;}
 if(current.status==='failed')throw Error(current.error);
 if(current.status==='complete'){
  assert.equal(current.result.candidate_packet_status,CANDIDATE);
  assert.equal(current.result.builder_run_uuid,request.builder_run_uuid);
  assert.ok(current.result._audit.repaired_candidate_packet);
  console.log('PASS: desktop bridge -> queue -> generation -> audit -> persisted candidate result.');
  console.log('Receipt:',JSON.stringify(current.result._cloud_receipt));
  process.exit(0);
 }
 await new Promise(resolve=>setTimeout(resolve,3000));
}
throw Error('Job still pending; inspect saved job id.');
