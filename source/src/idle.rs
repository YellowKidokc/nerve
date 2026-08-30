//! Idle Canon Worker — implementation steps 1-5 of `IDLE_CANON_WORKER_SPEC.md`.
//!
//! When the machine has been continuously idle past a configured threshold,
//! this worker performs *deterministic* C0 capture over approved roots and
//! writes receipts and a daily report. It stops the moment the user returns.
//!
//! The spec's ordering is deliberate and is followed here: the idle lifecycle,
//! cancellation, path boundaries, deterministic capture and receipts are earned
//! first. There is no AI classification in this module and no code path that
//! could reach one — step 7 is a later, separate change.
//!
//! Three properties are structural rather than checked at runtime:
//!
//! * **No source mutation.** Nothing here opens a source path for writing.
//!   That is why the stop path needs no rollback.
//! * **No admission.** The only authority string written is `C0_SIDECAR`.
//! * **No canon database access.** This module never touches SQLite.

use crate::config::{Config, IdleWorkerConfig};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{info, warn};

/// Authority ceiling. Declared once so a search for the string finds exactly
/// one producer of it.
const CANONIZATION_CLASS: &str = "C0_SIDECAR";

/// How often the idle detector samples. Also the worst-case latency between
/// the user touching the machine and the worker observing it.
const POLL: Duration = Duration::from_secs(5);

/// Guards against two runs overlapping inside one process. The spec also
/// requires a cross-process guard; that is the on-disk lock in `run_state/`.
static RUN_ACTIVE: AtomicBool = AtomicBool::new(false);

// ---------------------------------------------------------------------------
// Step 1: idle detection
// ---------------------------------------------------------------------------

/// Milliseconds since the last keyboard or mouse input, system-wide.
///
/// `GetLastInputInfo` reports input across the whole session, which is what
/// "the user came back" means here — a hook of our own would only observe the
/// input we already handle.
#[cfg(windows)]
pub fn idle_millis() -> u64 {
    use windows::Win32::System::SystemInformation::GetTickCount;
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};

    let mut info = LASTINPUTINFO {
        cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
        dwTime: 0,
    };
    unsafe {
        if GetLastInputInfo(&mut info).as_bool() {
            // Both values come from the same 32-bit tick counter, so the
            // subtraction stays correct across its ~49-day wrap.
            GetTickCount().wrapping_sub(info.dwTime) as u64
        } else {
            0
        }
    }
}

#[cfg(not(windows))]
pub fn idle_millis() -> u64 {
    0
}

// ---------------------------------------------------------------------------
// Receipts
// ---------------------------------------------------------------------------

/// Terminal state of a run. `SafeHalt` is a first-class outcome, not an error:
/// a failed preflight must say so on disk rather than silently skip, so that
/// "nothing happened last night" stays distinguishable from "nothing was
/// recorded".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunOutcome {
    Complete,
    UserReturned,
    SafeHalt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub run_id: String,
    pub started_at: String,
    pub ended_at: String,
    pub outcome: RunOutcome,
    pub idle_minutes_at_start: u64,
    pub files_inspected: usize,
    pub files_skipped: usize,
    pub sidecars_created: usize,
    pub sidecars_versioned: usize,
    pub sidecars_unchanged: usize,
    /// Located reasons. A bare failure is never sufficient, so every halt and
    /// every skip names what tripped and where.
    pub notes: Vec<String>,
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub path: String,
    pub sha256: String,
}

// ---------------------------------------------------------------------------
// Step 2: configuration and approved-root validation
// ---------------------------------------------------------------------------

/// Resolved, validated paths for one run.
struct Bounds {
    roots: Vec<PathBuf>,
    output: PathBuf,
}

/// Strip Windows' extended-length prefix from a canonicalised path.
///
/// `canonicalize` returns `\\?\D:\...`, which is correct for the Win32 API and
/// wrong for everything else: it does not compare equal to the paths recorded
/// elsewhere in the canon corpus, and it is not what a downstream reader — human
/// or AI — will match against. The prefix is stripped for storage only; the
/// resolved `PathBuf` keeps it for filesystem calls.
fn display_path(p: &Path) -> String {
    let s = p.to_string_lossy().replace('\\', "/");
    s.strip_prefix("//?/").unwrap_or(&s).to_string()
}

/// Preflight gates 4 and 5.
///
/// The containment check runs in both directions. Output-inside-source would
/// make the worker's own writes look like source changes on the next run;
/// source-inside-output would let a later cleanup of the output tree delete
/// corpus files. Neither is recoverable by inspection afterwards, so both are
/// refused before any work starts.
fn resolve_bounds(cfg: &IdleWorkerConfig) -> Result<Bounds> {
    if cfg.approved_roots.is_empty() {
        bail!("no approved_roots configured; the worker has nothing it may read");
    }
    if cfg.output_root.trim().is_empty() {
        bail!("no output_root configured; the worker has nowhere it may write");
    }

    let output = PathBuf::from(&cfg.output_root);
    if !output.is_absolute() {
        bail!("output_root '{}' is not an absolute path", cfg.output_root);
    }
    std::fs::create_dir_all(&output)?;
    let output = output.canonicalize()?;

    let mut roots = Vec::new();
    for raw in &cfg.approved_roots {
        let p = PathBuf::from(raw);
        if !p.is_absolute() {
            bail!("approved root '{}' is not an absolute path", raw);
        }
        let p = p
            .canonicalize()
            .map_err(|e| anyhow::anyhow!("approved root '{}' does not resolve: {}", raw, e))?;
        if output.starts_with(&p) {
            bail!(
                "output_root '{}' lies inside source root '{}'",
                output.display(),
                p.display()
            );
        }
        if p.starts_with(&output) {
            bail!(
                "source root '{}' lies inside output_root '{}'",
                p.display(),
                output.display()
            );
        }
        roots.push(p);
    }

    Ok(Bounds { roots, output })
}

// ---------------------------------------------------------------------------
// Step 3: single-run lock
// ---------------------------------------------------------------------------

/// On-disk lock, released on drop so a panic cannot strand it.
struct RunLock(PathBuf);

impl RunLock {
    fn acquire(output: &Path, run_id: &str) -> Result<Self> {
        let dir = output.join("run_state");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("idle_worker.lock");

        // create_new is the atomic half of this; the staleness sweep below
        // handles a previous process that died holding the lock.
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut f) => {
                writeln!(f, "{} {}", run_id, now_iso())?;
                Ok(Self(path))
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                let age = std::fs::metadata(&path)
                    .and_then(|m| m.modified())
                    .ok()
                    .and_then(|t| t.elapsed().ok())
                    .unwrap_or_default();
                if age > Duration::from_secs(12 * 3600) {
                    warn!("idle: clearing a lock left behind {}s ago", age.as_secs());
                    std::fs::remove_file(&path)?;
                    return Self::acquire(output, run_id);
                }
                bail!("another idle run holds the lock at {}", path.display())
            }
            Err(e) => Err(e.into()),
        }
    }
}

impl Drop for RunLock {
    fn drop(&mut self) {
        std::fs::remove_file(&self.0).ok();
    }
}

// ---------------------------------------------------------------------------
// Hashing and time
// ---------------------------------------------------------------------------

fn sha256_file(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    let mut f = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut f, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn now_iso() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = secs / 86_400;
    let tod = secs % 86_400;
    let (y, m, d) = civil_from_days(days as i64);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y,
        m,
        d,
        tod / 3600,
        (tod % 3600) / 60,
        tod % 60
    )
}

fn today_stamp() -> String {
    now_iso()[..10].to_string()
}

/// Howard Hinnant's days-from-civil, inverted. Exact for every date this will
/// see, and avoids a date dependency for one format string.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// Run id without a uuid dependency: a time prefix keeps runs sortable, and
/// the hash suffix separates two runs starting within the same second.
fn new_run_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let h = sha256_bytes(&nanos.to_le_bytes());
    let stamp = now_iso().replace(':', "").replace('-', "").replace('Z', "");
    format!("{}-{}", stamp, &h[..8])
}

// ---------------------------------------------------------------------------
// Step 4: deterministic C0 sidecar capture
// ---------------------------------------------------------------------------

/// The C0 envelope, matching the spec's minimum shape.
///
/// `classification` and `authority` are written empty and false on purpose:
/// they are the slots a *later* human ruling fills, and emitting them empty is
/// what makes their absence explicit rather than merely unrecorded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sidecar {
    pub sidecar_id: String,
    pub canonization_class: String,
    pub status: String,
    pub capture: Capture,
    pub classification: Classification,
    pub authority: Authority,
    /// Append-only. A changed source adds a version; it never overwrites one,
    /// or the record would stop being evidence of what the file used to say.
    pub versions: Vec<VersionEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capture {
    pub method: String,
    pub source_path: String,
    pub captured_at: String,
    pub captured_by: String,
    pub source_sha256: String,
    pub source_bytes: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Classification {
    pub inherited: Vec<String>,
    pub discovered: Vec<String>,
    pub ai_proposals: Vec<String>,
    pub human_ruling: Option<String>,
}

/// All three are false for every record this module writes. There is no code
/// path here that sets any of them true.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Authority {
    pub candidate: bool,
    pub admitted: bool,
    pub canonical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionEntry {
    pub version: u32,
    pub source_sha256: String,
    pub observed_at: String,
}

/// What one file's capture did, for the counters and the report.
enum CaptureResult {
    Created,
    Versioned,
    Unchanged,
}

/// Deterministic capture of one source file.
///
/// Idempotent by source hash (acceptance test 13) and version-on-change rather
/// than overwrite (acceptance test 14). Both properties come from reading any
/// existing sidecar first and comparing hashes.
fn capture_file(src: &Path, root: &Path, out: &Path) -> Result<CaptureResult> {
    let meta = std::fs::metadata(src)?;
    let hash = sha256_file(src)?;
    let rel = src.strip_prefix(root).unwrap_or(src);

    // Sidecar identity derives from the path, so a re-run finds the same
    // record instead of accumulating duplicates for one file.
    let id = format!("SC-{}", &sha256_bytes(rel.to_string_lossy().as_bytes())[..16]);
    let dir = out.join("sidecars");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.json", id));

    if path.exists() {
        let existing: Sidecar = serde_json::from_str(&std::fs::read_to_string(&path)?)?;
        if existing.capture.source_sha256 == hash {
            return Ok(CaptureResult::Unchanged);
        }
        let mut updated = existing;
        let next = updated.versions.last().map(|v| v.version).unwrap_or(0) + 1;
        updated.versions.push(VersionEntry {
            version: next,
            source_sha256: hash.clone(),
            observed_at: now_iso(),
        });
        updated.capture.source_sha256 = hash;
        updated.capture.captured_at = now_iso();
        updated.capture.source_bytes = meta.len();
        write_atomic(&path, serde_json::to_string_pretty(&updated)?.as_bytes())?;
        return Ok(CaptureResult::Versioned);
    }

    let sidecar = Sidecar {
        sidecar_id: id,
        canonization_class: CANONIZATION_CLASS.into(),
        status: "CAPTURED_UNREVIEWED".into(),
        capture: Capture {
            method: "IDLE_DETERMINISTIC_SCAN".into(),
            source_path: display_path(src),
            captured_at: now_iso(),
            captured_by: std::env::var("USERNAME").unwrap_or_else(|_| "unknown".into()),
            source_sha256: hash.clone(),
            source_bytes: meta.len(),
        },
        classification: Classification::default(),
        authority: Authority::default(),
        versions: vec![VersionEntry {
            version: 1,
            source_sha256: hash,
            observed_at: now_iso(),
        }],
    };
    write_atomic(&path, serde_json::to_string_pretty(&sidecar)?.as_bytes())?;
    Ok(CaptureResult::Created)
}

/// Write via a temp file and rename, so a stop mid-write cannot leave a
/// half-parsed sidecar behind. This is the "finish only the current atomic
/// write" clause of the stop behaviour.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Run
// ---------------------------------------------------------------------------

/// Walk one approved root, breadth-bounded, honouring the cancel flag.
fn collect_files(root: &Path, cfg: &IdleWorkerConfig, cancel: &AtomicBool) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        if cancel.load(Ordering::Relaxed) || out.len() >= cfg.max_files_per_run {
            break;
        }
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let p = entry.path();
            let name = entry.file_name();
            let name = name.to_string_lossy();
            // Skip VCS and dependency trees: large, machine-owned, and never
            // the material a capture is meant to preserve.
            if name.starts_with('.') || name == "node_modules" || name == "__pycache__" {
                continue;
            }
            if p.is_dir() {
                stack.push(p);
            } else if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                let ext = ext.to_ascii_lowercase();
                if cfg.extensions.iter().any(|e| e.eq_ignore_ascii_case(&ext)) {
                    out.push(p);
                }
            }
        }
    }
    out
}

/// One bounded run. Returns the receipt; never propagates a preflight failure
/// as an error, because a `SafeHalt` receipt is the required outcome.
pub fn run_once(cfg: &IdleWorkerConfig, cancel: Arc<AtomicBool>) -> Receipt {
    let run_id = new_run_id();
    let started = now_iso();
    let idle_at_start = idle_millis() / 60_000;

    let mut receipt = Receipt {
        run_id: run_id.clone(),
        started_at: started,
        ended_at: String::new(),
        outcome: RunOutcome::SafeHalt,
        idle_minutes_at_start: idle_at_start,
        files_inspected: 0,
        files_skipped: 0,
        sidecars_created: 0,
        sidecars_versioned: 0,
        sidecars_unchanged: 0,
        notes: Vec::new(),
        artifacts: Vec::new(),
    };

    let bounds = match resolve_bounds(cfg) {
        Ok(b) => b,
        Err(e) => {
            receipt.notes.push(format!("preflight: {}", e));
            receipt.ended_at = now_iso();
            return receipt;
        }
    };

    let _lock = match RunLock::acquire(&bounds.output, &run_id) {
        Ok(l) => l,
        Err(e) => {
            receipt.notes.push(format!("preflight: {}", e));
            receipt.ended_at = now_iso();
            // Recorded even though the lock was refused, so a contended night
            // is visible rather than silent.
            write_receipt(&bounds.output, &receipt).ok();
            return receipt;
        }
    };

    let deadline = std::time::Instant::now() + Duration::from_secs(cfg.max_run_minutes * 60);
    let mut stopped_by_user = false;

    'roots: for root in &bounds.roots {
        // Checked before the walk as well as inside it. `collect_files`
        // returns early when cancelled, which would otherwise leave an empty
        // file list indistinguishable from a root with nothing in it — and the
        // run would report `Complete` when it had in fact been stopped.
        if cancel.load(Ordering::Relaxed) {
            stopped_by_user = true;
            break 'roots;
        }
        let files = collect_files(root, cfg, &cancel);
        if cancel.load(Ordering::Relaxed) {
            stopped_by_user = true;
            break 'roots;
        }

        for file in files {
            if cancel.load(Ordering::Relaxed) {
                stopped_by_user = true;
                break 'roots;
            }
            if std::time::Instant::now() >= deadline {
                receipt.notes.push(format!(
                    "max_run_minutes ({}) reached; remaining files left for the next run",
                    cfg.max_run_minutes
                ));
                break 'roots;
            }
            if receipt.files_inspected >= cfg.max_files_per_run {
                receipt.notes.push(format!(
                    "max_files_per_run ({}) reached",
                    cfg.max_files_per_run
                ));
                break 'roots;
            }

            match std::fs::metadata(&file) {
                Ok(m) if m.len() > cfg.max_bytes_per_file => {
                    receipt.files_skipped += 1;
                    receipt.notes.push(format!(
                        "skipped {}: {} bytes exceeds max_bytes_per_file",
                        file.display(),
                        m.len()
                    ));
                    continue;
                }
                Err(e) => {
                    receipt.files_skipped += 1;
                    receipt
                        .notes
                        .push(format!("skipped {}: {}", file.display(), e));
                    continue;
                }
                _ => {}
            }

            receipt.files_inspected += 1;
            match capture_file(&file, root, &bounds.output) {
                Ok(CaptureResult::Created) => receipt.sidecars_created += 1,
                Ok(CaptureResult::Versioned) => receipt.sidecars_versioned += 1,
                Ok(CaptureResult::Unchanged) => receipt.sidecars_unchanged += 1,
                Err(e) => {
                    receipt.files_skipped += 1;
                    receipt
                        .notes
                        .push(format!("capture failed for {}: {}", file.display(), e));
                }
            }
        }
    }

    receipt.outcome = if stopped_by_user {
        RunOutcome::UserReturned
    } else {
        RunOutcome::Complete
    };
    receipt.ended_at = now_iso();

    match write_report(&bounds.output, &receipt) {
        Ok(a) => receipt.artifacts.push(a),
        Err(e) => receipt.notes.push(format!("report write failed: {}", e)),
    }
    if let Err(e) = write_receipt(&bounds.output, &receipt) {
        warn!("idle: receipt write failed: {}", e);
    }

    receipt
}

/// Append-only receipts. One JSON object per line, so a truncated write costs
/// at most the last run rather than the file.
fn write_receipt(output: &Path, receipt: &Receipt) -> Result<()> {
    let dir = output.join("receipts");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join("receipts.jsonl");
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    writeln!(f, "{}", serde_json::to_string(receipt)?)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Step 5: daily report
// ---------------------------------------------------------------------------

/// The report links to artifacts and restates counters; it never holds a fact
/// that is not recoverable from the sidecars and receipts, so deleting and
/// regenerating it loses nothing (acceptance test 15).
fn write_report(output: &Path, receipt: &Receipt) -> Result<Artifact> {
    let dir = output.join("reports");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("idle-report-{}.md", today_stamp()));

    let mut body = String::new();
    body.push_str(&format!("# Idle Canon Worker — {}\n\n", today_stamp()));
    body.push_str(&format!("Run `{}`\n\n", receipt.run_id));
    body.push_str(&format!("- Started: {}\n", receipt.started_at));
    body.push_str(&format!("- Ended: {}\n", receipt.ended_at));
    body.push_str(&format!("- Outcome: {:?}\n", receipt.outcome));
    body.push_str(&format!(
        "- Idle at start: {} minutes\n\n",
        receipt.idle_minutes_at_start
    ));
    body.push_str("| Metric | Count |\n|---|---|\n");
    body.push_str(&format!(
        "| Files inspected | {} |\n",
        receipt.files_inspected
    ));
    body.push_str(&format!("| Files skipped | {} |\n", receipt.files_skipped));
    body.push_str(&format!(
        "| Sidecars created | {} |\n",
        receipt.sidecars_created
    ));
    body.push_str(&format!(
        "| Sidecars versioned | {} |\n",
        receipt.sidecars_versioned
    ));
    body.push_str(&format!(
        "| Sidecars unchanged | {} |\n",
        receipt.sidecars_unchanged
    ));
    body.push_str("\nAuthority produced: `C0_SIDECAR` only. Nothing was admitted, ");
    body.push_str("promoted, moved, renamed, or rewritten.\n");

    if !receipt.notes.is_empty() {
        body.push_str("\n## Notes\n\n");
        for n in &receipt.notes {
            body.push_str(&format!("- {}\n", n));
        }
    }

    // Append rather than replace: several runs can share one day, and an
    // earlier run's record must not be lost to a later one.
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    writeln!(f, "{}", body)?;

    Ok(Artifact {
        sha256: sha256_file(&path)?,
        path: display_path(&path),
    })
}

// ---------------------------------------------------------------------------
// Engine loop
// ---------------------------------------------------------------------------

/// Watch for idle, run once per idle period, stop on user return.
///
/// A run is allowed once per *idle period*, not once per threshold crossing:
/// `armed` only resets after the user comes back, so a machine left alone
/// overnight does one run rather than one every poll.
pub fn engine(cfg: Arc<Mutex<Config>>) -> Result<()> {
    let mut armed = true;
    loop {
        std::thread::sleep(POLL);

        let worker = {
            let guard = cfg.lock().unwrap();
            guard.idle_worker.clone()
        };
        if !worker.enabled {
            continue;
        }

        let idle_min = idle_millis() / 60_000;
        if idle_min < worker.idle_minutes {
            armed = true;
            continue;
        }
        if !armed {
            continue;
        }
        armed = false;

        info!("idle: {} minutes idle, starting bounded run", idle_min);

        // The cancel flag is the single stop signal. The watcher sets it as
        // soon as observed idle time drops, which only happens on real input.
        let cancel = Arc::new(AtomicBool::new(false));
        let watch_cancel = Arc::clone(&cancel);
        let threshold = worker.idle_minutes;
        let stop_on_activity = worker.stop_on_user_activity;
        let watcher = std::thread::spawn(move || {
            while !watch_cancel.load(Ordering::Relaxed) {
                std::thread::sleep(POLL);
                if stop_on_activity && idle_millis() / 60_000 < threshold {
                    watch_cancel.store(true, Ordering::Relaxed);
                    return;
                }
            }
        });

        if RUN_ACTIVE.swap(true, Ordering::SeqCst) {
            cancel.store(true, Ordering::Relaxed);
            watcher.join().ok();
            warn!("idle: a run was already active in this process");
            continue;
        }

        let receipt = run_once(&worker, Arc::clone(&cancel));

        RUN_ACTIVE.store(false, Ordering::SeqCst);
        cancel.store(true, Ordering::Relaxed);
        watcher.join().ok();

        info!(
            "idle: run {} finished {:?} — {} inspected, {} new, {} versioned",
            receipt.run_id,
            receipt.outcome,
            receipt.files_inspected,
            receipt.sidecars_created,
            receipt.sidecars_versioned
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("nerve-idle-test-{}", name));
        std::fs::remove_dir_all(&p).ok();
        std::fs::create_dir_all(&p).unwrap();
        p.canonicalize().unwrap()
    }

    fn cfg_for(src: &Path, out: &Path) -> IdleWorkerConfig {
        IdleWorkerConfig {
            enabled: true,
            approved_roots: vec![src.to_string_lossy().into()],
            output_root: out.to_string_lossy().into(),
            ..Default::default()
        }
    }

    /// Acceptance test 4: with nothing approved, the worker reads nothing.
    #[test]
    fn unconfigured_roots_halt_rather_than_scanning_everything() {
        let cfg = IdleWorkerConfig {
            enabled: true,
            output_root: tmp("noroots").to_string_lossy().into(),
            ..Default::default()
        };
        let r = run_once(&cfg, Arc::new(AtomicBool::new(false)));
        assert_eq!(r.outcome, RunOutcome::SafeHalt);
        assert_eq!(r.files_inspected, 0);
        assert!(r.notes[0].contains("approved_roots"));
    }

    /// Acceptance test 5: output cannot resolve inside a source directory.
    #[test]
    fn output_inside_a_source_root_is_refused() {
        let src = tmp("contained-src");
        let out = src.join("out");
        std::fs::create_dir_all(&out).unwrap();
        let r = run_once(&cfg_for(&src, &out), Arc::new(AtomicBool::new(false)));
        assert_eq!(r.outcome, RunOutcome::SafeHalt);
        assert!(r.notes[0].contains("lies inside"));
    }

    /// Acceptance test 13: rerunning unchanged input is idempotent by hash.
    /// Acceptance test 14: a changed source versions rather than overwrites.
    #[test]
    fn capture_is_idempotent_and_versions_on_change() {
        let src = tmp("capture-src");
        let out = tmp("capture-out");
        let file = src.join("paper.md");
        std::fs::write(&file, "first").unwrap();
        let cfg = cfg_for(&src, &out);

        let a = run_once(&cfg, Arc::new(AtomicBool::new(false)));
        assert_eq!(a.sidecars_created, 1);

        let b = run_once(&cfg, Arc::new(AtomicBool::new(false)));
        assert_eq!(b.sidecars_created, 0);
        assert_eq!(b.sidecars_unchanged, 1);

        std::fs::write(&file, "second").unwrap();
        let c = run_once(&cfg, Arc::new(AtomicBool::new(false)));
        assert_eq!(c.sidecars_versioned, 1);

        let entry = std::fs::read_dir(out.join("sidecars"))
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        let sc: Sidecar = serde_json::from_str(&std::fs::read_to_string(entry).unwrap()).unwrap();
        assert_eq!(sc.versions.len(), 2, "the first version must survive");
        assert_eq!(sc.versions[0].source_sha256, sha256_bytes(b"first"));
    }

    /// Acceptance test 7: the worker cannot emit ADMITTED or C3_CANONICAL.
    #[test]
    fn captures_never_carry_authority() {
        let src = tmp("authority-src");
        let out = tmp("authority-out");
        std::fs::write(src.join("a.md"), "x").unwrap();
        run_once(&cfg_for(&src, &out), Arc::new(AtomicBool::new(false)));

        for e in std::fs::read_dir(out.join("sidecars")).unwrap() {
            let sc: Sidecar =
                serde_json::from_str(&std::fs::read_to_string(e.unwrap().path()).unwrap()).unwrap();
            assert_eq!(sc.canonization_class, "C0_SIDECAR");
            assert!(!sc.authority.candidate);
            assert!(!sc.authority.admitted);
            assert!(!sc.authority.canonical);
            assert!(sc.classification.human_ruling.is_none());
        }
    }

    /// Acceptance tests 2 and 12: a cancelled run reports `UserReturned`
    /// rather than failing, and whatever it completed stays on disk.
    #[test]
    fn cancellation_is_a_normal_stop_not_an_error() {
        let src = tmp("cancel-src");
        let out = tmp("cancel-out");
        std::fs::write(src.join("a.md"), "x").unwrap();
        let r = run_once(&cfg_for(&src, &out), Arc::new(AtomicBool::new(true)));
        assert_eq!(r.outcome, RunOutcome::UserReturned);
    }

    /// Acceptance test 3: a second run cannot proceed while a lock is held.
    #[test]
    fn concurrent_runs_are_refused() {
        let src = tmp("lock-src");
        let out = tmp("lock-out");
        std::fs::write(src.join("a.md"), "x").unwrap();
        let cfg = cfg_for(&src, &out);
        let bounds = resolve_bounds(&cfg).unwrap();
        let _held = RunLock::acquire(&bounds.output, "holder").unwrap();

        let r = run_once(&cfg, Arc::new(AtomicBool::new(false)));
        assert_eq!(r.outcome, RunOutcome::SafeHalt);
        assert!(r.notes[0].contains("holds the lock"));
    }

    /// Acceptance test 8/9 in miniature: nothing the worker writes touches the
    /// source file, so its bytes are unchanged after a run.
    #[test]
    fn source_files_are_never_modified() {
        let src = tmp("readonly-src");
        let out = tmp("readonly-out");
        let file = src.join("paper.md");
        std::fs::write(&file, "untouched").unwrap();
        let before = sha256_file(&file).unwrap();

        run_once(&cfg_for(&src, &out), Arc::new(AtomicBool::new(false)));

        assert_eq!(sha256_file(&file).unwrap(), before);
    }

    /// Manual smoke check against the live corpus. Ignored by default because
    /// it depends on a path outside the repository; run with
    /// `cargo test idle::tests::live -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn live_corpus_smoke() {
        let src = PathBuf::from(r"D:\GitHub\Faith-through-physics-atoms\axioms");
        if !src.exists() {
            eprintln!("corpus not present; nothing to check");
            return;
        }
        let out = tmp("live-out");
        let r = run_once(&cfg_for(&src, &out), Arc::new(AtomicBool::new(false)));
        eprintln!(
            "{:?}: {} inspected, {} created, {} skipped",
            r.outcome, r.files_inspected, r.sidecars_created, r.files_skipped
        );
        for n in r.notes.iter().take(5) {
            eprintln!("  note: {}", n);
        }
        assert_eq!(r.outcome, RunOutcome::Complete);
        assert!(r.files_inspected > 0, "the corpus should yield files");
    }

    #[test]
    fn civil_date_conversion_is_correct() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(19_000), (2022, 1, 8));
    }
}
