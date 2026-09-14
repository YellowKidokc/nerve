"""Loopback-only proxy. Cloud credential stays in this process, never in HTML."""
import json
import os
import urllib.error
import urllib.request
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

ORIGINS = {'http://127.0.0.1:8768', 'http://localhost:8768', 'http://nerve.localhost', 'https://nerve.localhost', 'nerve://localhost'}
BASE = os.environ.get('NERVE_ATOM_URL', '').rstrip('/')
TOKEN = os.environ.get('NERVE_ATOM_TOKEN', '')

class Handler(BaseHTTPRequestHandler):
    def log_message(self, *_):
        pass

    def reply(self, code, body):
        self.send_response(code)
        self.send_header('Content-Type', 'application/json')
        origin = self.headers.get('Origin', '')
        if origin in ORIGINS:
            self.send_header('Access-Control-Allow-Origin', origin)
            self.send_header('Vary', 'Origin')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.end_headers()
        self.wfile.write(body if isinstance(body, bytes) else json.dumps(body).encode())

    def allowed(self):
        return self.headers.get('Host') in {'127.0.0.1:7861', 'localhost:7861'} and self.headers.get('Origin', '') in ORIGINS | {''}

    def do_OPTIONS(self):
        self.reply(204 if self.allowed() else 403, b'')

    def do_GET(self):
        self.proxy()

    def do_POST(self):
        self.proxy()

    def proxy(self):
        if not self.allowed():
            return self.reply(403, {'error': 'Origin or host is not permitted'})
        if not BASE.startswith('https://') or not TOKEN:
            return self.reply(503, {'error': 'Cloud connection is not configured'})
        path = self.path
        if path == '/health' and self.command == 'GET':
            remote = '/health'
        elif path.startswith('/jobs/') and len(path) == 70 and all(c in '0123456789abcdef' for c in path[6:]) and self.command == 'GET':
            remote = path
        elif path == '/jobs' and self.command == 'POST':
            remote = path
        elif path in ['/api/atom-builder/' + x for x in ('builder', 'audit', 'regenerate')] and self.command == 'POST':
            remote = '/jobs'
        else:
            return self.reply(404, {'error': 'Not found'})
        try:
            data = None
            if self.command == 'POST':
                length = int(self.headers.get('Content-Length', '0'))
                if length <= 0 or length > 60000:
                    return self.reply(413, {'error': 'Job must be between 1 and 60000 bytes'})
                body = json.loads(self.rfile.read(length))
                if path.startswith('/api/atom-builder/'):
                    body = {'pass': path.rsplit('/', 1)[1], 'request': body}
                data = json.dumps(body, ensure_ascii=False).encode('utf-8')
            req = urllib.request.Request(BASE + remote, data=data, method=self.command, headers={'Authorization': 'Bearer ' + TOKEN, 'Content-Type': 'application/json', 'User-Agent': 'Nerve-Atom-Bridge/1'})
            with urllib.request.urlopen(req, timeout=40) as response:
                self.reply(response.status, response.read())
        except urllib.error.HTTPError as error:
            self.reply(error.code, error.read())
        except (ValueError, json.JSONDecodeError):
            self.reply(400, {'error': 'Invalid JSON request'})
        except Exception:
            self.reply(502, {'error': 'Cloud connection failed; submitted jobs remain in Cloudflare'})

if __name__ == '__main__':
    ThreadingHTTPServer(('127.0.0.1', 7861), Handler).serve_forever()
