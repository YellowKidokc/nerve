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
 *   atoms:all          — JSON object map of canon/candidate atoms (keyed by canonical id)
 *   axioms:all         — JSON object map of axiom candidate packets (keyed by canonical id)
 *
 * API (atoms & axioms):
 *   GET  /api/atoms                 — list atoms (?q= ?status= ?limit=)
 *   GET  /api/atoms/:id             — one atom
 *   POST /api/atoms/batch           — intake batch { atoms: [...] } or single packet
 *   GET  /api/axioms                — list axiom packets (?q= ?status= ?limit=)
 *   GET  /api/axioms/:id            — one axiom packet
 *   POST /api/axioms/batch          — intake batch { axioms: [...] } or { packet_type:"axiom-batch" }
 *
 * Intake rule (NO SILENT CANON WRITE): packets arrive as candidate_draft only.
 * Any packet claiming admitted status is rejected with 422.
 *
 * Auth: Bearer token in Authorization header, checked against NERVE_API_TOKEN secret.
 */

const DEFAULT_AI_MODEL = '@cf/deepseek-ai/deepseek-r1-distill-qwen-32b';
const STRUCTURED_AI_MODEL = '@cf/meta/llama-3.3-70b-instruct-fp8-fast';
const MAX_PROMPT_CHARS = 60000;

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
      if (path === '/api/ai' && request.method === 'POST') {
        ensureAiBinding(env);
        const body = await request.json();
        const prompt = String(body.prompt || '').trim();
        const systemPrompt = String(body.system_prompt || 'You are an expert epistemologist and formal ontologist.').trim();
        if (!prompt) return corsJson({ error: 'prompt is required' }, 400);
        if (prompt.length > MAX_PROMPT_CHARS) return corsJson({ error: 'prompt is too large' }, 413);
        const model = chooseAiModel(systemPrompt);
        const result = await runDeepSeek(env, prompt, systemPrompt);
        return corsJson({ ok: true, provider: 'cloudflare-workers-ai', model, response: result, generated_at: new Date().toISOString() });
      }

      if (path === '/api/harvest' && request.method === 'POST') {
        ensureAiBinding(env);
        const body = await request.json();
        const sourceText = String(body.text || '').trim();
        const maxPredicates = clampNumber(body.max_predicates, 1, 40, 20);
        if (!sourceText) return corsJson({ error: 'text is required' }, 400);
        if (sourceText.length > MAX_PROMPT_CHARS) return corsJson({ error: 'text is too large' }, 413);
        const prompt = [
          `Extract at most ${maxPredicates} independently reviewable truth predicates from the source below.`,
          'Return only a JSON array of concise strings. Do not include markdown, commentary, or a code fence.',
          'Do not merge claims that could stand or fail independently.',
          '',
          sourceText,
        ].join('\n');
        const raw = await runDeepSeek(env, prompt, 'You decompose source material into atomic, faithful, non-duplicative claims.');
        const predicates = parsePredicateArray(raw).slice(0, maxPredicates);
        if (!predicates.length) return corsJson({ error: 'DeepSeek returned no parseable predicates', raw_response: raw }, 502);
        return corsJson({ ok: true, provider: 'cloudflare-workers-ai', model: DEFAULT_AI_MODEL, predicates, generated_at: new Date().toISOString() });
      }
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
          ai: Boolean(env.AI),
          ai_model: DEFAULT_AI_MODEL,
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

function ensureAiBinding(env) {
  if (!env.AI || typeof env.AI.run !== 'function') throw new Error('Cloudflare Workers AI binding is not configured');
}

async function runDeepSeek(env, prompt, systemPrompt) {
  const model = chooseAiModel(systemPrompt);
  const options = {
    messages: [
      { role: 'system', content: systemPrompt },
      { role: 'user', content: prompt },
    ],
    max_tokens: 8192,
    temperature: 0.1,
  };
  if (model === STRUCTURED_AI_MODEL) options.response_format = { type: 'json_object' };
  const result = await env.AI.run(model, options);
  const structuredResult = model === STRUCTURED_AI_MODEL && result && typeof result === 'object'
    ? (result.response ?? result)
    : null;
  const response = typeof result === 'string'
    ? result
    : typeof result?.response === 'string'
      ? result.response
      : structuredResult
        ? JSON.stringify(structuredResult)
        : '';
  if (!response) throw new Error('AI model returned an empty response');
  return response.replace(/<think>[\s\S]*?<\/think>/gi, '').trim();
}

function chooseAiModel(systemPrompt) {
  return String(systemPrompt || '').includes('neutral epistemic intake instrument')
    ? STRUCTURED_AI_MODEL
    : DEFAULT_AI_MODEL;
}

function parsePredicateArray(raw) {
  const cleaned = String(raw || '').replace(/<think>[\s\S]*?<\/think>/gi, '').trim();
  const start = cleaned.indexOf('[');
  const end = cleaned.lastIndexOf(']');
  if (start === -1 || end <= start) return [];
  try {
    const parsed = JSON.parse(cleaned.slice(start, end + 1));
    return Array.isArray(parsed) ? parsed.map(value => String(value).trim()).filter(Boolean) : [];
  } catch {
    return [];
  }
}

function clampNumber(value, min, max, fallback) {
  const number = Number.parseInt(value, 10);
  return Number.isFinite(number) ? Math.min(max, Math.max(min, number)) : fallback;
}

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
