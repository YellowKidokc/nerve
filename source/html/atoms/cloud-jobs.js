(() => {
  'use strict';
  const BASE = 'http://127.0.0.1:7861';
  const KEY = 'nerve-cloud-jobs/v1';
  let history = []; try { history = JSON.parse(localStorage.getItem(KEY) || '[]'); } catch {}
  const listeners = new Set();
  function save() { localStorage.setItem(KEY,JSON.stringify(history)); listeners.forEach(fn=>fn()); }
  async function api(path, body) {
    const response = await fetch(BASE+path,{method:body?'POST':'GET',headers:body?{'Content-Type':'application/json'}:{},body:body?JSON.stringify(body):undefined,signal:AbortSignal.timeout(45000)});
    const result = await response.json(); if(!response.ok)throw new Error(result.error||`Cloud connection: ${response.status}`);return result;
  }
  async function submit(pass, request, label='Selected atom') {
    const result = await api('/jobs',{pass,request});
    if (!history.some(j=>j.id===result.id)) history.unshift({id:result.id,pass,label,status:result.status,created_at:new Date().toISOString(),run_uuid:request.builder_run_uuid});
    save(); return result.id;
  }
  async function refresh(id) {
    const result = await api('/jobs/'+id);
    const row = history.find(j=>j.id===id);if(row){row.status=result.status;row.error=result.error;row.attempts=result.attempts;save();}return result;
  }
  async function run(pass,request) {
    const id = await submit(pass,request);
    for(let i=0;i<400;i++) {
      const job=await refresh(id);
      if(job.status==='complete')return job.result;
      if(job.status==='failed')throw new Error(job.error||'Cloud job failed');
      const status=document.querySelector('#prompt-run-status');if(status)status.textContent=`Cloud ${pass}: ${job.status} · ${id.slice(0,12)} · result is saved remotely`;
      await new Promise(resolve=>setTimeout(resolve,3000));
    }
    throw new Error('Cloud job is still pending. Open Cloud jobs to retrieve its result later.');
  }
  async function sources() {
    return new Promise((resolve,reject)=>{
      const opening=indexedDB.open('atom-prompt-source-tray/v1');
      opening.onerror=()=>reject(opening.error);
      opening.onsuccess=()=>{const db=opening.result;if(!db.objectStoreNames.contains('sources')){db.close();resolve([]);return;}const req=db.transaction('sources').objectStore('sources').getAll();req.onsuccess=()=>{db.close();resolve(req.result);};req.onerror=()=>{db.close();reject(req.error);};};
    });
  }
  window.NerveCloudJobs={run,submit,refresh};
  window.addEventListener('load',()=>{
    const nav=document.querySelector('.workspace-nav');if(!nav)return;
    const button=document.createElement('button');button.type='button';button.dataset.view='cloud';button.innerHTML='04 <span>Cloud jobs<small>Queue, progress and saved results</small></span>';nav.insertBefore(button,nav.querySelector('.workspace-nav-bottom'));
    const panel=document.createElement('section');panel.className='cloud-jobs-panel';
    panel.innerHTML=`<div class="cloud-job-actions"><button type="button" id="cloud-health">Check connection</button><button type="button" id="cloud-queue-library">Queue saved sources</button><button type="button" id="cloud-refresh">Refresh jobs</button></div><p>Each source uses the selected atom's object type and current prompts. Sources are sent to your Cloudflare account. Jobs generate and audit candidates; they do not admit them.</p><label>Maximum sources this submission <input id="cloud-batch-limit" type="number" min="1" max="500" value="10"></label><p id="cloud-status" role="status" aria-live="polite">Connection has not been checked. No batch starts automatically.</p><div id="cloud-job-list"></div><details id="cloud-result"><summary>Selected job result</summary><pre></pre></details>`;
    document.querySelector('.workspace-heading').after(panel);
    const status=panel.querySelector('#cloud-status');
    const render=()=>{
      const list=panel.querySelector('#cloud-job-list');list.replaceChildren();
      for(const job of history.slice(0,100)){
        const row=document.createElement('div');row.className='cloud-job-row';
        const name=document.createElement('span');name.textContent=job.label;
        const state=document.createElement('span');state.textContent=job.status;
        const view=document.createElement('button');view.type='button';view.textContent='Inspect result';
        view.onclick=async()=>{try{const current=await refresh(job.id);const details=panel.querySelector('#cloud-result');details.querySelector('pre').textContent=JSON.stringify(current,null,2);details.open=true;
          details.querySelector('.cloud-open-review')?.remove();
          if(current.status==='complete' && current.result?._object_type){
            const review=document.createElement('button');review.type='button';review.className='cloud-open-review';review.textContent='Open as new candidate for review';
            review.onclick=()=>{document.dispatchEvent(new CustomEvent('nerve-cloud-candidate',{detail:current.result}));nav.querySelector('[data-view="review"]').click();};
            details.querySelector('summary').after(review);
          }
        }catch(error){status.textContent=error.message;}};
        row.append(name,state,view);list.append(row);
      }
    };listeners.add(render);render();
    panel.querySelector('#cloud-health').onclick=async()=>{try{const result=await api('/health');status.textContent=`Connected · ${result.model} · candidate generation only`;}catch(error){status.textContent=`Not connected: ${error.message}. Start the Nerve cloud bridge.`;}};
    panel.querySelector('#cloud-refresh').onclick=async()=>{try{for(const job of history.filter(j=>!['complete','failed'].includes(j.status)))await refresh(job.id);status.textContent='Job statuses refreshed.';}catch(error){status.textContent=error.message;}};
    panel.querySelector('#cloud-queue-library').onclick=async event=>{
      const submitButton=event.currentTarget;submitButton.disabled=true;
      try{
        const records=await sources();if(!records.length)throw new Error('Import source files first.');
        const store=new AtomPromptRail.AppendOnlyStore(localStorage);
        const config=AtomPromptRail.collectConfig(document,store);
        const type=config.acceptedContext.target_atom?.object_type;
        if(!type)throw new Error('In Build, select an object type before queueing sources.');
        const limit=Number(panel.querySelector('#cloud-batch-limit').value);if(!Number.isInteger(limit)||limit<1||limit>500)throw new Error('Choose a submission size between 1 and 500.');
        let count=0;
        for(const record of records.slice(0,limit)){
          const text=await record.blob.text();
          const sourceConfig={...config,sourceText:text,sourceCoordinates:`${record.path} · whole file · ${record.size} bytes`,acceptedContext:{target_atom:{object_type:type},source_path:record.path}};
          const request=await AtomPromptRail.createBuilderRequest(sourceConfig);
          await submit('pipeline',request,`${record.path} · ${type}`);count++;
          status.textContent=`${count} of ${Math.min(limit,records.length)} sources queued. Submitted jobs continue in Cloudflare.`;
        }
        status.textContent=`${count} sources submitted for generation and audit. Refresh jobs to inspect results.`;
      }catch(error){status.textContent=`Submission stopped: ${error.message}. Already submitted jobs remain saved.`;}finally{submitButton.disabled=false;}
    };
  });
})();
