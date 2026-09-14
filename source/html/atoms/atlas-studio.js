(() => {
 'use strict';
 const STORAGE='nerve-atlas-review/v1', PROMPTS='nerve-atlas-prompts/v1';
 let records=[],selected=-1,prompts={};
 const byId=id=>document.getElementById(id);
 const questions=Object.values(SECTIONS_DATA).flat();
 const systemDefault='Evaluate this diagnostic question using the preserved source. Distinguish formal, empirical, semantic and theological claims. State missing evidence explicitly. Return a candidate answer, never a canon admission or invented verification.';
 const current=()=>records[selected];
 function persist(){localStorage.setItem(STORAGE,JSON.stringify({records,selected}));}
 function notice(message){byId('studio-import-status').textContent=message;byId('api-activity-msg').textContent=message;}
 function exportFile(name,text,type){const a=document.createElement('a');const url=URL.createObjectURL(new Blob([text],{type}));a.href=url;a.download=name;a.click();setTimeout(()=>URL.revokeObjectURL(url),30000);}
 function decisionControls(key,value){
  const bar=document.createElement('div');bar.className='studio-actions';
  for(const [label,status] of [['Accept','ACCEPTED'],['Save revision','REVISED'],['Reject','REJECTED'],['Defer','DEFERRED']]){
   const button=document.createElement('button');button.type='button';button.textContent=label;button.dataset.decision=status;
   button.onclick=()=>{const record=current();if(!record){notice('Import a candidate record before recording a decision.');return;}const val=value();CKGImport.decide(record,key,status,val);persist();bar.querySelector('.studio-decision').textContent=status;updateReview();};bar.append(button);
  }
  const badge=document.createElement('span');badge.className='studio-decision';badge.textContent=current()?.decisions.filter(d=>d.key===key).at(-1)?.status||'PROPOSED';bar.append(badge);return bar;
 }
 function renderPredicates(){
  const record=current();if(!record)return;
  const box=byId('predicates-container');box.replaceChildren();
  for(const p of record.predicates){
   const card=document.createElement('div');card.className='studio-predicate';
   const top=document.createElement('div');top.className='studio-predicate-top';
   const id=document.createElement('strong');id.textContent=p.id;const role=document.createElement('span');role.textContent=p.role||'Role not supplied';top.append(id,role);
   const input=document.createElement('input');input.type='text';input.className='form-input pred-text';input.value=p.statement;input.setAttribute('aria-label',p.id+' statement');
   input.oninput=()=>{p.statement=input.value;persist();};
   const formal=document.createElement('pre');formal.textContent=p.formal||'No formal expression supplied.';
   card.append(top,input,formal,decisionControls(p.key,()=>input.value));box.append(card);
  }
 }
 function updateReview(){
  const record=current(),list=byId('studio-review-list');list.replaceChildren();
  if(!record){list.textContent='Import a completed JSON record to begin review.';return;}
  const latest=new Map(record.decisions.map(d=>[d.key,d]));
  byId('studio-review-summary').textContent=`${record.predicates.length} predicates · ${Object.keys(record.report_fields||{}).length} report sections · ${[...latest.values()].filter(d=>d.status==='ACCEPTED'||d.status==='REVISED').length} accepted or revised decisions · candidate only`;
  for(const p of record.predicates){const card=document.createElement('div');card.className='studio-predicate';const h=document.createElement('div');h.className='studio-predicate-top';h.textContent=`${p.id} · ${latest.get(p.key)?.status||'PROPOSED'}`;const text=document.createElement('p');text.textContent=p.statement;const formal=document.createElement('pre');formal.textContent=p.formal;card.append(h,text,formal,decisionControls(p.key,()=>p.statement));list.append(card);}
  for(const [key,value] of Object.entries(record.answers)){const card=document.createElement('div');card.className='studio-predicate';const h=document.createElement('div');h.className='studio-predicate-top';h.textContent=key;const text=document.createElement('p');text.textContent=typeof value==='string'?value:JSON.stringify(value);card.append(h,text,decisionControls(key,()=>value));list.append(card);}
  for(const [key,value] of Object.entries(record.report_fields||{})){const card=document.createElement('div');card.className='studio-predicate';const h=document.createElement('div');h.className='studio-predicate-top';h.textContent=key;const input=document.createElement('textarea');input.className='form-textarea';input.value=String(value);input.setAttribute('aria-label',key+' review');input.onchange=()=>{record.report_fields[key]=input.value;persist();};card.append(h,input,decisionControls('report:'+key,()=>input.value));list.append(card);}
 }
 function unresolvedSelect(id,value){const el=byId(id);if(!el)return;if(![...el.options].some(o=>o.value===''))el.add(new Option('Not supplied — needs review',''),0);el.value=[...el.options].some(o=>o.value===value)?value:'';}
 function selectRecord(index){
  selected=index;const r=current();if(!r)return;
  activeRecord={...activeRecord,uuid:crypto.randomUUID(),node_id:r.original.node_id||r.original.id||'',standing:'CANDIDATE',answers:{...r.answers}};
  byId('active-uuid-badge').textContent=r.title+' · CANDIDATE';byId('node-id-input').value=activeRecord.node_id;
  unresolvedSelect('active-lane-select',r.lane);unresolvedSelect('global-domain-home',r.original.domains?.home||'');unresolvedSelect('global-warrant-select',r.original.warrant||'');
  for(const [id,value] of Object.entries(r.metadata||{})){const control=byId(id);if(control)control.value=value;}
  byId('why-closure-display').value=r.original.why_closure||'NOT EVALUATED';
  byId('raw-source-box').value=r.source_text;
  byId('raw-source-box').placeholder=r.source_file?'Original text is not embedded in this JSON. Add the source text before AI generation.':'Paste preserved source text here.';
  byId('preview-claim-text').textContent=r.original.claim_text||r.report_fields?.['Central claim']||r.predicates[0]?.statement||'';
  const universal={q15_smallest_unit:'field-univ-q15',q16_assertion_mode:'field-univ-q16',q17_level:'field-univ-q17',q18_domain_ablated:'field-univ-q18'};
  for(const [key,id] of Object.entries(universal)){const input=byId(id);if(input)input.value=r.universal[key]||'';}
  for(const id of ['field-univ-q16-exp','field-univ-q17-exp'])byId(id).value=r.universal[id]||'';
  for(const q of questions){const row=byId('row-'+q.qid),input=row?.querySelector('textarea');if(input)input.value=typeof r.answers[q.field_key]==='string'?r.answers[q.field_key]:'';const badge=byId('badge-'+q.qid);if(badge)badge.textContent=input?.value?'PROPOSED':'MISSING';row?.querySelector('.studio-field-decisions')?.remove();if(input){const bar=decisionControls(q.field_key,()=>input.value);bar.classList.add('studio-field-decisions');row.querySelector('.qa-right-substance')?.append(bar);}}
  if(byId('active-lane-select').value && typeof filterActiveLane==='function')filterActiveLane(byId('active-lane-select').value);
  renderPredicates();updateReview();persist();
  byId('studio-record-select').value=String(index);
  byId('studio-metadata').textContent=`${r.title} · ${r.predicates.length} predicates\nSource: ${r.source_file||'not supplied'}\nSource hash (as supplied): ${r.source_sha256||'not supplied'}\nImported file SHA-256: ${r.import_sha256}\n${Object.keys(r.answers).length} explicit diagnostic answers. Unmapped fields remain in the original JSON.`;
  notice(`Loaded ${r.title}: ${r.predicates.length} predicates and ${Object.keys(r.report_fields||{}).length} report sections. Missing diagnostic answers remain blank.`);
 }
 async function importTexts(items){
  const pending=[];
  for(const item of items){const record=/\.md$/i.test(item.name)?CKGImport.parseMarkdown(item.text):CKGImport.parse(item.text);const hash=await crypto.subtle.digest('SHA-256',new TextEncoder().encode(item.text));record.import_sha256=[...new Uint8Array(hash)].map(x=>x.toString(16).padStart(2,'0')).join('');record.import_name=item.name;if(record.title==='ckg_evaluation:')record.title=item.name.replace(/\.md$/i,'');pending.push(record);}
  records.push(...pending);
  renderSelector();selectRecord(records.length-1);switchStage(Object.keys(current().report_fields||{}).length?4:2);
 }
 function renderSelector(){const select=byId('studio-record-select');select.replaceChildren();records.forEach((r,i)=>select.add(new Option(`${i+1}. ${r.title} (${r.predicates.length} predicates)`,String(i))));}
 function attachPrompts(){
  for(const q of questions){const row=byId('row-'+q.qid);if(!row||row.querySelector('.studio-prompt-editor'))continue;const details=document.createElement('details');details.className='studio-prompt-editor';const summary=document.createElement('summary');summary.textContent='View / edit the prompt for '+q.qid;const editor=document.createElement('textarea');editor.value=prompts[q.qid]||q.prompt;editor.setAttribute('aria-label',q.qid+' generation prompt');editor.onchange=()=>{prompts[q.qid]=editor.value;localStorage.setItem(PROMPTS,JSON.stringify(prompts));};const description=document.createElement('p');description.className='studio-intro';description.textContent='The current source text and the governance instruction are sent with this prompt. Edits are saved when you leave the box.';details.append(summary,editor,description);row.querySelector('.qa-right-substance')?.append(details);}
 }
 function init(){
  document.body.classList.add('atlas-studio');
  document.querySelector('.brand-text p').textContent='SOURCE · CANDIDATE · HUMAN REVIEW';
  const center=document.querySelector('.center-content');
  const intake=document.createElement('div');intake.className='studio-import';
  intake.innerHTML=`<div class="studio-kicker">COMPLETED RECORDS</div><h2 class="studio-title">Import, then review.</h2><p class="studio-intro">Load one or several CKG JSON records. Each document stays separate; its predicates, formal expressions and source information go into matching fields.</p><div class="studio-actions"><button type="button" id="studio-pick-json">Choose JSON files</button><button type="button" id="studio-raw-batch">Process raw documents →</button></div><input id="studio-json-files" type="file" accept=".json,application/json" multiple hidden><label for="studio-paste-json">Or paste a completed JSON record</label><textarea id="studio-paste-json" placeholder="Paste CKG_RECORD_V2.0 or Atlas JSON here"></textarea><div class="studio-actions"><button type="button" id="studio-import-paste">Place JSON into fields</button></div><label for="studio-record-select">Imported documents</label><select id="studio-record-select" class="form-select"></select><pre id="studio-metadata" class="studio-notice"></pre><p id="studio-import-status" class="studio-notice" role="status">Imports create review candidates. Your original files remain intact.</p>`;
  center.prepend(intake);
  byId('studio-json-files').accept='.json,.md,application/json,text/markdown';
  byId('studio-pick-json').textContent='Choose completed records';
  intake.querySelector('.studio-intro').textContent='Load one or several CKG JSON or evidence-chain Markdown records. Each document stays separate. Review the original assessment, edit it, and record your own decisions.';
  const review=document.createElement('div');review.className='studio-review';review.innerHTML='<div class="studio-kicker">YOUR DECISIONS</div><h2 class="studio-title">Accept, revise, reject, or defer.</h2><p id="studio-review-summary" class="studio-intro">Every value remains a proposal until you review it.</p><div id="studio-review-list" class="studio-review-list"></div>';center.append(review);
  const exporting=document.createElement('div');exporting.className='studio-export';exporting.innerHTML='<div class="studio-kicker">REVIEWED CANDIDATE</div><h2 class="studio-title">Keep the source and your decisions.</h2><p class="studio-intro">The reviewed JSON retains the complete original record, edited values, and the decision history. The Markdown export is its readable companion.</p><div class="studio-actions"><button id="studio-export-json" type="button">Export reviewed JSON</button><button id="studio-export-md" type="button">Export review Markdown</button></div>';center.append(exporting);
  const stages=['1. Import','2. Predicates','3. Diagnostics','4. Review','5. Export'];
  document.querySelectorAll('.stage-step').forEach((el,i)=>{el.textContent=stages[i];el.setAttribute('role','button');el.tabIndex=0;el.onkeydown=e=>{if(e.key==='Enter'||e.key===' '){e.preventDefault();switchStage(i+1);}};});
  window.switchStage=step=>{document.body.dataset.studioStep=String(step);document.querySelectorAll('.stage-step').forEach((el,i)=>{el.classList.toggle('active',i+1===step);el.setAttribute('aria-pressed',String(i+1===step));});if(step===4)updateReview();};
  const sidebar=document.createElement('div');sidebar.className='studio-sidebar-import';sidebar.innerHTML='<button class="hbtn hbtn-gold" id="studio-import-nav">Import completed JSON</button><button class="hbtn" id="studio-batch-nav">Batch raw documents</button><button class="hbtn" id="studio-cloud-nav">Cloud job control</button>';
  document.querySelector('.sidebar-left').append(sidebar);
  // Keep the existing batch engine and automatic Markdown export intact.
  document.querySelectorAll('.sidebar-left>.card').forEach(card=>{if(card.textContent.includes('Ready for folder or multi-file intake'))card.style.display='none';});
  byId('studio-import-nav').onclick=()=>switchStage(1);byId('studio-batch-nav').onclick=()=>openDeepIntakeModal();byId('studio-raw-batch').onclick=()=>openDeepIntakeModal();byId('studio-cloud-nav').onclick=()=>{location.href='atom-builder.html';};
  byId('studio-pick-json').onclick=()=>byId('studio-json-files').click();
  byId('studio-json-files').onchange=async event=>{try{const items=await Promise.all([...event.target.files].map(async file=>({name:file.name,text:await file.text()})));await importTexts(items);}catch(error){notice('Import stopped: '+error.message);}finally{event.target.value='';}};
  byId('studio-import-paste').onclick=()=>importTexts([{name:'pasted.json',text:byId('studio-paste-json').value}]).catch(error=>notice('Import stopped: '+error.message));
  byId('studio-record-select').onchange=event=>selectRecord(Number(event.target.value));
  byId('studio-export-json').onclick=()=>{const r=current();if(r)exportFile(r.title.replace(/[^a-z0-9_-]/gi,'_')+'_reviewed.json',JSON.stringify(CKGImport.packet(r),null,2),'application/json');else notice('Import a record first.');};
  byId('studio-export-md').onclick=()=>{const r=current();if(!r){notice('Import a record first.');return;}const decisions=new Map(r.decisions.map(d=>[d.key,d]));const text=['---','status: CANDIDATE_DRAFT','admitted_to_canon: false','---','',`# ${r.title} — review`,'',...r.predicates.flatMap(p=>[`## ${p.id} — ${decisions.get(p.key)?.status||'PROPOSED'}`,'',p.statement,'',p.formal,'']),...Object.entries(r.report_fields||{}).flatMap(([key,value])=>[`## ${key}`,'',`Review: ${decisions.get('report:'+key)?.status||'PROPOSED'}`,'',value,'']), '## Original record','', '````',r.original_text,'````','', '## Decision history','', '```json',JSON.stringify(r.decisions,null,2),'```'].join('\n');exportFile(r.title.replace(/[^a-z0-9_-]/gi,'_')+'_review.md',text,'text/markdown');};
  const headerJson=document.querySelector('header [onclick="exportAtlasJSON()"]');if(headerJson)headerJson.onclick=()=>current()?byId('studio-export-json').click():exportAtlasJSON();
  const nativeHealth=window.checkBackendHealth;
  window.checkBackendHealth=async()=>{if(byId('api-provider-select').value!=='cloudflare')return nativeHealth();try{const response=await fetch('http://127.0.0.1:7861/health');if(!response.ok)throw Error('Bridge unavailable');const data=await response.json();notice('Connected · Cloudflare · '+data.model);}catch{notice('Cloud bridge is not running. Start Nerve cloud bridge on this desktop.');}};
  const nativeConfig=window.updateApiConfig;
  window.updateApiConfig=()=>{nativeConfig();if(byId('api-provider-select').value==='cloudflare'){byId('cf-worker-url').value='http://127.0.0.1:7861';byId('cf-worker-url').style.display='none';byId('api-model-select').replaceChildren(new Option('Llama 3.3 70B · Cloudflare','@cf/meta/llama-3.3-70b-instruct-fp8-fast'));}};
  try{const saved=JSON.parse(localStorage.getItem(STORAGE)||'null');if(saved?.records){records=saved.records;selected=saved.selected;}prompts=JSON.parse(localStorage.getItem(PROMPTS)||'{}');}catch{}
  attachPrompts();renderSelector();if(records[selected])selectRecord(selected);else activeRecord.standing='CANDIDATE';switchStage(1);
  const nativeAnswer=window.handleAnswerInput;window.handleAnswerInput=(qid,key,value)=>{nativeAnswer(qid,key,value);if(current()){current().answers[key]=value;persist();}const badge=byId('badge-'+qid);if(badge)badge.textContent=value.trim()?'PROPOSED':'MISSING';};
  const universalKeys={'field-univ-q15':'q15_smallest_unit','field-univ-q16':'q16_assertion_mode','field-univ-q17':'q17_level','field-univ-q18':'q18_domain_ablated','field-univ-q16-exp':'field-univ-q16-exp','field-univ-q17-exp':'field-univ-q17-exp'};
  for(const [id,key] of Object.entries(universalKeys)){byId(id)?.addEventListener('change',event=>{if(current()){current().universal[key]=event.target.value;persist();}});}
  for(const id of ['node-id-input','active-lane-select','global-domain-home','global-domain-bridged','global-warrant-select']){byId(id)?.addEventListener('change',event=>{if(current()){current().metadata||={};current().metadata[id]=event.target.value;persist();}});}
  byId('raw-source-box').addEventListener('change',event=>{if(current()){current().source_text=event.target.value;persist();}});
  window.runSingleQuestion=async qid=>{
   const q=questions.find(q=>q.qid===qid),row=byId('row-'+qid);if(!q||!row)return;
   const source=byId('raw-source-box').value.trim();if(!source){notice('Add preserved source text before generating an answer.');return;}
   const question=row.querySelector('.studio-prompt-editor textarea').value;const input=row.querySelector('textarea');
   notice('Generating a candidate answer for '+qid+'…');
   try{const answer=await callLLM(`PRESERVED SOURCE:\n${source}\n\nQUESTION:\n${question}`,systemDefault);if(typeof answer!=='string'||!answer.trim())throw Error('No answer returned');input.value=answer;handleAnswerInput(qid,q.field_key,answer);notice(qid+' candidate answer ready for review.');}catch(error){notice('Generation failed: '+error.message);}
  };
  window.runLeanVerifyCall=()=>notice('Lean verification is not connected here. No proof check was performed.');
  const prototypeLean=byId('accordion-lean-summary-text')?.closest('.lean-summary-badge')?.parentElement?.parentElement;
  if(prototypeLean){prototypeLean.style.display='none';const explanation=document.createElement('p');explanation.className='studio-notice';explanation.textContent='Formal verification: no imported or verified Lean receipt. The former demonstration results are hidden. Formal diagnostic questions remain available below.';prototypeLean.before(explanation);}
  window.openLeanModal=()=>notice('No verified Lean receipt is attached to this imported record.');
  window.exportLeanCodeFile=()=>notice('No source-bound Lean proof is attached for export.');
  window.autoClassifyRow=()=>notice('Automatic classification is not connected. Choose classifications explicitly after review.');
  window.autoClassifyActiveSection=()=>notice('Automatic section classification is not connected.');
  byId('btn-toggle-cf').textContent='Cloudflare Workers';
  const cloudOption=byId('api-provider-select').querySelector('option[value="cloudflare"]');if(cloudOption)cloudOption.textContent='Cloudflare Workers AI';
  byId('api-activity-msg').textContent='Ready · candidate review · no verification run';
  const nativeLLM=window.callLLM;
  window.callLLM=async(prompt,system)=>{
   if(byId('api-provider-select').value!=='cloudflare')return nativeLLM(prompt,system);
   const result=await NerveCloudJobs.run('pipeline',{pass:'BUILDER',authority:'CANDIDATE_DRAFT — NOT ADMITTED',builder_run_uuid:crypto.randomUUID(),global_prompt:systemDefault,builder_prompt:system,prompt_versions:{global:'atlas-studio-v1'},preserved_source:{text:prompt,coordinates:'Atlas current source and prompt'},field_prompts:{answer:{question:'Follow the requested task exactly. Put the complete response in suggested_value as a string. If JSON was requested, that string must contain the requested JSON.',expected_type:'string'}},response_schema:{candidate_packet_status:'CANDIDATE_DRAFT — NOT ADMITTED',field_results:[{field_key:'answer',suggested_value:'',source_anchors:[],confidence:0,warnings:[],answer_status:'PROPOSED'}],open_discoveries:[]}});
   const value=result.field_results.find(f=>f.field_key==='answer')?.suggested_value;if(typeof value!=='string')throw Error('Cloud response did not contain an answer string');return value;
  };
 }
 window.addEventListener('load',init);
})();
