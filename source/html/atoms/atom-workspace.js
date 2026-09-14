/* Presentation layer: existing controls, draft storage and review gates remain authoritative. */
(() => {
  'use strict';
  window.addEventListener('load', () => {
    const command = document.querySelector('.prompt-command');
    const app = document.querySelector('.app');
    if (!command || !app) return;
    document.body.classList.add('atom-workspace');
    const nav = document.createElement('nav');
    nav.className = 'workspace-nav';
    nav.setAttribute('aria-label', 'Atom workspace');
    nav.innerHTML = `<div class="workspace-brand">NERVE<span>ATOM STUDIO</span></div>
      <p class="workspace-nav-label">WORKSPACE</p>
      <button type="button" data-view="build">01 <span>Build atoms<small>Source to structured proposal</small></span></button>
      <button type="button" data-view="prompts">02 <span>Prompts<small>Inspect each field's instructions</small></span></button>
      <button type="button" data-view="review">03 <span>Review<small>Answers, anchors and decisions</small></span></button>
      <div class="workspace-nav-bottom"><p class="workspace-nav-label">SOURCE LIBRARY</p>
      <button type="button" data-view="import">↓ <span>Import sources<small>Files, folders and saved tranches</small></span></button>
      <p class="workspace-boundary">Candidate workspace<br>Admission requires separate review.</p></div>`;
    app.prepend(nav);
    const heading = document.createElement('section');
    heading.className = 'workspace-heading';
    heading.innerHTML = '<p class="workspace-eyebrow">SOURCE → PROPOSAL → AUDIT → HUMAN REVIEW</p><h2></h2><p class="workspace-description"></p>';
    command.before(heading);
    const empty = document.querySelector('#emp');
    if (empty) document.querySelector('#ac').before(empty);
    const views = {
      build: ['Build atoms', 'Choose the source and object type, then generate a candidate for review.'],
      prompts: ['Make the instructions visible', 'Expand any field to inspect and edit its prompt. Lock Prompt saves a version; generation uses the current text.'],
      review: ['Review the reasoning', 'Inspect proposed values, source anchors and warnings before accepting a draft value.'],
      import: ['Prepare your sources', 'Add files in tranches. Sources stay in this browser’s library until you explicitly clear the tray.'],
      cloud: ['Cloud jobs', 'Submit independent source jobs. Each runs generation and audit in Cloudflare, with results saved for review.']
    };
    const tools = document.createElement('div');
    tools.className = 'workspace-field-tools';
    tools.innerHTML = '<button type="button" id="workspace-expand">Show all field prompts</button><button type="button" id="workspace-collapse">Collapse field details</button><span>Prompt edits apply to the next run. Use Lock Prompt to save a version.</span>';
    command.after(tools);
    const sourceTray = command.querySelector('.prompt-source-tray');
    const sourceHint = document.createElement('p');
    sourceHint.className = 'workspace-source-hint';
    sourceHint.textContent = 'Import is preparation. Loading a source chooses the material for an atom; running generates proposals and uses your selected AI provider.';
    sourceTray.prepend(sourceHint);
    const selection = document.createElement('div');
    selection.className = 'workspace-source-selection';
    const choose = document.createElement('button');
    choose.type = 'button'; choose.textContent = 'Choose / import source';
    choose.onclick = () => setView('import');
    const sourceState = document.createElement('span');
    const refreshSource = () => { sourceState.textContent = command.querySelector('#source-tray-summary')?.textContent || 'No source selected'; };
    selection.append(choose, sourceState); command.prepend(selection);
    new MutationObserver(refreshSource).observe(command.querySelector('#source-tray-summary'), {childList: true, subtree: true});
    refreshSource();
    const endpoint = command.querySelector('#prompt-endpoint')?.closest('label');
    if (endpoint) {
      const advanced = document.createElement('details'); advanced.className = 'workspace-connection';
      const summary = document.createElement('summary'); summary.textContent = 'Connection settings';
      advanced.append(summary); endpoint.before(advanced); advanced.append(endpoint);
    }
    command.querySelector('#run-builder').textContent = 'Generate + audit selected atom';
    command.querySelector('#accept-all-proposals').textContent = 'Accept proposed draft values';
    command.querySelector('#run-all-four').textContent = 'Generate all four object types';
    command.querySelector('#source-tray-batch-run').textContent = 'Generate from all saved sources';
    command.querySelector('#prompt-run-status').setAttribute('role', 'status');
    command.querySelector('#prompt-run-status').setAttribute('aria-live', 'polite');
    const importNext = document.createElement('button');
    importNext.type = 'button'; importNext.className = 'workspace-import-next';
    importNext.textContent = 'Continue to Build →'; importNext.onclick = () => setView('build'); sourceTray.append(importNext);
    function showRails(open) {
      document.querySelectorAll('#ac .prompt-rail').forEach(rail => { rail.open = open; });
      if (open) document.querySelectorAll('#ac .step').forEach(step => step.classList.add('open'));
    }
    function setView(view) {
      document.body.dataset.workspaceView = view;
      nav.querySelectorAll('[data-view]').forEach(button => {
        if (button.dataset.view === view) button.setAttribute('aria-current', 'page');
        else button.removeAttribute('aria-current');
      });
      heading.querySelector('h2').textContent = views[view][0];
      heading.querySelector('.workspace-description').textContent = views[view][1];
      if (view === 'prompts' || view === 'review') showRails(true);
      document.querySelector('#cf').scrollTop = 0;
    }
    nav.addEventListener('click', event => { const button = event.target.closest('[data-view]'); if (button) setView(button.dataset.view); });
    tools.querySelector('#workspace-expand').onclick = () => showRails(true);
    tools.querySelector('#workspace-collapse').onclick = () => showRails(false);
    setView('build');
  });
})();
