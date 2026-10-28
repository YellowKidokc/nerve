//! Night Shift: a queue of jobs that run while David is away from the machine.
//!
//! The idle worker in `idle.rs` does one fixed thing (C0 capture). Night Shift
//! is the general scheduler beside it: a list of tasks, each with a time
//! offset from the moment the machine went idle, run one at a time.
//!
//! ```text
//! idle for idle_minutes ──► session starts (clock = moment input stopped)
//!   +30m  change scan          (what moved, changed, appeared, vanished)
//!   +35m  David OS / FIS sync  (command, reads NIGHTSHIFT_CHANGES)
//!   +3h after "ollama check"   (chained: gap after another task finishes)
//! user returns ──► running task killed if stop_on_return, session ends
//! ```
//!
//! Two rules keep it honest:
//!
//! * **Walk-through first.** A task never runs unattended until it has been
//!   run once from the panel and approved. Approval stores a fingerprint of
//!   the task definition; editing the task voids it.
//! * **Receipts, always.** Every run, scheduled or walked, writes a JSON
//!   record and a full log under `<output_root>/night_shift/`.
//!
//! Night Shift itself never edits source files. Command tasks can — that is
//! their owner's business, and why they must be walked through first.

use crate::idle::{idle_millis, now_iso, sha256_bytes, sha256_file, write_atomic};
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::{BufRead, BufReader, Write};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tracing::{info, warn};

const CREATE_NO_WINDOW: u32 = 0x0800_0000;
const POLL: Duration = Duration::from_secs(15);

/// Sends live output to whoever is watching (the panel during a walk-through).
pub type Emit = Arc<dyn Fn(&str) + Send + Sync>;

// ---------------------------------------------------------------------------
// The queue file
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Queue {
    /// Master switch for unattended runs. Walk-throughs work either way.
    #[serde(default)]
    pub enabled: bool,
    /// Continuous idle time before a night session opens.
    #[serde(default = "default_idle_minutes")]
    pub idle_minutes: u64,
    /// Where receipts, logs, manifests and reports go.
    #[serde(default)]
    pub output_root: String,
    #[serde(default)]
    pub tasks: Vec<Task>,
}

fn default_idle_minutes() -> u64 {
    20
}
fn default_true() -> bool {
    true
}
fn default_max_minutes() -> u64 {
    60
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub notes: String,
    pub job: Job,
    #[serde(default)]
    pub schedule: Schedule,
    /// Hard ceiling on one run.
    #[serde(default = "default_max_minutes")]
    pub max_minutes: u64,
    /// Kill the task when the user comes back. Off for long jobs that are
    /// harmless to finish (an embedding pass), on for anything heavy.
    #[serde(default = "default_true")]
    pub stop_on_return: bool,
    /// Fingerprint of the definition that was walked through and approved.
    #[serde(default)]
    pub approved: Option<Approval>,
    #[serde(default)]
    pub last_run: Option<RunRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub fingerprint: String,
    pub at: String,
    pub run_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Schedule {
    /// Minutes after the idle session's start (when input stopped).
    #[serde(default)]
    pub after_minutes: u64,
    /// If set, wait for this task to finish in the same session, then
    /// `gap_minutes` more. `after_minutes` is then ignored.
    #[serde(default)]
    pub after_task: Option<String>,
    #[serde(default)]
    pub gap_minutes: u64,
}

/// What a task does. Tagged so the JSON reads `{"kind": "command", ...}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Job {
    /// Any program: a Python script, a .bat, David OS, a vectorizer.
    /// Gets `NIGHTSHIFT_OUTPUT`, `NIGHTSHIFT_CHANGES`, `NIGHTSHIFT_RUN_ID`.
    Command {
        program: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        cwd: String,
    },
    /// Diff folders against last night: added, modified, deleted, moved,
    /// copied. Read-only on the folders.
    ChangeScan {
        roots: Vec<String>,
        #[serde(default)]
        extensions: Vec<String>,
    },
    /// Send each file the latest change scan flagged to local Ollama with the
    /// canon reference and a prompt. Writes one report; touches no source.
    OllamaReview {
        #[serde(default = "default_ollama_url")]
        endpoint: String,
        model: String,
        prompt: String,
        #[serde(default)]
        canon_file: String,
        #[serde(default = "default_ollama_max_files")]
        max_files: usize,
    },
}

fn default_ollama_url() -> String {
    "http://127.0.0.1:11434/api/generate".into()
}
fn default_ollama_max_files() -> usize {
    25
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Outcome {
    Complete,
    Failed,
    UserReturned,
    TimedOut,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRecord {
    pub run_id: String,
    pub task_id: String,
    /// "walkthrough" or "night".
    pub mode: String,
    pub started_at: String,
    pub ended_at: String,
    pub outcome: Outcome,
    pub summary: String,
    pub log_path: String,
}

impl Task {
    /// Hash of everything that decides what the task does. Name, notes,
    /// enablement and history are left out so renaming does not void approval.
    pub fn fingerprint(&self) -> String {
        let def = serde_json::json!({
            "job": self.job,
            "max_minutes": self.max_minutes,
        });
        sha256_bytes(def.to_string().as_bytes())
    }

    pub fn is_approved(&self) -> bool {
        self.approved
            .as_ref()
            .map_or(false, |a| a.fingerprint == self.fingerprint())
    }
}

pub fn queue_path() -> PathBuf {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("clipsync-agent");
    std::fs::create_dir_all(&dir).ok();
    dir.join("night_shift.json")
}

/// One lock around load-modify-save so the panel and the engine never
/// overwrite each other's last_run.
fn file_lock() -> &'static Mutex<()> {
    static L: OnceLock<Mutex<()>> = OnceLock::new();
    L.get_or_init(|| Mutex::new(()))
}

pub fn load() -> Queue {
    let path = queue_path();
    match std::fs::read_to_string(&path) {
        Ok(raw) => serde_json::from_str(&raw).unwrap_or_else(|e| {
            warn!("night shift: {} is not valid, using an empty queue: {}", path.display(), e);
            Queue::empty()
        }),
        Err(_) => Queue::starter(),
    }
}

/// The queue as the panel sees it: each task also says whether its approval
/// still matches its definition, so an edited job shows as needing a re-walk.
pub fn load_for_panel() -> serde_json::Value {
    let q = load();
    let mut v = serde_json::to_value(&q).unwrap_or_default();
    if let Some(tasks) = v.get_mut("tasks").and_then(|t| t.as_array_mut()) {
        for (json, task) in tasks.iter_mut().zip(&q.tasks) {
            json["approval_current"] = serde_json::json!(task.is_approved());
        }
    }
    v
}

fn save(q: &Queue) -> Result<()> {
    write_atomic(&queue_path(), serde_json::to_string_pretty(q)?.as_bytes())
}

fn update<F: FnOnce(&mut Queue)>(f: F) -> Result<()> {
    let _g = file_lock().lock().unwrap();
    let mut q = load();
    f(&mut q);
    save(&q)
}

impl Queue {
    fn empty() -> Self {
        Self { enabled: false, idle_minutes: default_idle_minutes(), output_root: String::new(), tasks: vec![] }
    }

    /// First-launch queue: the routine that should always run, disabled and
    /// unapproved, with placeholder paths to fill in from the panel.
    fn starter() -> Self {
        let task = |id: &str, name: &str, notes: &str, job: Job, schedule: Schedule| Task {
            id: id.into(),
            name: name.into(),
            enabled: false,
            notes: notes.into(),
            job,
            schedule,
            max_minutes: default_max_minutes(),
            stop_on_return: true,
            approved: None,
            last_run: None,
        };
        Self {
            enabled: false,
            idle_minutes: default_idle_minutes(),
            output_root: String::new(),
            tasks: vec![
                task(
                    "scan",
                    "File change scan",
                    "What was added, edited, moved, copied or deleted since the last scan.",
                    Job::ChangeScan { roots: vec![r"O:\_Theophysics_v5".into()], extensions: vec!["md".into(), "html".into()] },
                    Schedule { after_minutes: 30, ..Default::default() },
                ),
                task(
                    "david-os",
                    "David OS: refresh FIS pages",
                    "Reads NIGHTSHIFT_CHANGES and updates the FIS HTML pages for what moved.",
                    Job::Command { program: "python".into(), args: vec![r"D:\GitHub\david-os\run.py".into(), "fis-sync".into()], cwd: String::new() },
                    Schedule { after_task: Some("scan".into()), gap_minutes: 1, ..Default::default() },
                ),
                task(
                    "canon-check",
                    "Ollama: check changes against Canon",
                    "Each changed file, read beside the canon reference. Proposals only.",
                    Job::OllamaReview {
                        endpoint: default_ollama_url(),
                        model: "llama3.1:8b".into(),
                        prompt: "Compare this document with the canon reference. List every claim that contradicts, extends, or is missing from canon. Quote exact lines. Do not rewrite anything.".into(),
                        canon_file: String::new(),
                        max_files: default_ollama_max_files(),
                    },
                    Schedule { after_task: Some("david-os".into()), gap_minutes: 5, ..Default::default() },
                ),
            ],
        }
    }
}

// ---------------------------------------------------------------------------
// Running one task
// ---------------------------------------------------------------------------

fn new_run_id(task_id: &str) -> String {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    format!("NS-{}-{:x}", task_id, nanos)
}

fn out_dir(q: &Queue, sub: &str) -> Result<PathBuf> {
    if q.output_root.trim().is_empty() {
        bail!("Set an output folder first (Night Shift settings).");
    }
    let p = Path::new(&q.output_root).join("night_shift").join(sub);
    std::fs::create_dir_all(&p).with_context(|| format!("creating {}", p.display()))?;
    Ok(p)
}

/// Writes to the run log and to the live viewer, if any.
struct Log {
    file: Mutex<std::fs::File>,
    emit: Option<Emit>,
}

impl Log {
    fn line(&self, text: &str) {
        let _ = writeln!(self.file.lock().unwrap(), "{}", text);
        if let Some(emit) = &self.emit {
            emit(&format!("{}\n", text));
        }
    }
}

/// Run one task to completion, cancellation or timeout. Always returns a
/// record and always writes it to `runs/`; failures are outcomes, not errors.
pub fn run_task(q: &Queue, task: &Task, mode: &str, cancel: Arc<AtomicBool>, emit: Option<Emit>) -> RunRecord {
    let run_id = new_run_id(&task.id);
    let started_at = now_iso();
    let runs = match out_dir(q, "runs") {
        Ok(p) => p,
        Err(e) => {
            if let Some(emit) = &emit {
                emit(&format!("✗ {:#}\n", e));
            }
            return RunRecord {
                run_id,
                task_id: task.id.clone(),
                mode: mode.into(),
                started_at: started_at.clone(),
                ended_at: started_at,
                outcome: Outcome::Failed,
                summary: format!("{:#}", e),
                log_path: String::new(),
            };
        }
    };
    let log_path = runs.join(format!("{}.log", run_id));
    let log = Log {
        file: Mutex::new(std::fs::File::create(&log_path).expect("run log")),
        emit,
    };
    log.line(&format!("── {} · {} · {} ──", task.name, mode, started_at));

    let deadline = Instant::now() + Duration::from_secs(task.max_minutes.max(1) * 60);
    let result = match &task.job {
        Job::Command { program, args, cwd } => run_command(q, &run_id, program, args, cwd, &cancel, deadline, &log),
        Job::ChangeScan { roots, extensions } => change_scan(q, &task.id, roots, extensions, &cancel, &log),
        Job::OllamaReview { endpoint, model, prompt, canon_file, max_files } => {
            ollama_review(q, &run_id, endpoint, model, prompt, canon_file, *max_files, &cancel, deadline, &log)
        }
    };
    let (outcome, summary) = match result {
        Ok(s) => (Outcome::Complete, s),
        Err(e) => {
            let msg = format!("{:#}", e);
            let o = if cancel.load(Ordering::Relaxed) {
                Outcome::UserReturned
            } else if Instant::now() >= deadline {
                Outcome::TimedOut
            } else {
                Outcome::Failed
            };
            (o, msg)
        }
    };
    log.line(&format!("── {:?}: {} ──", outcome, summary));

    let record = RunRecord {
        run_id: run_id.clone(),
        task_id: task.id.clone(),
        mode: mode.into(),
        started_at,
        ended_at: now_iso(),
        outcome,
        summary,
        log_path: log_path.to_string_lossy().into(),
    };
    if let Ok(json) = serde_json::to_vec_pretty(&record) {
        let _ = write_atomic(&runs.join(format!("{}.json", run_id)), &json);
    }
    let stored = record.clone();
    let _ = update(|q| {
        if let Some(t) = q.tasks.iter_mut().find(|t| t.id == stored.task_id) {
            t.last_run = Some(stored);
        }
    });
    record
}

fn latest_changes(q: &Queue) -> Option<PathBuf> {
    let p = Path::new(&q.output_root).join("night_shift").join("changes").join("latest.json");
    p.is_file().then_some(p)
}

#[allow(clippy::too_many_arguments)]
fn run_command(
    q: &Queue,
    run_id: &str,
    program: &str,
    args: &[String],
    cwd: &str,
    cancel: &AtomicBool,
    deadline: Instant,
    log: &Log,
) -> Result<String> {
    if program.trim().is_empty() {
        bail!("no program set");
    }
    let mut cmd = Command::new(program);
    cmd.args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("PYTHONIOENCODING", "utf-8")
        .env("PYTHONUNBUFFERED", "1")
        .env("NIGHTSHIFT_RUN_ID", run_id)
        .env("NIGHTSHIFT_OUTPUT", Path::new(&q.output_root).join("night_shift"));
    if let Some(changes) = latest_changes(q) {
        cmd.env("NIGHTSHIFT_CHANGES", changes);
    }
    if !cwd.trim().is_empty() {
        cmd.current_dir(cwd);
    }
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    log.line(&format!("$ {} {}", program, args.join(" ")));
    let mut child = cmd.spawn().with_context(|| format!("starting {}", program))?;

    let lines = Arc::new(Mutex::new(Vec::<String>::new()));
    let readers: Vec<_> = [
        child.stdout.take().map(|s| Box::new(s) as Box<dyn std::io::Read + Send>),
        child.stderr.take().map(|s| Box::new(s) as Box<dyn std::io::Read + Send>),
    ]
    .into_iter()
    .flatten()
    .map(|stream| {
        let lines = Arc::clone(&lines);
        std::thread::spawn(move || {
            for line in BufReader::new(stream).lines().map_while(|l| l.ok()) {
                lines.lock().unwrap().push(line);
            }
        })
    })
    .collect();

    let flush = || {
        for l in lines.lock().unwrap().drain(..) {
            log.line(&l);
        }
    };
    let status = loop {
        flush();
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if cancel.load(Ordering::Relaxed) {
            let _ = child.kill();
            let _ = child.wait();
            bail!("stopped: user returned");
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            bail!("stopped: hit max_minutes");
        }
        std::thread::sleep(Duration::from_millis(300));
    };
    for r in readers {
        let _ = r.join();
    }
    flush();
    if !status.success() {
        bail!("exited with {}", status);
    }
    Ok("exit 0".into())
}

// ---------------------------------------------------------------------------
// Change scan
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Seen {
    size: u64,
    mtime: u64,
    sha256: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Changes {
    pub scanned_at: String,
    pub roots: Vec<String>,
    /// First scan of a root only builds the baseline; nothing is "added".
    pub baseline: bool,
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub deleted: Vec<String>,
    /// `{from, to}` — the old path vanished and the same bytes appeared here.
    pub moved: Vec<BTreeMap<String, String>>,
    /// `{from, to}` — new path whose bytes match a file that is still there.
    pub copied: Vec<BTreeMap<String, String>>,
}

impl Changes {
    fn total(&self) -> usize {
        self.added.len() + self.modified.len() + self.deleted.len() + self.moved.len() + self.copied.len()
    }
    /// Files worth a second look by later tasks: content that is new here.
    pub fn touched(&self) -> Vec<String> {
        let mut v = self.added.clone();
        v.extend(self.modified.iter().cloned());
        v.extend(self.moved.iter().filter_map(|m| m.get("to").cloned()));
        v
    }
}

fn walk(root: &Path, exts: &HashSet<String>, cancel: &AtomicBool, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(root) else { return };
    for e in entries.flatten() {
        if cancel.load(Ordering::Relaxed) {
            return;
        }
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        if name.starts_with('.') || name == "node_modules" {
            continue;
        }
        match e.file_type() {
            Ok(t) if t.is_dir() => walk(&p, exts, cancel, out),
            Ok(t) if t.is_file() => {
                let ext = p.extension().map(|x| x.to_string_lossy().to_lowercase()).unwrap_or_default();
                if exts.is_empty() || exts.contains(&ext) {
                    out.push(p);
                }
            }
            _ => {}
        }
    }
}

/// Compare a fresh walk with the previous manifest. Hashes are only
/// recomputed when size or mtime moved, so a quiet night is cheap.
fn diff(prev: &HashMap<String, Seen>, now: &HashMap<String, Seen>) -> Changes {
    let mut c = Changes::default();
    let mut gone: Vec<&String> = prev.keys().filter(|k| !now.contains_key(*k)).collect();
    gone.sort();
    let mut new: Vec<&String> = now.keys().filter(|k| !prev.contains_key(*k)).collect();
    new.sort();

    let mut gone_by_hash: HashMap<&str, Vec<&String>> = HashMap::new();
    for g in &gone {
        gone_by_hash.entry(prev[*g].sha256.as_str()).or_default().push(g);
    }
    let mut stayed_by_hash: HashMap<&str, &String> = HashMap::new();
    for (k, v) in now {
        if prev.contains_key(k) {
            stayed_by_hash.entry(v.sha256.as_str()).or_insert(k);
        }
    }
    let mut moved_from: HashSet<&String> = HashSet::new();
    let pair = |from: &str, to: &str| BTreeMap::from([("from".to_string(), from.to_string()), ("to".to_string(), to.to_string())]);

    for n in new {
        let h = now[n].sha256.as_str();
        if let Some(src) = gone_by_hash.get_mut(h).and_then(|v| v.pop()) {
            moved_from.insert(src);
            c.moved.push(pair(src, n));
        } else if let Some(src) = stayed_by_hash.get(h) {
            c.copied.push(pair(src, n));
        } else {
            c.added.push(n.clone());
        }
    }
    c.deleted = gone.into_iter().filter(|g| !moved_from.contains(g)).cloned().collect();
    let mut modified: Vec<String> = now
        .iter()
        .filter(|(k, v)| prev.get(*k).map_or(false, |p| p.sha256 != v.sha256))
        .map(|(k, _)| k.clone())
        .collect();
    modified.sort();
    c.modified = modified;
    c
}

fn change_scan(q: &Queue, task_id: &str, roots: &[String], exts: &[String], cancel: &AtomicBool, log: &Log) -> Result<String> {
    if roots.is_empty() {
        bail!("no folders to scan");
    }
    let manifest_path = out_dir(q, "manifests")?.join(format!("{}.json", task_id));
    let first = !manifest_path.is_file();
    let prev: HashMap<String, Seen> = if first {
        HashMap::new()
    } else {
        serde_json::from_slice(&std::fs::read(&manifest_path)?).context("reading manifest")?
    };

    let exts: HashSet<String> = exts.iter().map(|e| e.trim_start_matches('.').to_lowercase()).collect();
    let mut files = Vec::new();
    for root in roots {
        let r = Path::new(root);
        if !r.is_dir() {
            bail!("folder not found: {}", root);
        }
        walk(r, &exts, cancel, &mut files);
    }
    if cancel.load(Ordering::Relaxed) {
        bail!("stopped: user returned");
    }
    log.line(&format!("{} files in {} folder(s)", files.len(), roots.len()));

    let mut now: HashMap<String, Seen> = HashMap::with_capacity(files.len());
    let mut hashed = 0usize;
    for f in files {
        if cancel.load(Ordering::Relaxed) {
            // Keep the old manifest: a half scan must not look like deletions.
            bail!("stopped: user returned");
        }
        let Ok(meta) = std::fs::metadata(&f) else { continue };
        let key = f.to_string_lossy().to_string();
        let mtime = meta.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map_or(0, |d| d.as_secs());
        let seen = match prev.get(&key) {
            Some(p) if p.size == meta.len() && p.mtime == mtime => p.clone(),
            _ => {
                hashed += 1;
                match sha256_file(&f) {
                    Ok(h) => Seen { size: meta.len(), mtime, sha256: h },
                    Err(e) => {
                        log.line(&format!("skip {}: {}", key, e));
                        continue;
                    }
                }
            }
        };
        now.insert(key, seen);
    }

    let mut changes = if first { Changes::default() } else { diff(&prev, &now) };
    changes.scanned_at = now_iso();
    changes.roots = roots.to_vec();
    changes.baseline = first;

    let changes_dir = out_dir(q, "changes")?;
    let json = serde_json::to_vec_pretty(&changes)?;
    let stamp = changes.scanned_at.replace(':', "-");
    write_atomic(&changes_dir.join(format!("{}-{}.json", task_id, stamp)), &json)?;
    write_atomic(&changes_dir.join("latest.json"), &json)?;
    write_atomic(&manifest_path, &serde_json::to_vec(&now)?)?;

    for (label, list) in [("added", &changes.added), ("modified", &changes.modified), ("deleted", &changes.deleted)] {
        for p in list.iter().take(40) {
            log.line(&format!("  {:<8} {}", label, p));
        }
    }
    for (label, list) in [("moved", &changes.moved), ("copied", &changes.copied)] {
        for m in list.iter().take(40) {
            log.line(&format!("  {:<8} {} → {}", label, m["from"], m["to"]));
        }
    }
    Ok(if first {
        format!("baseline of {} files recorded", now.len())
    } else {
        format!(
            "{} changes ({} added, {} modified, {} deleted, {} moved, {} copied); {} rehashed",
            changes.total(),
            changes.added.len(),
            changes.modified.len(),
            changes.deleted.len(),
            changes.moved.len(),
            changes.copied.len(),
            hashed
        )
    })
}

// ---------------------------------------------------------------------------
// Ollama review
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn ollama_review(
    q: &Queue,
    run_id: &str,
    endpoint: &str,
    model: &str,
    prompt: &str,
    canon_file: &str,
    max_files: usize,
    cancel: &AtomicBool,
    deadline: Instant,
    log: &Log,
) -> Result<String> {
    let changes_path = latest_changes(q).ok_or_else(|| anyhow!("no change scan yet: run a change_scan task first"))?;
    let changes: Changes = serde_json::from_slice(&std::fs::read(&changes_path)?)?;
    let files = changes.touched();
    if files.is_empty() {
        return Ok("no changed files to review".into());
    }
    let canon = if canon_file.trim().is_empty() {
        String::new()
    } else {
        std::fs::read_to_string(canon_file).with_context(|| format!("reading canon {}", canon_file))?
    };

    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    let client = reqwest::Client::builder().timeout(Duration::from_secs(600)).build()?;
    let mut report = format!(
        "# Night Shift · Ollama review\n\nrun `{}` · model `{}` · {}\n\nAll findings are AI_PROPOSED. Nothing was changed.\n\n",
        run_id, model, now_iso()
    );
    let mut done = 0;
    for f in files.iter().take(max_files) {
        if cancel.load(Ordering::Relaxed) || Instant::now() >= deadline {
            break;
        }
        let Ok(text) = std::fs::read_to_string(f) else {
            log.line(&format!("skip (not text) {}", f));
            continue;
        };
        let text: String = text.chars().take(24_000).collect();
        let full = if canon.is_empty() {
            format!("{}\n\n=== DOCUMENT: {} ===\n{}", prompt, f, text)
        } else {
            format!("{}\n\n=== CANON REFERENCE ===\n{}\n\n=== DOCUMENT: {} ===\n{}", prompt, canon, f, text)
        };
        log.line(&format!("→ {}", f));
        let body = serde_json::json!({ "model": model, "prompt": full, "stream": false });
        let reply: Result<String> = rt.block_on(async {
            let r = client.post(endpoint).json(&body).send().await?.error_for_status()?;
            let v: serde_json::Value = r.json().await?;
            Ok(v.get("response").and_then(|s| s.as_str()).unwrap_or("").to_string())
        });
        match reply {
            Ok(answer) => {
                report.push_str(&format!("## {}\n\n{}\n\n", f, answer.trim()));
                done += 1;
            }
            Err(e) => {
                log.line(&format!("  ✗ {:#}", e));
                report.push_str(&format!("## {}\n\n_Ollama failed: {:#}_\n\n", f, e));
            }
        }
    }
    let path = out_dir(q, "reports")?.join(format!("{}.md", run_id));
    write_atomic(&path, report.as_bytes())?;
    log.line(&format!("report: {}", path.display()));
    if cancel.load(Ordering::Relaxed) {
        bail!("stopped after {} file(s): user returned (partial report kept)", done);
    }
    Ok(format!("{} of {} file(s) reviewed → {}", done, files.len().min(max_files), path.display()))
}

// ---------------------------------------------------------------------------
// Walk-through (panel) and approval
// ---------------------------------------------------------------------------

fn walk_cancel() -> &'static Mutex<HashMap<String, Arc<AtomicBool>>> {
    static M: OnceLock<Mutex<HashMap<String, Arc<AtomicBool>>>> = OnceLock::new();
    M.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Run a task now with live output. `done` receives the record.
pub fn walk_through(task_id: &str, emit: Emit, done: Box<dyn FnOnce(RunRecord) + Send>) -> Result<()> {
    let q = load();
    let task = q.tasks.iter().find(|t| t.id == task_id).cloned().ok_or_else(|| anyhow!("no task '{}'", task_id))?;
    let cancel = Arc::new(AtomicBool::new(false));
    {
        let mut m = walk_cancel().lock().unwrap();
        if m.contains_key(task_id) {
            bail!("'{}' is already running", task.name);
        }
        m.insert(task_id.to_string(), Arc::clone(&cancel));
    }
    let id = task_id.to_string();
    std::thread::spawn(move || {
        let rec = run_task(&q, &task, "walkthrough", cancel, Some(emit));
        walk_cancel().lock().unwrap().remove(&id);
        done(rec);
    });
    Ok(())
}

pub fn stop_walk(task_id: &str) {
    if let Some(c) = walk_cancel().lock().unwrap().get(task_id) {
        c.store(true, Ordering::Relaxed);
    }
}

/// Approve a task for unattended runs. Only allowed right after a complete
/// walk-through of the exact current definition.
pub fn approve(task_id: &str) -> Result<()> {
    let _g = file_lock().lock().unwrap();
    let mut q = load();
    let t = q.tasks.iter_mut().find(|t| t.id == task_id).ok_or_else(|| anyhow!("no task '{}'", task_id))?;
    let rec = t.last_run.as_ref().ok_or_else(|| anyhow!("walk it through once first"))?;
    if rec.mode != "walkthrough" || rec.outcome != Outcome::Complete {
        bail!("the last run was not a completed walk-through");
    }
    t.approved = Some(Approval { fingerprint: t.fingerprint(), at: now_iso(), run_id: rec.run_id.clone() });
    t.enabled = true;
    save(&q)
}

/// Replace the queue from the panel. Approvals and history are carried over
/// from disk, never taken from the panel, so the panel cannot forge them.
pub fn save_from_panel(mut incoming: Queue) -> Result<()> {
    let _g = file_lock().lock().unwrap();
    let current = load();
    let mut ids = HashSet::new();
    for t in &mut incoming.tasks {
        if t.id.trim().is_empty() || !ids.insert(t.id.clone()) {
            bail!("every task needs a unique id (problem: '{}')", t.id);
        }
        let old = current.tasks.iter().find(|o| o.id == t.id);
        t.approved = old.and_then(|o| o.approved.clone());
        t.last_run = old.and_then(|o| o.last_run.clone());
    }
    save(&incoming)
}

// ---------------------------------------------------------------------------
// The night engine
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize)]
pub struct Status {
    pub phase: String,
    pub idle_minutes: u64,
    pub session_started: Option<String>,
    pub running: Option<String>,
    pub finished: Vec<String>,
}

fn status_cell() -> &'static Mutex<Status> {
    static S: OnceLock<Mutex<Status>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(Status { phase: "awake".into(), ..Default::default() }))
}

pub fn status() -> Status {
    let mut s = status_cell().lock().unwrap().clone();
    s.idle_minutes = idle_millis() / 60_000;
    s
}

/// Which approved task, if any, is due `elapsed` minutes into a session.
/// `finished` maps task id → minutes into the session it ended.
pub fn next_due<'a>(q: &'a Queue, elapsed: u64, finished: &HashMap<String, u64>) -> Option<&'a Task> {
    q.tasks
        .iter()
        .filter(|t| t.enabled && t.is_approved() && !finished.contains_key(&t.id))
        .filter(|t| match &t.schedule.after_task {
            Some(dep) => finished.get(dep).map_or(false, |end| elapsed >= end + t.schedule.gap_minutes),
            None => elapsed >= t.schedule.after_minutes,
        })
        .min_by_key(|t| t.schedule.after_minutes)
}

pub fn engine() -> Result<()> {
    let mut session: Option<(Instant, HashMap<String, u64>)> = None;
    loop {
        std::thread::sleep(POLL);
        let q = load();
        let idle = Duration::from_millis(idle_millis());
        let set = |phase: &str| status_cell().lock().unwrap().phase = phase.into();

        if !q.enabled || idle < Duration::from_secs(q.idle_minutes * 60) {
            if session.take().is_some() {
                info!("night shift: user is back, session closed");
            }
            *status_cell().lock().unwrap() = Status { phase: if q.enabled { "awake" } else { "off" }.into(), ..Default::default() };
            continue;
        }

        // The session clock starts when input stopped, not when we noticed.
        let (start, finished) = session.get_or_insert_with(|| {
            info!("night shift: idle {} min, session open", idle.as_secs() / 60);
            let mut s = status_cell().lock().unwrap();
            s.session_started = Some(now_iso());
            s.finished.clear();
            (Instant::now().checked_sub(idle).unwrap_or_else(Instant::now), HashMap::new())
        });
        let elapsed = start.elapsed().as_secs() / 60;
        let Some(task) = next_due(&q, elapsed, finished).cloned() else {
            set("idle · waiting");
            continue;
        };

        set("running");
        status_cell().lock().unwrap().running = Some(task.name.clone());
        let cancel = Arc::new(AtomicBool::new(false));
        let watch = Arc::clone(&cancel);
        let stop_on_return = task.stop_on_return;
        let watcher = std::thread::spawn(move || {
            while !watch.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_secs(2));
                if stop_on_return && idle_millis() < 60_000 {
                    watch.store(true, Ordering::Relaxed);
                }
            }
        });
        let rec = run_task(&q, &task, "night", Arc::clone(&cancel), None);
        let returned = cancel.swap(true, Ordering::Relaxed);
        watcher.join().ok();
        info!("night shift: {} → {:?}: {}", task.name, rec.outcome, rec.summary);

        finished.insert(task.id.clone(), start.elapsed().as_secs() / 60);
        let mut s = status_cell().lock().unwrap();
        s.running = None;
        s.finished.push(format!("{} · {:?}", task.name, rec.outcome));
        if returned {
            drop(s);
            session = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seen(h: &str) -> Seen {
        Seen { size: 1, mtime: 1, sha256: h.into() }
    }

    #[test]
    fn diff_names_every_kind_of_change() {
        let prev = HashMap::from([
            ("a".to_string(), seen("A")),
            ("b".to_string(), seen("B")),
            ("c".to_string(), seen("C")),
            ("d".to_string(), seen("D")),
        ]);
        let now = HashMap::from([
            ("a".to_string(), seen("A")),  // unchanged
            ("b".to_string(), seen("B2")), // modified
            ("c2".to_string(), seen("C")), // c moved
            ("a2".to_string(), seen("A")), // copy of a
            ("e".to_string(), seen("E")),  // added
        ]); // d deleted
        let c = diff(&prev, &now);
        assert_eq!(c.added, vec!["e"]);
        assert_eq!(c.modified, vec!["b"]);
        assert_eq!(c.deleted, vec!["d"]);
        assert_eq!(c.moved[0]["from"], "c");
        assert_eq!(c.moved[0]["to"], "c2");
        assert_eq!(c.copied[0]["from"], "a");
        assert_eq!(c.copied[0]["to"], "a2");
    }

    fn approved(mut t: Task) -> Task {
        t.enabled = true;
        t.approved = Some(Approval { fingerprint: t.fingerprint(), at: String::new(), run_id: String::new() });
        t
    }

    #[test]
    fn schedule_respects_offsets_chains_and_approval() {
        let mut q = Queue::starter();
        q.tasks = q.tasks.into_iter().map(approved).collect();
        let mut done = HashMap::new();
        assert!(next_due(&q, 10, &done).is_none());
        assert_eq!(next_due(&q, 30, &done).unwrap().id, "scan");
        done.insert("scan".to_string(), 32);
        assert!(next_due(&q, 32, &done).is_none());
        assert_eq!(next_due(&q, 33, &done).unwrap().id, "david-os");

        // Editing the job voids approval.
        if let Job::Command { args, .. } = &mut q.tasks[1].job {
            args.push("--extra".into());
        }
        assert!(next_due(&q, 33, &done).is_none());
    }

    #[test]
    fn change_scan_baseline_then_diff() {
        let base = std::env::temp_dir().join("nerve-ns-test");
        std::fs::remove_dir_all(&base).ok();
        let src = base.join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("one.md"), "one").unwrap();
        std::fs::write(src.join("two.md"), "two").unwrap();
        let q = Queue { output_root: base.join("out").to_string_lossy().into(), ..Queue::empty() };
        let log = Log { file: Mutex::new(std::fs::File::create(base.join("log")).unwrap()), emit: None };
        let roots = vec![src.to_string_lossy().to_string()];
        let cancel = AtomicBool::new(false);

        let s = change_scan(&q, "t", &roots, &["md".into()], &cancel, &log).unwrap();
        assert!(s.starts_with("baseline of 2"));

        std::fs::rename(src.join("one.md"), src.join("uno.md")).unwrap();
        std::fs::write(src.join("three.md"), "three").unwrap();
        change_scan(&q, "t", &roots, &["md".into()], &cancel, &log).unwrap();
        let c: Changes = serde_json::from_slice(&std::fs::read(latest_changes(&q).unwrap()).unwrap()).unwrap();
        assert_eq!(c.moved.len(), 1);
        assert_eq!(c.added.len(), 1);
        assert!(c.deleted.is_empty());
    }
}
