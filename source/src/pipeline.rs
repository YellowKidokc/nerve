//! Station Console: drive pipeline-workflows stations from Nerve (Ctrl+Alt+W).
//!
//! Each configured root is a pipeline folder (one per drive) that carries
//! `_system/engine/console.py`. `console.py list` supplies station inventory;
//! Nerve reads OUTBOX results and narrowly allowlisted Backside files. A run
//! is the station's own `.bat`
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
/// Legacy station text files; Backside uses a separate, narrow allowlist.
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
    let script = Path::new(root)
        .join("_system")
        .join("engine")
        .join("console.py");
    if !script.is_file() {
        bail!("no console.py at {}", script.display());
    }
    let mut cmd = Command::new(&cfg.python_exe);
    cmd.arg(&script)
        .arg("list")
        .current_dir(root)
        .env("PYTHONIOENCODING", "utf-8");
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    let out = cmd
        .output()
        .with_context(|| format!("starting {}", cfg.python_exe))?;
    if !out.status.success() {
        bail!(
            "console.py failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
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
    if bat
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("bat"))
        != Some(true)
    {
        bail!(
            "only a station's .bat buttons can be run: {}",
            bat.display()
        );
    }
    let front = bat
        .parent()
        .ok_or_else(|| anyhow!("button has no folder"))?
        .to_path_buf();
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
    let mut child = cmd
        .spawn()
        .with_context(|| format!("starting {}", bat.display()))?;
    info!("pipeline: run {} started {}", run_id, bat.display());

    let output = Arc::new(Mutex::new(String::new()));
    for stream in [
        child
            .stdout
            .take()
            .map(|s| Box::new(s) as Box<dyn Read + Send>),
        child
            .stderr
            .take()
            .map(|s| Box::new(s) as Box<dyn Read + Send>),
    ]
    .into_iter()
    .flatten()
    {
        let sink = Arc::clone(&output);
        std::thread::spawn(move || pump(stream, sink));
    }
    let stdin = child.stdin.take();
    runs().lock().unwrap().insert(
        run_id.to_string(),
        Running {
            child,
            stdin,
            output,
        },
    );

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
            Err(_) => carry.len(), // not UTF-8 (console code page): take it lossily
        };
        let text = String::from_utf8_lossy(&carry[..valid]).into_owned();
        carry.drain(..valid);
        sink.lock().unwrap().push_str(&text);
    }
    if !carry.is_empty() {
        sink.lock()
            .unwrap()
            .push_str(&String::from_utf8_lossy(&carry));
    }
}

/// Every 150 ms: hand new output to the panel; when the child is gone, report its exit.
fn watch(run_id: String, emit: Emit) {
    loop {
        std::thread::sleep(Duration::from_millis(150));
        let (text, status) = {
            let mut map = runs().lock().unwrap();
            let Some(run) = map.get_mut(&run_id) else {
                return;
            };
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
    let run = map
        .get_mut(run_id)
        .ok_or_else(|| anyhow!("run {} is not going", run_id))?;
    let stdin = run
        .stdin
        .as_mut()
        .ok_or_else(|| anyhow!("run {} takes no input", run_id))?;
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
        warn!(
            "pipeline: taskkill {} said {}",
            pid,
            String::from_utf8_lossy(&out.stderr).trim()
        );
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

/// Save standing focus, retaining a timestamped sibling backup before changes.
pub fn save_focus(cfg: &PipelineConfig, path: &str, text: &str) -> Result<()> {
    let (file, _) = inside_roots(cfg, path)?;
    if named(&file)? != "FOCUS.md" {
        bail!("only FOCUS.md can be saved from the console");
    }
    backup_write(&file, text)
}

/// The run ledger lives with the pipeline: `<root>\_system\LOGS\console-ledger.jsonl`.
fn ledger_path(root: &Path) -> PathBuf {
    root.join("_system").join("LOGS").join(LEDGER)
}

pub fn ledger_append(cfg: &PipelineConfig, root: &str, entry: &serde_json::Value) -> Result<()> {
    let (root, _) = inside_roots(cfg, root)?;
    let path = ledger_path(&root);
    std::fs::create_dir_all(path.parent().unwrap())?;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    writeln!(f, "{}", serde_json::to_string(entry)?)?;
    Ok(())
}

/// The last `limit` ledger lines of a root, oldest first.
pub fn ledger_read(
    cfg: &PipelineConfig,
    root: &str,
    limit: usize,
) -> Result<Vec<serde_json::Value>> {
    let (root, _) = inside_roots(cfg, root)?;
    let raw = std::fs::read_to_string(ledger_path(&root)).unwrap_or_default();
    let lines: Vec<&str> = raw.lines().filter(|l| !l.trim().is_empty()).collect();
    let start = lines.len().saturating_sub(limit);
    Ok(lines[start..]
        .iter()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect())
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    /// A button that asks a question, echoes the answer, then exits 3: the
    /// console must show the question before the answer exists, pass the
    /// answer through, and report the exit code.
    #[test]
    #[cfg(windows)]
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
        assert!(
            wait_for("How many?"),
            "question never shown: {:?}",
            seen.lock().unwrap()
        );
        answer("T1", "7").unwrap();
        assert!(
            wait_for("got [7]"),
            "answer not passed: {:?}",
            seen.lock().unwrap()
        );
        assert!(
            wait_for("stationExit(\"T1\", 3)"),
            "exit not reported: {:?}",
            seen.lock().unwrap()
        );

        // Outside the roots, and non-.bat files, are refused.
        assert!(start(&cfg, r"C:\Windows\System32\cmd.exe", "T2", Arc::new(|_| {})).is_err());
        std::fs::write(front.join("FOCUS.md"), "# old").unwrap();
        save_focus(&cfg, &front.join("FOCUS.md").to_string_lossy(), "# new").unwrap();
        assert_eq!(
            read_text(&cfg, &front.join("FOCUS.md").to_string_lossy()).unwrap(),
            "# new"
        );
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

// The first panel release deliberately exposes only these four station fronts.
const PANEL_STATIONS: [&str; 4] = [
    "01_CKG/020_CKG",
    "01_CKG/025_CKG_V4",
    "041_STORIES",
    "041_STORIES/OUTBOX/024_CRAFT_PASS",
];

fn station_front(cfg: &PipelineConfig, front: &str) -> Result<PathBuf> {
    let (front, root) = inside_roots(cfg, front)?;
    if !PANEL_STATIONS
        .iter()
        .any(|p| root.join(p).canonicalize().ok().as_ref() == Some(&front))
    {
        bail!("this panel supports only CKG and Stories stations");
    }
    Ok(front)
}

fn backside_file(cfg: &PipelineConfig, path: &str) -> Result<PathBuf> {
    let (file, _) = inside_roots(cfg, path)?;
    // Check both the requested layout and the resolved layout: symlinks cannot widen the allowlist.
    let requested = Path::new(path);
    let back = if requested.file_name().and_then(|s| s.to_str()) == Some("ckg.json") {
        requested.parent()
    } else if matches!(
        requested.file_name().and_then(|s| s.to_str()),
        Some("prompt.md" | "output.md")
    ) {
        requested
            .parent()
            .and_then(Path::parent)
            .filter(|p| p.file_name().and_then(|s| s.to_str()) == Some("calls"))
            .and_then(Path::parent)
    } else {
        None
    }
    .ok_or_else(|| {
        anyhow!("only BACKSIDE/ckg.json and calls/*/prompt.md or output.md may be edited")
    })?;
    if back.file_name().and_then(|s| s.to_str()) != Some("BACKSIDE") {
        bail!("not a BACKSIDE file");
    }
    let front = station_front(
        cfg,
        &back
            .parent()
            .ok_or_else(|| anyhow!("missing station"))?
            .to_string_lossy(),
    )?;
    let relative = requested.strip_prefix(back)?;
    let expected = front.join("BACKSIDE").join(relative);
    if file != expected || !file.is_file() {
        bail!("linked or invalid Backside file");
    }
    Ok(file)
}

/// Keep a unique, timestamped sibling copy before changing any editable file.
fn backup_write(file: &Path, text: &str) -> Result<()> {
    let old = std::fs::read(file)?;
    if old == text.as_bytes() {
        return Ok(());
    }
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos();
    let backup = file.with_file_name(format!(
        "{}.console-{}.bak",
        file.file_name().unwrap().to_string_lossy(),
        stamp
    ));
    let mut keep = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(backup)?;
    keep.write_all(&old)?;
    keep.sync_all()?;
    std::fs::write(file, text)?;
    Ok(())
}

pub fn backside(cfg: &PipelineConfig, front: &str) -> Result<serde_json::Value> {
    let front = station_front(cfg, front)?;
    let path = front.join("BACKSIDE/ckg.json");
    if !path.exists() {
        return Ok(serde_json::json!({"available": false}));
    }
    let file = backside_file(cfg, &path.to_string_lossy())?;
    let config: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&file)?)?;
    let mut calls = Vec::new();
    for call in config["calls"]
        .as_array()
        .ok_or_else(|| anyhow!("ckg.json has no calls array"))?
    {
        let folder = call["folder"]
            .as_str()
            .ok_or_else(|| anyhow!("call has no folder"))?;
        let parts: Vec<_> = Path::new(folder).components().collect();
        if parts.len() != 2
            || parts[0].as_os_str() != "calls"
            || !parts
                .iter()
                .all(|p| matches!(p, std::path::Component::Normal(_)))
        {
            bail!("invalid call folder");
        }
        let mut item = call.clone();
        for name in ["prompt.md", "output.md"] {
            let path = file.parent().unwrap().join(folder).join(name);
            // Paste-source entries may have no prompt/output editors.
            if path.exists() {
                let path = backside_file(cfg, &path.to_string_lossy())?;
                item[name] =
                    serde_json::json!({"path": path, "text": std::fs::read_to_string(path)?});
            }
        }
        calls.push(item);
    }
    Ok(serde_json::json!({"available": true, "path": file, "calls": calls}))
}

pub fn save_backside(cfg: &PipelineConfig, path: &str, text: &str) -> Result<()> {
    let file = backside_file(cfg, path)?;
    if file.file_name().and_then(|s| s.to_str()) == Some("ckg.json") {
        bail!("use the order controls to update ckg.json");
    }
    backup_write(&file, text)
}

/// Only order fields change; identities, dependencies, and all other settings survive.
pub fn reorder(cfg: &PipelineConfig, front: &str, order: &[String], field: &str) -> Result<()> {
    if !matches!(field, "run_order" | "page_order") {
        bail!("invalid order field");
    }
    let front = station_front(cfg, front)?;
    let file = backside_file(cfg, &front.join("BACKSIDE/ckg.json").to_string_lossy())?;
    let mut config: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&file)?)?;
    let calls = config["calls"]
        .as_array_mut()
        .ok_or_else(|| anyhow!("no calls"))?;
    let eligible: Vec<_> = calls
        .iter()
        .filter(|c| c.get(field).is_some_and(|v| !v.is_null()))
        .collect();
    let ids: std::collections::HashSet<_> = order.iter().collect();
    if ids.len() != order.len()
        || order.len() != eligible.len()
        || eligible
            .iter()
            .any(|c| !order.iter().any(|id| c["id"].as_str() == Some(id.as_str())))
    {
        bail!("order must contain every call ID exactly once");
    }
    for call in calls
        .iter_mut()
        .filter(|c| c.get(field).is_some_and(|v| !v.is_null()))
    {
        let pos = order
            .iter()
            .position(|id| call["id"].as_str() == Some(id.as_str()))
            .unwrap();
        call[field] = serde_json::json!(pos + 1);
    }
    backup_write(&file, &(serde_json::to_string_pretty(&config)? + "\n"))
}

fn result_files(cfg: &PipelineConfig, front: &str) -> Result<Vec<PathBuf>> {
    let front = station_front(cfg, front)?;
    let out = front.join("OUTBOX");
    if !out.exists() {
        return Ok(Vec::new());
    }
    let (out, _) = inside_roots(cfg, &out.to_string_lossy())?;
    if out != front.join("OUTBOX") {
        bail!("linked OUTBOX is not supported");
    }
    fn visit(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let ty = entry.file_type()?;
            if ty.is_symlink() {
                continue;
            }
            if ty.is_dir() {
                // Nested stations own their own Results view.
                if !path.join("BACKSIDE").is_dir()
                    && !matches!(
                        entry.file_name().to_str(),
                        Some("BACKSIDE" | "INBOX" | "_older")
                    )
                {
                    visit(&path, files)?;
                }
            } else if ty.is_file() {
                files.push(path);
            }
        }
        Ok(())
    }
    let mut files = Vec::new();
    visit(&out, &mut files)?;
    files.sort_by_key(|p| std::cmp::Reverse(p.metadata().and_then(|m| m.modified()).ok()));
    Ok(files)
}

pub fn results(cfg: &PipelineConfig, front: &str) -> Result<serde_json::Value> {
    let files = result_files(cfg, front)?;
    Ok(serde_json::json!({"files": files.iter().map(|p| {
        let m = p.metadata().ok();
        serde_json::json!({"path": p, "name": p.file_name().unwrap().to_string_lossy(), "bytes": m.as_ref().map(|m| m.len()), "modified": m.and_then(|m| m.modified().ok()).and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs())})
    }).collect::<Vec<_>>()}))
}

pub fn open_result(cfg: &PipelineConfig, front: &str, path: &str) -> Result<()> {
    let (file, _) = inside_roots(cfg, path)?;
    if !result_files(cfg, front)?.contains(&file) {
        bail!("not a result of this station");
    }
    let ext = file
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if ![
        "md", "txt", "json", "html", "htm", "pdf", "csv", "png", "jpg",
    ]
    .contains(&ext.as_str())
    {
        bail!("this result type cannot be opened from Nerve");
    }
    // Explorer opens the document via its registered association, with no shell interpolation.
    Command::new("explorer").arg(&file).spawn()?;
    Ok(())
}

#[cfg(test)]
mod panel_tests {
    use super::*;
    struct Fixture {
        root: PathBuf,
        front: PathBuf,
        cfg: PipelineConfig,
    }
    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "nerve-panel-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            let front = root.join("01_CKG/025_CKG_V4");
            let calls = front.join("BACKSIDE/calls/001_SYNOPSIS");
            std::fs::create_dir_all(&calls).unwrap();
            std::fs::write(calls.join("prompt.md"), "old prompt").unwrap();
            std::fs::write(calls.join("output.md"), "old output").unwrap();
            std::fs::write(front.join("BACKSIDE/FOCUS.md"), "old focus").unwrap();
            std::fs::write(front.join("BACKSIDE/ckg.json"), r#"{"model":"deepseek-chat","calls":[{"id":"SYNOPSIS","folder":"calls/001_SYNOPSIS","run_order":1,"page_order":1,"on":true,"run_group":1},{"id":"ORIGINAL","folder":"calls/099_ORIGINAL","page_order":99,"kind":"paste_source"}]}"#).unwrap();
            let cfg = PipelineConfig {
                enabled: true,
                roots: vec![root.to_string_lossy().into()],
                python_exe: "python3".into(),
            };
            Self { root, front, cfg }
        }
        fn path(&self, name: &str) -> String {
            self.front.join(name).to_string_lossy().into()
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }
    fn backups(file: &Path) -> Vec<PathBuf> {
        let prefix = format!("{}.console-", file.file_name().unwrap().to_string_lossy());
        std::fs::read_dir(file.parent().unwrap())
            .unwrap()
            .flatten()
            .filter(|e| e.file_name().to_string_lossy().starts_with(&prefix))
            .map(|e| e.path())
            .collect()
    }
    #[test]
    fn edits_backup_every_previous_version_and_preserve_unicode() {
        let f = Fixture::new();
        let p = f.path("BACKSIDE/calls/001_SYNOPSIS/prompt.md");
        assert_eq!(
            backside(&f.cfg, &f.front.to_string_lossy()).unwrap()["calls"][0]["prompt.md"]["text"],
            "old prompt"
        );
        save_backside(&f.cfg, &p, "new · prompt").unwrap();
        save_backside(&f.cfg, &p, "third").unwrap();
        save_backside(&f.cfg, &p, "third").unwrap();
        let copies = backups(Path::new(&p));
        assert_eq!(copies.len(), 2);
        let texts: Vec<_> = copies
            .iter()
            .map(|p| std::fs::read_to_string(p).unwrap())
            .collect();
        assert!(texts.contains(&"old prompt".into()) && texts.contains(&"new · prompt".into()));
        assert_eq!(std::fs::read_to_string(p).unwrap(), "third");
        let focus = f.path("BACKSIDE/FOCUS.md");
        save_focus(&f.cfg, &focus, "new focus").unwrap();
        assert_eq!(
            std::fs::read_to_string(&backups(Path::new(&focus))[0]).unwrap(),
            "old focus"
        );
    }
    #[test]
    fn refuses_scripts_arbitrary_json_other_groups_and_escape_paths() {
        let f = Fixture::new();
        for name in [
            "BACKSIDE/script.py",
            "BACKSIDE/other.json",
            "prompt.md",
            "BACKSIDE/calls/001_SYNOPSIS/no.md",
        ] {
            std::fs::write(f.front.join(name), "unchanged").unwrap();
            assert!(
                save_backside(&f.cfg, &f.path(name), "bad").is_err(),
                "{name}"
            );
            assert_eq!(
                std::fs::read_to_string(f.front.join(name)).unwrap(),
                "unchanged"
            );
        }
        assert!(save_backside(&f.cfg, &f.path("BACKSIDE/ckg.json"), "{}").is_err());
        let other = f.root.join("02_C_P_E_L/023_CPEL/BACKSIDE/calls/001_TEST");
        std::fs::create_dir_all(&other).unwrap();
        std::fs::write(other.join("prompt.md"), "untouched").unwrap();
        assert!(save_backside(&f.cfg, &other.join("prompt.md").to_string_lossy(), "bad").is_err());
        assert!(save_backside(&f.cfg, "/etc/passwd", "bad").is_err());
    }
    #[test]
    fn order_changes_preserve_identity_settings_and_paste_source_semantics() {
        let f = Fixture::new();
        let front = f.front.to_string_lossy();
        reorder(
            &f.cfg,
            &front,
            &["ORIGINAL".into(), "SYNOPSIS".into()],
            "page_order",
        )
        .unwrap();
        reorder(&f.cfg, &front, &["SYNOPSIS".into()], "run_order").unwrap();
        let v: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(f.front.join("BACKSIDE/ckg.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(v["model"], "deepseek-chat");
        assert_eq!(v["calls"][0]["run_group"], 1);
        assert_eq!(v["calls"][0]["page_order"], 2);
        assert_eq!(v["calls"][1]["page_order"], 1);
        assert!(v["calls"][1].get("run_order").is_none());
        assert!(reorder(
            &f.cfg,
            &front,
            &["SYNOPSIS".into(), "SYNOPSIS".into()],
            "page_order"
        )
        .is_err());
        assert!(reorder(&f.cfg, &front, &["BOGUS".into()], "run_order").is_err());
        assert!(reorder(&f.cfg, &front, &["SYNOPSIS".into()], "model").is_err());
        assert_eq!(backups(&f.front.join("BACKSIDE/ckg.json")).len(), 1);
    }
    #[test]
    fn results_are_newest_first_and_exclude_nested_station_internals() {
        let f = Fixture::new();
        let out = f.front.join("OUTBOX");
        std::fs::create_dir_all(out.join("nested/BACKSIDE")).unwrap();
        std::fs::create_dir_all(out.join("papers")).unwrap();
        std::fs::write(out.join("nested/BACKSIDE/private.md"), "hidden").unwrap();
        std::fs::write(out.join("old.md"), "old").unwrap();
        std::thread::sleep(Duration::from_millis(20));
        std::fs::write(out.join("papers/new · CKG.md"), "new").unwrap();
        let r = results(&f.cfg, &f.front.to_string_lossy()).unwrap();
        assert_eq!(r["files"].as_array().unwrap().len(), 2);
        assert_eq!(r["files"][0]["name"], "new · CKG.md");
        assert!(open_result(
            &f.cfg,
            &f.front.to_string_lossy(),
            &f.path("BACKSIDE/FOCUS.md")
        )
        .is_err());
    }
    #[cfg(unix)]
    #[test]
    fn symlinks_cannot_extend_edits_or_results() {
        use std::os::unix::fs::symlink;
        let f = Fixture::new();
        let prompt = f.front.join("BACKSIDE/calls/001_SYNOPSIS/prompt.md");
        let target = f.front.join("BACKSIDE/script.py");
        std::fs::write(&target, "untouched").unwrap();
        std::fs::remove_file(&prompt).unwrap();
        symlink(&target, &prompt).unwrap();
        assert!(save_backside(&f.cfg, &prompt.to_string_lossy(), "bad").is_err());
        assert_eq!(std::fs::read_to_string(target).unwrap(), "untouched");
        std::fs::create_dir_all(f.front.join("OUTBOX")).unwrap();
        symlink(
            f.front.join("BACKSIDE/FOCUS.md"),
            f.front.join("OUTBOX/leak.md"),
        )
        .unwrap();
        assert_eq!(
            results(&f.cfg, &f.front.to_string_lossy()).unwrap()["files"]
                .as_array()
                .unwrap()
                .len(),
            0
        );
    }
    #[test]
    fn pump_keeps_split_utf8_and_final_output() {
        let sink = Arc::new(Mutex::new(String::new()));
        struct Bytewise(std::io::Cursor<Vec<u8>>);
        impl Read for Bytewise {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                self.0.read(&mut buf[..1])
            }
        }
        pump(
            Box::new(Bytewise(std::io::Cursor::new(
                "SYNOPSIS → AUDIT · 完了".as_bytes().to_vec(),
            ))),
            Arc::clone(&sink),
        );
        assert_eq!(*sink.lock().unwrap(), "SYNOPSIS → AUDIT · 完了");
    }
}
