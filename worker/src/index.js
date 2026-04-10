/**
 * Nerve Sync API — Cloudflare Worker
 *
 * KV keys:
 *   config:hotkeys     — JSON array of hotkey bindings
 *   config:hotstrings  — JSON array of hotstrings
 *   config:prompts     — JSON array of prompts
 *   config:tts         — JSON object of TTS settings
 *   clipboard:history  — JSON array of recent clips (newest first)
 *   clipboard:slots    — JSON array of clip slots
 *
 * Auth: Bearer token in Authorization header, checked against NERVE_API_TOKEN secret.
 */

export default {
  async fetch(request, env) {
    // CORS preflight
    if (request.method === 'OPTIONS') {
      return corsResponse(new Response(null, { status: 204 }));
    }

    const url = new URL(request.url);
    const path = url.pathname;

    // Auth check
    const authHeader = request.headers.get('Authorization') || '';
    const token = authHeader.replace('Bearer ', '').trim();
    const expectedToken = env.NERVE_API_TOKEN || '';

    if (!expectedToken) {
      // No token configured — allow all (dev mode)
    } else if (token !== expectedToken) {
      return corsResponse(new Response(JSON.stringify({ error: 'unauthorized' }), {
        status: 401,
        headers: { 'Content-Type': 'application/json' },
      }));
    }

    try {
      // ── CLIPBOARD ──

      // POST /clipboard/push — add a clip to history
      if (path === '/clipboard/push' && request.method === 'POST') {
        const body = await request.json();
        const content = body.content || '';
        const source = body.source || 'unknown';

        if (!content) {
          return corsJson({ error: 'empty content' }, 400);
        }

        // Get existing history
        let history = await kvGetJson(env.NERVE_KV, 'clipboard:history') || [];

        // Prepend new entry, dedup, cap at 200
        history = history.filter(c => c.content !== content);
        history.unshift({
          content,
          source,
          timestamp: new Date().toISOString(),
        });
        if (history.length > 200) history = history.slice(0, 200);

        await env.NERVE_KV.put('clipboard:history', JSON.stringify(history));
        return corsJson({ ok: true, count: history.length });
      }

      // GET /clipboard/history — retrieve clip history
      if (path === '/clipboard/history' && request.method === 'GET') {
        const limit = parseInt(url.searchParams.get('limit') || '40', 10);
        const history = await kvGetJson(env.NERVE_KV, 'clipboard:history') || [];
        return corsJson(history.slice(0, limit));
      }

      // ── CONFIG ENDPOINTS ──

      // GET /config/hotkeys
      if (path === '/config/hotkeys' && request.method === 'GET') {
        const data = await kvGetJson(env.NERVE_KV, 'config:hotkeys') || [];
        return corsJson(data);
      }

      // GET /config/hotstrings
      if (path === '/config/hotstrings' && request.method === 'GET') {
        const data = await kvGetJson(env.NERVE_KV, 'config:hotstrings') || [];
        return corsJson(data);
      }

      // GET /config/prompts
      if (path === '/config/prompts' && request.method === 'GET') {
        const data = await kvGetJson(env.NERVE_KV, 'config:prompts') || [];
        return corsJson(data);
      }

      // GET /config/tts
      if (path === '/config/tts' && request.method === 'GET') {
        const data = await kvGetJson(env.NERVE_KV, 'config:tts') || {};
        return corsJson(data);
      }

      // GET /config/all — full config dump for PWA
      if (path === '/config/all' && request.method === 'GET') {
        const [hotkeys, hotstrings, prompts, tts, clipSlots, clipHistory] = await Promise.all([
          kvGetJson(env.NERVE_KV, 'config:hotkeys'),
          kvGetJson(env.NERVE_KV, 'config:hotstrings'),
          kvGetJson(env.NERVE_KV, 'config:prompts'),
          kvGetJson(env.NERVE_KV, 'config:tts'),
          kvGetJson(env.NERVE_KV, 'clipboard:slots'),
          kvGetJson(env.NERVE_KV, 'clipboard:history'),
        ]);
        return corsJson({
          hotkeys: hotkeys || [],
          hotstrings: hotstrings || [],
          prompts: prompts || [],
          tts: tts || {},
          clip_slots: clipSlots || [],
          clipboard_history: (clipHistory || []).slice(0, 40),
        });
      }

      // PUT /config/sync — full config push from desktop
      if (path === '/config/sync' && request.method === 'PUT') {
        const body = await request.json();

        const writes = [];
        if (body.hotkeys) writes.push(env.NERVE_KV.put('config:hotkeys', JSON.stringify(body.hotkeys)));
        if (body.hotstrings) writes.push(env.NERVE_KV.put('config:hotstrings', JSON.stringify(body.hotstrings)));
        if (body.prompts) writes.push(env.NERVE_KV.put('config:prompts', JSON.stringify(body.prompts)));
        if (body.tts) writes.push(env.NERVE_KV.put('config:tts', JSON.stringify(body.tts)));
        if (body.clip_slots) writes.push(env.NERVE_KV.put('clipboard:slots', JSON.stringify(body.clip_slots)));

        await Promise.all(writes);
        return corsJson({ ok: true, synced: writes.length });
      }

      // PUT /config/hotkeys — update hotkeys
      if (path === '/config/hotkeys' && request.method === 'PUT') {
        const body = await request.json();
        await env.NERVE_KV.put('config:hotkeys', JSON.stringify(body));
        return corsJson({ ok: true });
      }

      // PUT /config/hotstrings — update hotstrings
      if (path === '/config/hotstrings' && request.method === 'PUT') {
        const body = await request.json();
        await env.NERVE_KV.put('config:hotstrings', JSON.stringify(body));
        return corsJson({ ok: true });
      }

      // PUT /config/prompts — update prompts
      if (path === '/config/prompts' && request.method === 'PUT') {
        const body = await request.json();
        await env.NERVE_KV.put('config:prompts', JSON.stringify(body));
        return corsJson({ ok: true });
      }

      // PUT /config/tts — update TTS settings
      if (path === '/config/tts' && request.method === 'PUT') {
        const body = await request.json();
        await env.NERVE_KV.put('config:tts', JSON.stringify(body));
        return corsJson({ ok: true });
      }

      // ── HEALTH ──
      if (path === '/' || path === '/health') {
        return corsJson({
          service: 'nerve-sync-api',
          status: 'ok',
          timestamp: new Date().toISOString(),
        });
      }

      return corsJson({ error: 'not found', path }, 404);

    } catch (err) {
      return corsJson({ error: err.message }, 500);
    }
  },
};

// ── Helpers ──

async function kvGetJson(kv, key) {
  const raw = await kv.get(key);
  if (!raw) return null;
  try { return JSON.parse(raw); }
  catch { return null; }
}

function corsJson(data, status = 200) {
  return corsResponse(new Response(JSON.stringify(data), {
    status,
    headers: { 'Content-Type': 'application/json' },
  }));
}

function corsResponse(response) {
  const headers = new Headers(response.headers);
  headers.set('Access-Control-Allow-Origin', '*');
  headers.set('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
  headers.set('Access-Control-Allow-Headers', 'Content-Type, Authorization');
  headers.set('Access-Control-Max-Age', '86400');
  return new Response(response.body, {
    status: response.status,
    headers,
  });
}
