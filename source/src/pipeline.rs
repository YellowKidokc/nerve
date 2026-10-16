//! Station Console: drive pipeline-workflows stations from Nerve (Ctrl+Alt+W).
//!
//! Each configured root is a pipeline folder (one per drive) that carries
//! `_system/engine/console.py`. Nerve never reads the station folders itself:
//! `console.py list` prints what exists, and a run is the station's own `.bat`
//! button, started exactly as a double-click would start it. The button's
//! questions arrive as output and the panel answers them through stdin, so the
//! pipeline logic stays in the pipeline folder.

use crate::config::PipelineConfig;
use anyhow::{anyhow, bail, Context, Result};
use std::collections::HashMap;
use std::io::{Read, Write};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use tracing::{info, warn};

const CREATE_NO_WINDOW: u32 = 0x0800_0000;
/// Files the console may read; only FOCUS.md may be written.
const READABLE: [&str; 3] = ["FOCUS.md", "PROMPT.md", "README.md"];
const LEDGER: &str = "console-ledger.jsonl";

/// Sends one script to the console panel.
pub type Emit = Arc<dyn Fn(String) + Send + Sync>;

struct Running {
    child: Child,
    stdin: Option<ChildStdin>,
    output: Arc<Mutex<String>>,
}

fn runs() -> &'static Mutex<HashMap<String, Running>> {
    static RUNS: OnceLock<Mutex<HashMap<String, Running>>> = OnceLock::new();
    RUNS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn js(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".into())
}

/// Every station in every configured root, as `console.py list` reports it.
pub fn list(cfg: &PipelineConfig) -> serde_json::Value {
    let roots: Vec<serde_json::Value> = cfg
        .roots
        .iter()
        .map(|root| match list_root(cfg, root) {
            Ok(data) => serde_json::json!({ "root": root, "ok": true, "data": data }),
            Err(e) => serde_json::json!({ "root": root, "ok": false, "error": format!("{:#}", e) }),
        })
        .collect();
    serde_json::json!({ "roots": roots })
}

fn list_root(cfg: &PipelineConfig, root: &str) -> Result<serde_json::Value> {
    let script = Path::new(root).join("_system").join("engine").join("console.py");
    if !script.is_file() {
        bail!("no console.py at {}", script.display());
    }
    let mut cmd = Command::new(&cfg.python_exe);
    cmd.arg(&script).arg("list").current_dir(root).env("PYTHONIOENCODING", "utf-8");
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    let out = cmd
        .output()
        .with_context(|| format!("starting {}", cfg.python_exe))?;
    if !out.status.success() {
        bail!("console.py failed: {}", String::from_utf8_lossy(&out.stderr).trim());
    }
    serde_json::from_slice(&out.stdout).context("console.py printed something that is not JSON")
}

/// `path` resolved, and only if it lies inside one of the configured roots.
fn inside_roots(cfg: &PipelineConfig, path: &str) -> Result<(PathBuf, PathBuf)> {
    let target = std::fs::canonicalize(path).with_context(|| format!("not found: {}", path))?;
    for root in &cfg.roots {
        if let Ok(r) = std::fs::canonicalize(root) {
            if target.starts_with(&r) {
                return Ok((target, r));
            }
        }
    }
    bail!("{} is outside the pipeline roots in settings", path)
}

/// Start one station button. Output and the exit code reach the panel as
/// `stationOutput(run_id, text)` and `stationExit(run_id, code)`.
pub fn start(cfg: &PipelineConfig, button: &str, run_id: &str, emit: Emit) -> Result<()> {
    if !cfg.enabled {
        bail!("The Station Console is turned off in settings.");
    }
    let (bat, _) = inside_roots(cfg, button)?;
    if bat.extension().and_then(|e| e.to_str()).map(|e| e.eq_ignore_ascii_case("bat")) != Some(true) {
        bail!("only a station's .bat buttons can be run: {}", bat.display());
    }
    let front = bat.parent().ok_or_else(|| anyhow!("button has no folder"))?.to_path_buf();
    if runs().lock().unwrap().contains_key(run_id) {
        bail!("run {} is already going", run_id);
    }

    // Rust escapes .bat arguments itself when it starts a batch file directly.
    let mut cmd = Command::new(&bat);
    cmd.current_dir(&front)
        .env("PYTHONUNBUFFERED", "1")
        .env("PYTHONIOENCODING", "utf-8")
        .env("NERVE_CONSOLE", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    let mut child = cmd.spawn().with_context(|| format!("starting {}", bat.display()))?;
    info!("pipeline: run {} started {}", run_id, bat.display());

    let output = Arc::new(Mutex::new(String::new()));
    for stream in [
        child.stdout.take().map(|s| Box::new(s) as Box<dyn Read + Send>),
        child.stderr.take().map(|s| Box::new(s) as Box<dyn Read + Send>),
    ]
    .into_iter()
    .flatten()
    {
        let sink = Arc::clone(&output);
        std::thread::spawn(move || pump(stream, sink));
    }
    let stdin = child.stdin.take();
    runs()
        .lock()
        .unwrap()
        .insert(run_id.to_string(), Running { child, stdin, output });

    let id = run_id.to_string();
    std::thread::spawn(move || watch(id, emit));
    Ok(())
}

/// Copy a child's output into the shared buffer, keeping UTF-8 intact across reads.
fn pump(mut stream: Box<dyn Read + Send>, sink: Arc<Mutex<String>>) {
    let mut buf = [0u8; 4096];
    let mut carry: Vec<u8> = Vec::new();
    loop {
        let n = match stream.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => n,
        };
        carry.extend_from_slice(&buf[..n]);
        let valid = match std::str::from_utf8(&carry) {
            Ok(_) => carry.len(),
            Err(e) if e.error_len().is_none() => e.valid_up_to(), // a character split across reads
            Err(_) => carry.len(),                                  // not UTF-8 (console code page): take it lossily
        };
        let text = String::from_utf8_lossy(&carry[..valid]).into_owned();
        carry.drain(..valid);
        sink.lock().unwrap().push_str(&text);
    }
    if !carry.is_empty() {
        sink.lock().unwrap().push_str(&String::from_utf8_lossy(&carry));
    }
}

/// Every 150 ms: hand new output to the panel; when the child is gone, report its exit.
fn watch(run_id: String, emit: Emit) {
    loop {
        std::thread::sleep(Duration::from_millis(150));
        let (text, status) = {
            let mut map = runs().lock().unwrap();
            let Some(run) = map.get_mut(&run_id) else { return };
            let text = std::mem::take(&mut *run.output.lock().unwrap());
            (text, run.child.try_wait().ok().flatten())
        };
        if !text.is_empty() {
            emit(format!("stationOutput({}, {});", js(&run_id), js(&text)));
        }
        if let Some(status) = status {
            // Let the readers drain what the child wrote last.
            std::thread::sleep(Duration::from_millis(200));
            let rest = runs()
                .lock()
                .unwrap()
                .remove(&run_id)
                .map(|r| std::mem::take(&mut *r.output.lock().unwrap()))
                .unwrap_or_default();
            if !rest.is_empty() {
                emit(format!("stationOutput({}, {});", js(&run_id), js(&rest)));
            }
            let code = status.code().unwrap_or(-1);
            info!("pipeline: run {} ended with {}", run_id, code);
            emit(format!("stationExit({}, {});", js(&run_id), code));
            return;
        }
    }
}

/// Answer the question a running button is asking (one line, Enter included).
pub fn answer(run_id: &str, text: &str) -> Result<()> {
    let mut map = runs().lock().unwrap();
    let run = map.get_mut(run_id).ok_or_else(|| anyhow!("run {} is not going", run_id))?;
    let stdin = run.stdin.as_mut().ok_or_else(|| anyhow!("run {} takes no input", run_id))?;
    stdin.write_all(format!("{}\r\n", text).as_bytes())?;
    stdin.flush()?;
    Ok(())
}

/// Stop a run and everything it started (the .bat's Python children too).
pub fn stop(run_id: &str) -> Result<()> {
    let pid = runs()
        .lock()
        .unwrap()
        .get(run_id)
        .map(|r| r.child.id())
        .ok_or_else(|| anyhow!("run {} is not going", run_id))?;
    let mut cmd = Command::new("taskkill");
    cmd.args(["/T", "/F", "/PID", &pid.to_string()]);
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    let out = cmd.output()?;
    if !out.status.success() {
        warn!("pipeline: taskkill {} said {}", pid, String::from_utf8_lossy(&out.stderr).trim());
    }
    Ok(())
}

fn named(path: &Path) -> Result<&str> {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    READABLE
        .iter()
        .find(|r| r.eq_ignore_ascii_case(name))
        .copied()
        .ok_or_else(|| anyhow!("the console only opens FOCUS.md, PROMPT.md and README.md"))
}

pub fn read_text(cfg: &PipelineConfig, path: &str) -> Result<String> {
    let (file, _) = inside_roots(cfg, path)?;
    named(&file)?;
    Ok(std::fs::read_to_string(&file)?)
}

/// FOCUS.md is the one file the console writes; the old text goes to `_older\`.
pub fn save_focus(cfg: &PipelineConfig, path: &str, text: &str) -> Result<()> {
    let (file, _) = inside_roots(cfg, path)?;
    if named(&file)? != "FOCUS.md" {
        bail!("only FOCUS.md can be saved from the console");
    }
    let old = std::fs::read_to_string(&file).unwrap_or_default();
    if old != text {
        let stamp = chrono_stamp();
        let keep = file.parent().unwrap().join("_older").join(&stamp);
        std::fs::create_dir_all(&keep)?;
        std::fs::write(keep.join("FOCUS.md"), old)?;
        std::fs::write(&file, text)?;
    }
    Ok(())
}

fn chrono_stamp() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("console-{}", secs)
}

/// The run ledger lives with the pipeline: `<root>\_system\LOGS\console-ledger.jsonl`.
fn ledger_path(root: &Path) -> PathBuf {
    root.join("_system").join("LOGS").join(LEDGER)
}

pub fn ledger_append(cfg: &PipelineConfig, root: &str, entry: &serde_json::Value) -> Result<()> {
    let (root, _) = inside_roots(cfg, root)?;
    let path = ledger_path(&root);
    std::fs::create_dir_all(path.parent().unwrap())?;
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&path)?;
    writeln!(f, "{}", serde_json::to_string(entry)?)?;
    Ok(())
}

/// The last `limit` ledger lines of a root, oldest first.
pub fn ledger_read(cfg: &PipelineConfig, root: &str, limit: usize) -> Result<Vec<serde_json::Value>> {
    let (root, _) = inside_roots(cfg, root)?;
    let raw = std::fs::read_to_string(ledger_path(&root)).unwrap_or_default();
    let lines: Vec<&str> = raw.lines().filter(|l| !l.trim().is_empty()).collect();
    let start = lines.len().saturating_sub(limit);
    Ok(lines[start..].iter().filter_map(|l| serde_json::from_str(l).ok()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A button that asks a question, echoes the answer, then exits 3: the
    /// console must show the question before the answer exists, pass the
    /// answer through, and report the exit code.
    #[test]
    fn button_asks_answers_and_exits() {
        let root = std::env::temp_dir().join(format!("nerve-console-test-{}", std::process::id()));
        let front = root.join("001_TEST");
        std::fs::create_dir_all(&front).unwrap();
        let bat = front.join("1 RUN HERE.bat");
        std::fs::write(
            &bat,
            "@echo off\r\necho hello from the station\r\nset /p NAME=How many? \r\necho got [%NAME%]\r\nexit /b 3\r\n",
        )
        .unwrap();
        let cfg = PipelineConfig {
            enabled: true,
            roots: vec![root.to_string_lossy().into()],
            python_exe: "python".into(),
        };
        let seen = Arc::new(Mutex::new(Vec::<String>::new()));
        let sink = Arc::clone(&seen);
        let emit: Emit = Arc::new(move |s| sink.lock().unwrap().push(s));

        start(&cfg, &bat.to_string_lossy(), "T1", emit).unwrap();
        let wait_for = |needle: &str| {
            for _ in 0..100 {
                if seen.lock().unwrap().iter().any(|s| s.contains(needle)) {
                    return true;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            false
        };
        assert!(wait_for("How many?"), "question never shown: {:?}", seen.lock().unwrap());
        answer("T1", "7").unwrap();
        assert!(wait_for("got [7]"), "answer not passed: {:?}", seen.lock().unwrap());
        assert!(wait_for("stationExit(\"T1\", 3)"), "exit not reported: {:?}", seen.lock().unwrap());

        // Outside the roots, and non-.bat files, are refused.
        assert!(start(&cfg, r"C:\Windows\System32\cmd.exe", "T2", Arc::new(|_| {})).is_err());
        std::fs::write(front.join("FOCUS.md"), "# old").unwrap();
        save_focus(&cfg, &front.join("FOCUS.md").to_string_lossy(), "# new").unwrap();
        assert_eq!(read_text(&cfg, &front.join("FOCUS.md").to_string_lossy()).unwrap(), "# new");
        assert!(save_focus(&cfg, &bat.to_string_lossy(), "x").is_err());
        let _ = std::fs::remove_dir_all(&root);
    }
}

/// Show a station folder (or its INBOX / OUTBOX) in Explorer.
pub fn open_folder(cfg: &PipelineConfig, path: &str) -> Result<()> {
    let (folder, _) = inside_roots(cfg, path)?;
    if !folder.is_dir() {
        bail!("not a folder: {}", folder.display());
    }
    Command::new("explorer").arg(&folder).spawn()?;
    Ok(())
}
