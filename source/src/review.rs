//! Review queue for work produced while the user was away.
//!
//! The idle worker writes `C0_SIDECAR` records to disk and stops. Without a
//! surface, reviewing them would mean finding the folder — so this module
//! loads the queue, validates records, and performs the one bulk transition
//! that is safe to do in bulk.
//!
//! The tier boundary is the whole point:
//!
//! * **Accept to Candidate** (`C0`/`C1` → `C2`) is a bulk action. It says "this
//!   is real and structurally sound", nothing more.
//! * **Admit to Canon** (`C2` → `C3`) is never reachable from here. Admission
//!   is a signed, individual, deliberate act performed by the canon engine
//!   behind its own bearer token.
//!
//! Nothing in this file writes `C3_CANONICAL`, and there is no code path from a
//! bulk gesture to one.

use crate::idle::Sidecar;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The tier a bulk accept may move a record to. Declared as a constant so a
/// search for the string finds exactly one producer.
const CANDIDATE_CLASS: &str = "C2_CANDIDATE";

/// One row in the review table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewItem {
    pub sidecar_id: String,
    pub canonization_class: String,
    pub status: String,
    pub source_path: String,
    pub captured_at: String,
    pub source_bytes: u64,
    /// Recognizer output, carried through so the table can group and filter
    /// without re-reading the source files.
    #[serde(default)]
    pub kinds: Vec<String>,
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(default)]
    pub confidence: Option<f32>,
    /// Empty when the record would pass a bulk accept. Populated with located
    /// reasons otherwise — never a bare failure.
    #[serde(default)]
    pub blocking_reasons: Vec<String>,
}

impl ReviewItem {
    pub fn ready(&self) -> bool {
        self.blocking_reasons.is_empty()
    }
}

/// What a queue load found.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Queue {
    pub items: Vec<ReviewItem>,
    pub total: usize,
    pub ready: usize,
    pub blocked: usize,
}

/// Load every record still awaiting review from an idle output root.
///
/// Records already accepted are excluded rather than shown greyed out: at 170
/// items the queue has to shrink as it is worked, or progress is invisible.
pub fn load(output_root: &Path) -> Result<Queue> {
    let dir = output_root.join("sidecars");
    let mut items = Vec::new();

    if dir.is_dir() {
        for entry in std::fs::read_dir(&dir)?.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            // A malformed sidecar is reported as a blocked row rather than
            // skipped, so a parse failure cannot quietly shrink the queue.
            match serde_json::from_str::<Sidecar>(&text) {
                Ok(sc) if is_pending(&sc) => items.push(to_item(sc)),
                Ok(_) => {}
                Err(e) => items.push(unreadable(&path, &e.to_string())),
            }
        }
    }

    // Least confident first: the rows needing judgement surface at the top,
    // and the obvious remainder can be swept in one gesture.
    items.sort_by(|a, b| {
        a.confidence
            .unwrap_or(0.0)
            .partial_cmp(&b.confidence.unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.sidecar_id.cmp(&b.sidecar_id))
    });

    let ready = items.iter().filter(|i| i.ready()).count();
    Ok(Queue {
        total: items.len(),
        ready,
        blocked: items.len() - ready,
        items,
    })
}

/// A record is pending while it has not yet been accepted to candidate.
fn is_pending(sc: &Sidecar) -> bool {
    sc.canonization_class != CANDIDATE_CLASS && !sc.authority.candidate
}

fn to_item(sc: Sidecar) -> ReviewItem {
    let blocking_reasons = validate(&sc);
    ReviewItem {
        sidecar_id: sc.sidecar_id,
        canonization_class: sc.canonization_class,
        status: sc.status,
        source_path: sc.capture.source_path,
        captured_at: sc.capture.captured_at,
        source_bytes: sc.capture.source_bytes,
        kinds: Vec::new(),
        domains: Vec::new(),
        confidence: None,
        blocking_reasons,
    }
}

fn unreadable(path: &Path, err: &str) -> ReviewItem {
    ReviewItem {
        sidecar_id: path.file_stem().unwrap_or_default().to_string_lossy().into(),
        canonization_class: "UNKNOWN".into(),
        status: "UNREADABLE".into(),
        source_path: path.to_string_lossy().replace('\\', "/"),
        captured_at: String::new(),
        source_bytes: 0,
        kinds: Vec::new(),
        domains: Vec::new(),
        confidence: None,
        blocking_reasons: vec![format!("sidecar could not be parsed: {}", err)],
    }
}

/// Structural checks a record must pass before a bulk accept may move it.
///
/// These are deliberately shallow. Bulk accept asserts that a record is real
/// and well-formed — not that its content is correct, which is what the later
/// ruling is for.
fn validate(sc: &Sidecar) -> Vec<String> {
    let mut reasons = Vec::new();

    if sc.capture.source_sha256.is_empty() {
        reasons.push("no source hash recorded; the record cannot be addressed".into());
    }
    if sc.capture.source_path.trim().is_empty() {
        reasons.push("no source path recorded".into());
    }
    if sc.versions.is_empty() {
        reasons.push("no version frozen; run ingestion before accepting".into());
    }
    // An already-ruled record must not be swept back through intake.
    if sc.classification.human_ruling.is_some() {
        reasons.push("a human ruling already exists; handle this record individually".into());
    }
    if sc.authority.admitted || sc.authority.canonical {
        reasons.push("already admitted; bulk accept does not apply".into());
    }

    reasons
}

/// Outcome of a bulk accept.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcceptResult {
    pub accepted: Vec<String>,
    /// Id paired with the reason it was refused.
    pub refused: Vec<(String, String)>,
}

/// Move validated records to `C2_CANDIDATE`.
///
/// Refusals are returned, never swallowed: at this volume a silently skipped
/// record would simply vanish from the user's attention.
///
/// This sets `authority.candidate` and nothing else. `admitted` and
/// `canonical` are untouched — reaching them requires the canon engine's
/// separate signed act.
pub fn accept_to_candidate(output_root: &Path, ids: &[String]) -> Result<AcceptResult> {
    let dir = output_root.join("sidecars");
    let mut result = AcceptResult::default();

    for id in ids {
        let path = dir.join(format!("{}.json", id));
        if !path.exists() {
            result.refused.push((id.clone(), "no such sidecar".into()));
            continue;
        }

        let mut sc: Sidecar = match std::fs::read_to_string(&path)
            .map_err(|e| e.to_string())
            .and_then(|t| serde_json::from_str(&t).map_err(|e| e.to_string()))
        {
            Ok(sc) => sc,
            Err(e) => {
                result.refused.push((id.clone(), e));
                continue;
            }
        };

        let reasons = validate(&sc);
        if !reasons.is_empty() {
            result.refused.push((id.clone(), reasons.join("; ")));
            continue;
        }

        sc.canonization_class = CANDIDATE_CLASS.into();
        sc.status = "ACCEPTED_TO_CANDIDATE".into();
        sc.authority.candidate = true;

        let tmp = path.with_extension("tmp");
        if let Err(e) = std::fs::write(&tmp, serde_json::to_string_pretty(&sc)?)
            .and_then(|_| std::fs::rename(&tmp, &path))
        {
            result.refused.push((id.clone(), e.to_string()));
            continue;
        }
        result.accepted.push(id.clone());
    }

    Ok(result)
}

/// Where the review UI reads from.
pub fn output_root(cfg: &crate::config::Config) -> PathBuf {
    PathBuf::from(&cfg.idle_worker.output_root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::idle::{Authority, Capture, Classification, VersionEntry};

    fn tmp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("nerve-review-{}", name));
        std::fs::remove_dir_all(&p).ok();
        std::fs::create_dir_all(p.join("sidecars")).unwrap();
        p
    }

    fn write(root: &Path, sc: &Sidecar) {
        std::fs::write(
            root.join("sidecars").join(format!("{}.json", sc.sidecar_id)),
            serde_json::to_string_pretty(sc).unwrap(),
        )
        .unwrap();
    }

    fn sidecar(id: &str) -> Sidecar {
        Sidecar {
            sidecar_id: id.into(),
            canonization_class: "C0_SIDECAR".into(),
            status: "CAPTURED_UNREVIEWED".into(),
            capture: Capture {
                method: "IDLE_DETERMINISTIC_SCAN".into(),
                source_path: "D:/corpus/a.md".into(),
                captured_at: "2026-08-29T00:00:00Z".into(),
                captured_by: "david".into(),
                source_sha256: "abc123".into(),
                source_bytes: 42,
            },
            classification: Classification::default(),
            authority: Authority::default(),
            versions: vec![VersionEntry {
                version: 1,
                source_sha256: "abc123".into(),
                observed_at: "2026-08-29T00:00:00Z".into(),
            }],
        }
    }

    #[test]
    fn a_clean_record_is_ready() {
        let root = tmp("clean");
        write(&root, &sidecar("SC-1"));
        let q = load(&root).unwrap();
        assert_eq!(q.total, 1);
        assert_eq!(q.ready, 1);
        assert_eq!(q.blocked, 0);
    }

    #[test]
    fn bulk_accept_stops_at_candidate() {
        let root = tmp("accept");
        write(&root, &sidecar("SC-1"));

        let r = accept_to_candidate(&root, &["SC-1".into()]).unwrap();
        assert_eq!(r.accepted, vec!["SC-1".to_string()]);

        let sc: Sidecar = serde_json::from_str(
            &std::fs::read_to_string(root.join("sidecars/SC-1.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(sc.canonization_class, "C2_CANDIDATE");
        assert!(sc.authority.candidate);
        // The tier boundary: bulk accept never reaches admission.
        assert!(!sc.authority.admitted, "bulk accept must not admit");
        assert!(!sc.authority.canonical, "bulk accept must not canonize");
    }

    #[test]
    fn accepted_records_leave_the_queue() {
        let root = tmp("shrink");
        write(&root, &sidecar("SC-1"));
        accept_to_candidate(&root, &["SC-1".into()]).unwrap();
        assert_eq!(load(&root).unwrap().total, 0);
    }

    #[test]
    fn a_record_without_a_frozen_version_is_blocked_with_a_reason() {
        let root = tmp("noversion");
        let mut sc = sidecar("SC-2");
        sc.versions.clear();
        write(&root, &sc);

        let q = load(&root).unwrap();
        assert_eq!(q.blocked, 1);
        assert!(q.items[0].blocking_reasons[0].contains("no version frozen"));

        // And it stays refused rather than slipping through the bulk path.
        let r = accept_to_candidate(&root, &["SC-2".into()]).unwrap();
        assert!(r.accepted.is_empty());
        assert_eq!(r.refused.len(), 1);
    }

    #[test]
    fn an_already_ruled_record_is_not_swept_through_intake() {
        let root = tmp("ruled");
        let mut sc = sidecar("SC-3");
        sc.classification.human_ruling = Some("CR-2026-004".into());
        write(&root, &sc);

        let r = accept_to_candidate(&root, &["SC-3".into()]).unwrap();
        assert!(r.accepted.is_empty());
        assert!(r.refused[0].1.contains("human ruling"));
    }

    #[test]
    fn an_unreadable_sidecar_is_shown_not_skipped() {
        let root = tmp("broken");
        std::fs::write(root.join("sidecars/SC-4.json"), "{ not json").unwrap();
        let q = load(&root).unwrap();
        assert_eq!(q.total, 1);
        assert_eq!(q.blocked, 1);
        assert_eq!(q.items[0].status, "UNREADABLE");
    }

    #[test]
    fn refusals_are_reported_never_swallowed() {
        let root = tmp("missing");
        let r = accept_to_candidate(&root, &["SC-nope".into()]).unwrap();
        assert_eq!(r.refused.len(), 1);
        assert!(r.refused[0].1.contains("no such sidecar"));
    }

    #[test]
    fn an_empty_root_is_an_empty_queue_not_an_error() {
        let root = tmp("empty");
        let q = load(&root).unwrap();
        assert_eq!(q.total, 0);
    }
}
