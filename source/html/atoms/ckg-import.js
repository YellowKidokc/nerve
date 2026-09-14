(function(root,factory){const api=factory();if(typeof module==='object'&&module.exports)module.exports=api;root.CKGImport=api;})(globalThis,()=>{
 'use strict';
 function parse(text){
  const original=JSON.parse(text.replace(/^\uFEFF/,''));
  if(original?.format==='nerve-reviewed-candidate/v1'){
   const record=original.original_record?.format==='evidence-chain-markdown/v1'?parseMarkdown(original.original_json_text):parse(original.original_json_text);
   record.predicates=original.review?.predicates||record.predicates;record.answers=original.review?.answers||record.answers;record.report_fields=original.review?.report_fields||record.report_fields;record.universal=original.review?.universal||record.universal;record.metadata=original.review?.metadata||{};record.source_text=original.review?.source_text??record.source_text;record.decisions=original.review?.decisions||[];return record;
  }
  if(!original||Array.isArray(original)||typeof original!=='object')throw Error('Choose one CKG or Atlas JSON record.');
  if(!Array.isArray(original.truth_predicates))throw Error('This record has no truth_predicates array.');
  const predicates=original.truth_predicates.map((item,index)=>{
   if(typeof item==='string')return {key:`predicate:${index}`,id:`P${index+1}`,statement:item,role:'',formal:'',original:item};
   if(!item||typeof item.statement!=='string')throw Error(`Predicate ${index+1} has no statement.`);
   return {key:`predicate:${index}`,id:String(item.id||`P${index+1}`),statement:item.statement,role:String(item.role||''),formal:String(item.formal||''),original:item};
  });
  return {original,original_text:text,title:String(original.title||original.node_id||'Imported candidate'),lane:String(original.object_type||original.lane||''),predicates,answers:original.answers&&typeof original.answers==='object'?JSON.parse(JSON.stringify(original.answers)):{},universal:{...original.universal_gate},report_fields:{...original.report_fields},source_file:original.source_file||'',source_sha256:original.source_sha256||'',source_text:original.preserved_source?.text||original.source_text||'',decisions:[],status:'CANDIDATE_DRAFT'};
 }
 function parseMarkdown(text){
  const sourceMarker=/^## Complete preserved source\s*$/m.exec(text);
  if(!sourceMarker)throw Error('This Markdown is not a completed evidence-chain report: no preserved-source section.');
  const analysis=text.slice(0,sourceMarker.index),report_fields={};
  const headings=[...analysis.matchAll(/^## (.+)\r?$/gm)];
  for(let i=0;i<headings.length;i++){const h=headings[i];report_fields[h[1].trim()]=analysis.slice(h.index+h[0].length,headings[i+1]?.index||analysis.length).trim();}
  if(!report_fields['Central claim']||!report_fields['Evidence chain'])throw Error('The report needs Central claim and Evidence chain sections.');
  const titleMatch=/^title:\s*"?([^\r\n]+?)"?\s*$/m.exec(analysis);
  const source_file=/^source_file:\s*(.*?)\r?$/m.exec(analysis)?.[1]||'';
  const source_sha256=/^source_sha256:\s*([a-f0-9]{64})/m.exec(analysis)?.[1]||'';
  return {original:{format:'evidence-chain-markdown/v1',source_file,source_sha256,report_fields:{...report_fields}},original_text:text,title:titleMatch?.[1]||'Evidence-chain report',lane:'',predicates:[],answers:{},universal:{},report_fields,source_file,source_sha256,source_text:text.slice(sourceMarker.index+sourceMarker[0].length).trim(),decisions:[],status:'CANDIDATE_DRAFT'};
 }
 function decide(record,key,status,value){
  if(!['ACCEPTED','REVISED','REJECTED','DEFERRED'].includes(status))throw Error('Unknown review decision');
  const entry={id:crypto.randomUUID(),key,status,value,timestamp:new Date().toISOString(),authority:'CANDIDATE_DRAFT'};record.decisions.push(entry);return entry;
 }
 function packet(record){return {format:'nerve-reviewed-candidate/v1',status:'CANDIDATE_DRAFT',admitted_to_canon:false,original_record:record.original,original_json_text:record.original_text,import_sha256:record.import_sha256||null,review:{predicates:record.predicates,answers:record.answers,report_fields:record.report_fields,universal:record.universal,metadata:record.metadata||{},source_text:record.source_text,decisions:record.decisions},unmapped_fields_preserved:true};}
 return {parse,parseMarkdown,decide,packet};
});
