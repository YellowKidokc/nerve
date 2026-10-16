"""Server-free panel regression checks using a test-only IPC peer.
Run: python -m pip install playwright; python -m playwright install chromium
     python source/tests/panel_ui.py
Optional: NERVE_TEST_CHROMIUM=/path/to/chromium
This tests UI messages and behavior, not Windows .bat execution or API calls.
"""
import json
import os
from pathlib import Path
from playwright.sync_api import sync_playwright, expect

PANEL = Path(__file__).resolve().parents[1] / 'html/hubs/station-console.html'
PEER = r"""
window.messages=[];
const root='D:\\pipeline\\API';
const station=(label,family,path)=>({label,family,front:root+'\\'+path,description:'Real station contract',inbox:2,outbox:1,uses_api:true,text:{'FOCUS.md':root+'\\'+path+'\\BACKSIDE\\FOCUS.md'},buttons:[{name:'1 RUN HERE',path:root+'\\'+path+'\\1 RUN HERE.bat'},{name:'2 RUN ON FOLDER',path:root+'\\'+path+'\\2 RUN ON FOLDER.bat'},...(label==='24_CRAFT_PASS'?[{name:'3 REPAIR FINISHED PAPERS',path:root+'\\'+path+'\\3 REPAIR FINISHED PAPERS.bat'}]:[])]});
const stations=[station('20_CKG_RUN','01_CKG','01_CKG\\020_CKG'),station('25_CKG_V4','01_CKG','01_CKG\\025_CKG_V4'),station('41_STORY','','041_STORIES'),station('24_CRAFT_PASS','041_STORIES','041_STORIES\\OUTBOX\\024_CRAFT_PASS'),station('10_CKG_THEOLOGY','01_CKG','01_CKG\\020_CKG\\OUTBOX\\010_CKG_THEOLOGY')];
const calls=['SYNOPSIS','LADDER','ORIGINAL'].map((id,i)=>({id,title:id,folder:'calls/'+id,page_order:i+1,...(i<2?{run_order:i+1,'prompt.md':{path:root+'\\01_CKG\\025_CKG_V4\\BACKSIDE\\calls\\'+id+'\\prompt.md',text:'Prompt '+id},'output.md':{path:root+'\\01_CKG\\025_CKG_V4\\BACKSIDE\\calls\\'+id+'\\output.md',text:'Output '+id}}:{kind:'paste_source'})}));
window.ipc={postMessage(raw){
 const m=JSON.parse(raw);window.messages.push(m);let result={ok:true};
 if(m.type==='pipeline_list')result.roots=[{root,ok:true,data:{stations}}];
 if(m.type==='pipeline_read')result.text='Standing focus';
 if(m.type==='pipeline_backside')Object.assign(result,{available:m.front.endsWith('025_CKG_V4'),calls:JSON.parse(JSON.stringify(calls))});
 if(m.type==='pipeline_results')result.files=[{name:'Paper · CKG.md',path:m.front+'\\OUTBOX\\Paper · CKG.md',modified:1700000000,bytes:300}];
 if(m.type==='pipeline_reorder') for(const c of calls)if(m.order.includes(c.id))c[m.field]=m.order.indexOf(c.id)+1;
 queueMicrotask(()=>window.onPipeline(m.type,m.request_id,result));
}};
"""

def run():
    with sync_playwright() as pw:
        options = {'headless': True}
        if os.environ.get('NERVE_TEST_CHROMIUM'):
            options['executable_path'] = os.environ['NERVE_TEST_CHROMIUM']
        browser = pw.chromium.launch(**options)
        page = browser.new_page(viewport={'width':1280, 'height':860})
        errors=[]
        page.on('pageerror', lambda e: errors.append(str(e)))
        page.add_init_script(PEER)
        page.goto(PANEL.as_uri())
        page.on('dialog', lambda dialog: dialog.accept())
        expect(page.locator('#title')).to_have_text('20_CKG_RUN')
        expect(page.locator('#stations button')).to_have_count(2)
        page.get_by_role('button', name='25_CKG_V4', exact=True).click()
        page.get_by_role('button', name='Settings', exact=True).click()
        page.locator('#workers').fill('7'); page.locator('#workers').dispatch_event('change')
        page.locator('#focus').fill('New focus'); page.locator('#save-focus').click()
        expect(page.locator('#notice')).to_contain_text('FOCUS.md saved')
        assert page.evaluate("messages.filter(m=>m.type==='pipeline_save_focus').at(-1).text") == 'New focus'
        page.get_by_role('button', name='Run', exact=True).click()
        page.locator('#button').select_option('1'); page.locator('#start').click()
        run_id=page.evaluate("messages.find(m=>m.type==='pipeline_run').run_id")
        # Split prompts and UTF-8 text arrive in independently timed messages.
        page.evaluate("id=>{stationOutput(id,'How many at');stationOutput(id,' once? (1-30, Enter = 8) ')}",run_id)
        page.wait_for_function("messages.some(m=>m.type==='pipeline_answer' && m.text==='7')")
        page.locator('#answer').fill('Y'); page.locator('#send').click()
        page.wait_for_function("messages.some(m=>m.type==='pipeline_answer' && m.text==='Y')")
        page.get_by_role('button', name='041_STORIES', exact=True).click()
        expect(page.locator('#title')).to_have_text('41_STORY')
        page.get_by_role('button',name='24_CRAFT_PASS',exact=True).click()
        expect(page.locator('#button option')).to_have_count(3)
        page.locator('#button').select_option(label='3 REPAIR FINISHED PAPERS'); page.locator('#start').click()
        assert page.evaluate("messages.filter(m=>m.type==='pipeline_run').at(-1).button.endsWith('3 REPAIR FINISHED PAPERS.bat')")
        page.evaluate("id=>stationOutput(id,'SYNOPSIS → LADDER → AUDIT · 完了')",run_id)
        page.get_by_role('button', name='01_CKG', exact=True).click()
        page.get_by_role('button',name='25_CKG_V4 · running',exact=True).click()
        expect(page.locator('#term')).to_contain_text('SYNOPSIS → LADDER → AUDIT · 完了')
        page.evaluate("id=>stationExit(id,0)",run_id)
        expect(page.locator('#run-status')).to_have_text('Exit 0')
        page.get_by_role('button',name='Results',exact=True).click()
        page.locator('.result').click()
        assert page.evaluate("messages.filter(m=>m.type==='pipeline_open_result').at(-1).path.endsWith('Paper · CKG.md')")
        page.get_by_role('button',name='Backside',exact=True).click()
        expect(page.locator('#calls button')).to_have_count(2)
        page.locator('#call-prompt').fill('Edited prompt'); page.locator('#call-output').fill('Edited output'); page.locator('#save-call').click()
        expect(page.locator('#notice')).to_contain_text('Call files saved')
        assert page.evaluate("messages.filter(m=>m.type==='pipeline_save_backside').map(m=>m.text)") == ['Edited prompt','Edited output']
        page.locator('#move-down').click(); page.locator('#save-order').click()
        expect(page.locator('#notice')).to_contain_text('Order saved')
        assert page.evaluate("messages.filter(m=>m.type==='pipeline_reorder').at(-1).order") == ['LADDER','SYNOPSIS']
        page.locator('#order-field').select_option('page_order')
        expect(page.locator('#calls button')).to_have_count(3)
        page.get_by_role('button', name='3. ORIGINAL · ORIGINAL',exact=True).click()
        expect(page.locator('#call-prompt')).to_be_disabled()
        page.locator('#position').select_option('0'); page.locator('#save-order').click()
        assert page.evaluate("messages.filter(m=>m.type==='pipeline_reorder').at(-1).field") == 'page_order'
        assert page.evaluate("messages.filter(m=>m.type==='pipeline_reorder').at(-1).order") == ['ORIGINAL','SYNOPSIS','LADDER']
        # Verify responsive layout without a server or network requests.
        page.set_viewport_size({'width':600,'height':900})
        assert page.evaluate('document.documentElement.scrollWidth <= innerWidth')
        assert not errors, errors
        page.close(run_before_unload=False)
        # Outside Nerve the page must explain the missing IPC, with no fake inventory.
        bare=browser.new_page(); bare.goto(PANEL.as_uri())
        expect(bare.locator('#notice')).to_contain_text('Open this panel inside Nerve')
        expect(bare.locator('#workspace')).to_be_hidden()
        browser.close()
        print('PASS: inventory scope, Stories family, settings, stdin workers, streaming across stations, repair button, results, paired editing, independent order controls, responsive layout, and missing IPC.')

if __name__ == '__main__':
    run()
