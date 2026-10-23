# YouTube Watch → Transcript Grabber

Chrome extension + local Python server. Every YouTube video you open is logged;
a popup asks "Download this video?"; clicking **Download** runs YTBSD's
`ytgrab.py` for that URL — no menu, no pasting.

Proxy order (handled by ytgrab.py): **direct → Webshare rotating residential → 300 free proxies**.
Webshare uses the `WEBSHARE_USER` / `WEBSHARE_PASS` saved by `SET_WEBSHARE_CREDENTIALS.bat`.

Transcripts land in `D:\GitHub\Research-Acquisition\yt-bulk-subtitles-downloader\subtitles\`.

## Setup
1. Double-click `start_server.bat` (leave the window open).
2. Chrome → `chrome://extensions` → **Developer mode** → **Load unpacked** → pick the `extension` folder.
3. Open any YouTube video — the prompt appears bottom-right.

## Download everything at once
http://127.0.0.1:8765/ → **Download all not-yet-downloaded**, or tick boxes → **Download selected**.
Queued videos are sent to ytgrab.py together (up to `batch_size` per run).

## config.json
| key | meaning |
|---|---|
| `mode` | `transcript` (YTBSD) or `video` (full yt-dlp video download) |
| `ytbsd_dir` | where YTBSD lives |
| `ytbsd_args` | extra ytgrab flags; drop `--free-proxies` to use only direct + Webshare |
| `batch_size` | URLs per ytgrab run |
| `download_dir`, `format`, `proxies` | used only in `video` mode |

History lives in `watch_log.db` (SQLite).
