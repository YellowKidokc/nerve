const test=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const api=require('../source/html/atoms/ckg-import.js');
const jsonPath='C:/Users/David/Documents/faiththruphysics.com/80_OPERATIONS/81_API/0002-a1-2-distinction_CKG_MASTER.json';
const mdPath='//192.168.2.50/h_hp/Desktop/_____pipeline-workflows-main/EVIDENCE_CHAIN_INTAKE/OUTBOX/01_CATEGORIZED_DOMAINS/09_AXIOM_CHAIN_AND_FORMAL_AUDIT/Lean4_and_Z3_Verification_Routes/does-mathematical-truth-require-a-logos-998d0aa9.knowledge-b8ce38d0.md';
test('exact Distinction JSON maps all 12 predicates without inventing diagnostics',()=>{
 const text=fs.readFileSync(jsonPath,'utf8'),r=api.parse(text);assert.equal(r.predicates.length,12);assert.equal(r.title,'Distinction');assert.equal(Object.keys(r.answers).length,0);assert.deepEqual(r.predicates.map(p=>p.statement),JSON.parse(text).truth_predicates.map(p=>p.statement));assert.equal(r.source_sha256,JSON.parse(text).source_sha256);assert.equal(r.original_text,text);
});
test('revision preserves original, unknown fields, and candidate boundary through reimport',()=>{
 const source={title:'Synthetic',truth_predicates:[{id:'P1',statement:'Original',formal:'x=x',role:'test',extra:'keep'}],unknown:{keep:true},answers:{key:'old'}};
 const r=api.parse(JSON.stringify(source));r.predicates[0].statement='Revised';r.answers.key='new';api.decide(r,'predicate:0','REVISED','Revised');
 const exported=api.packet(r);assert.deepEqual(exported.original_record,source);assert.equal(exported.admitted_to_canon,false);const again=api.parse(JSON.stringify(exported));assert.equal(again.predicates[0].statement,'Revised');assert.equal(again.decisions.length,1);assert.equal(again.original.answers.key,'old');
});
test('exact chosen evidence-chain report imports its sections and preserves all source text',()=>{
 const text=fs.readFileSync(mdPath,'utf8'),r=api.parseMarkdown(text);assert.ok(r.report_fields['Evidence chain']);assert.ok(r.report_fields['What this does not establish']);assert.ok(r.source_text.includes('PRINCIPIA MATHEMATICA MORALIA'));assert.equal(r.predicates.length,0);assert.equal(r.original_text,text);
 const before=r.original.report_fields['Central claim'];r.report_fields['Central claim']='Synthetic review edit';assert.equal(r.original.report_fields['Central claim'],before);
 const round=api.parse(JSON.stringify(api.packet(r)));assert.equal(round.report_fields['Central claim'],'Synthetic review edit');assert.equal(round.original_text,text);
});
test('invalid or unrelated documents fail explicitly',()=>{
 assert.throws(()=>api.parse('{broken'));assert.throws(()=>api.parse('{"title":"missing predicates"}'));assert.throws(()=>api.parseMarkdown('# Ordinary raw document'));
});
