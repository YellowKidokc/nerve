//! Narrow client for the local Canonical Content OS service.
//!
//! Deliberate boundary: Nerve captures and displays. It POSTs immutable
//! candidates and GETs the canon-session agenda. It has no promotion route,
//! holds no promotion credential, and must never gain one — promotion is a
//! separate authenticated human act performed by the canon service itself.
//!
//! Failure is never coerced into success. If the service is unreachable, the
//! capture is preserved locally and reported as pending, exactly as the Ollama
//! review layer requires of its own failure paths.

use crate::config::CanonConfig;
use crate::selection::Capture;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{info, warn};

/// What Nerve sends when the user picks a semantic node action.
///
/// This mirrors `canon-engine`'s `CandidateInput` field for field. The engine
/// ignores unknown keys silently, so anything not named here would be dropped
/// without warning — everything Nerve knows about the capture therefore goes
/// inside `atom`, which the engine treats as opaque and hashes whole.
///
/// This is a *proposal*, not a record. The engine assigns identity, hashes,
/// stages, and runs deterministic checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateRequest {
    /// Structured payload. Opaque to the engine, meaningful to the canon graph.
    pub atom: serde_json::Value,
    /// The atom this candidate proposes a version of, when the user named one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Ids this candidate declares it depends on. Missing targets open a
    /// conflict at the engine; they never block intake.
    #[serde(default)]
    pub depends_on: Vec<String>,
}

impl CandidateRequest {
    pub fn from_capture(
        capture: &Capture,
        node_type: &str,
        capsule: serde_json::Value,
        author: &str,
    ) -> Self {
        // Capture-time provenance travels inside the atom. The engine computes
        // its own content hash over the whole payload; ours is recorded as the
        // capture-time address so the two can be compared later.
        let atom = serde_json::json!({
            "schema": "nerve-capture/v0.1",
            "node_type": node_type,
            "text": capture.text,
            "capsule": capsule,
            "capture": {
                "kinds": capture.kinds,
                "source_window": capture.source_window,
                "captured_at": capture.captured_at,
                "capture_hash": capture.content_hash,
                "client": format!("nerve/{}", env!("CARGO_PKG_VERSION")),
            },
        });

        let author = author.trim();

        Self {
            atom,
            atom_id: None,
            author: if author.is_empty() {
                None
            } else {
                Some(author.to_string())
            },
            source_uri: None,
            note: if capture.source_window.is_empty() {
                None
            } else {
                Some(format!("captured from {}", capture.source_window))
            },
            depends_on: Vec::new(),
        }
    }
}

/// Outcome of trying to stage a candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateOutcome {
    /// "staged" when the service accepted it, "pending_local" when the service
    /// was unreachable and the candidate was spooled to disk instead.
    pub state: String,
    /// Service-assigned candidate ID, when we got one.
    #[serde(default)]
    pub candidate_id: String,
    /// Human-readable detail. On failure this carries a located reason —
    /// never a bare FAIL.
    pub detail: String,
    /// Raw service response, passed through for the capsule UI to render.
    #[serde(default)]
    pub response: serde_json::Value,
    /// Deterministic verdict from the engine: pass / uncertain / fail.
    #[serde(default)]
    pub verdict: String,
    /// Candidate status the engine assigned (reviewed / flagged / unreviewed).
    #[serde(default)]
    pub status: String,
    /// Located reasons — each names a check, a trigger, and a smallest fix.
    /// A bare failure is never sufficient.
    #[serde(default)]
    pub reasons: Vec<serde_json::Value>,
    /// Conflict ids opened by this candidate.
    #[serde(default)]
    pub conflicts: Vec<String>,
}

/// Stage a candidate with the local canon service.
///
/// When the service is off or unreachable the capture is *not* lost: it is
/// appended to a local spool file and reported as `pending_local`, so the
/// user can see exactly what happened and recover it.
pub async fn submit_candidate(
    cfg: &CanonConfig,
    request: &CandidateRequest,
) -> CandidateOutcome {
    if !cfg.enabled {
        let detail = spool(request)
            .map(|p| {
                format!(
                    "Canon service disabled in settings. Candidate spooled to {}",
                    p.display()
                )
            })
            .unwrap_or_else(|e| format!("Canon service disabled, and spooling failed: {}", e));
        return CandidateOutcome {
            state: "pending_local".into(),
            candidate_id: String::new(),
            detail,
            response: serde_json::Value::Null,
            verdict: String::new(),
            status: String::new(),
            reasons: Vec::new(),
            conflicts: Vec::new(),
        };
    }

    match post_candidate(cfg, request).await {
        Ok(value) => {
            let candidate_id = value
                .get("candidate_id")
                .or_else(|| value.get("id"))
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let verdict = str_field(&value, "verdict");
            let status = str_field(&value, "status");
            let reasons = arr_field(&value, "reasons");
            let conflicts: Vec<String> = value
                .get("conflicts_opened")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|c| c.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            info!(
                "canon: candidate '{}' staged, verdict={}, conflicts={}",
                candidate_id,
                verdict,
                conflicts.len()
            );

            CandidateOutcome {
                state: "staged".into(),
                candidate_id,
                detail: value
                    .get("note")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Accepted and staged. This is not canon.")
                    .to_string(),
                response: value,
                verdict,
                status,
                reasons,
                conflicts,
            }
        }
        Err(e) => {
            warn!("canon: submit failed: {}", e);
            let detail = match spool(request) {
                Ok(path) => format!(
                    "Canon service unreachable ({}). Candidate preserved at {}",
                    e,
                    path.display()
                ),
                Err(spool_err) => format!(
                    "Canon service unreachable ({}), and spooling failed: {}",
                    e, spool_err
                ),
            };
            CandidateOutcome {
                state: "pending_local".into(),
                candidate_id: String::new(),
                detail,
                response: serde_json::Value::Null,
                verdict: String::new(),
                status: String::new(),
                reasons: Vec::new(),
                conflicts: Vec::new(),
            }
        }
    }
}

fn str_field(value: &serde_json::Value, key: &str) -> String {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}

fn arr_field(value: &serde_json::Value, key: &str) -> Vec<serde_json::Value> {
    value
        .get(key)
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
}

async fn post_candidate(
    cfg: &CanonConfig,
    request: &CandidateRequest,
) -> Result<serde_json::Value> {
    let url = format!(
        "{}{}",
        cfg.base_url.trim_end_matches('/'),
        &cfg.candidate_route
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(20))
        .build()?;

    let resp = client.post(&url).json(request).send().await?;
    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        bail!(
            "canon service returned {}: {}",
            status,
            &body[..body.len().min(400)]
        );
    }

    Ok(serde_json::from_str(&body).unwrap_or(serde_json::Value::String(body)))
}

/// Fetch the canon-session agenda for display.
pub async fn fetch_agenda(cfg: &CanonConfig) -> Result<serde_json::Value> {
    if !cfg.enabled {
        bail!("Canon service is disabled in settings.");
    }
    let url = format!(
        "{}{}",
        cfg.base_url.trim_end_matches('/'),
        &cfg.agenda_route
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(20))
        .build()?;

    let resp = client.get(&url).send().await?;
    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        bail!(
            "canon service returned {}: {}",
            status,
            &body[..body.len().min(400)]
        );
    }

    Ok(serde_json::from_str(&body)?)
}

/// Local spool for captures taken while the service is down. Append-only
/// JSONL so nothing is overwritten and everything stays recoverable.
pub fn spool_path() -> PathBuf {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("clipsync-agent");
    std::fs::create_dir_all(&dir).ok();
    dir.join("pending-candidates.jsonl")
}

fn spool(request: &CandidateRequest) -> Result<PathBuf> {
    use std::io::Write;

    let path = spool_path();
    let line = serde_json::to_string(request)?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    writeln!(file, "{}", line)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_capture() -> Capture {
        Capture {
            text: "Entropy is the arrow of time".into(),
            kinds: vec!["claimish".into(), "any".into()],
            len: 28,
            cursor_x: 0,
            cursor_y: 0,
            captured_at: 1000,
            source_window: "Obsidian".into(),
            content_hash: "fnv1a64:dead".into(),
            // Derived recognition metadata, deliberately absent from the atom.
            ..Default::default()
        }
    }

    #[test]
    fn candidate_matches_the_engine_field_contract() {
        let req = CandidateRequest::from_capture(
            &sample_capture(),
            "claim",
            serde_json::json!({ "plain_statement": "time has a direction" }),
            "david",
        );
        let wire = serde_json::to_value(&req).unwrap();

        // The engine reads exactly these top-level keys.
        assert!(wire.get("atom").is_some());
        assert_eq!(wire["author"], "david");

        // Everything Nerve knows survives inside the opaque atom, because the
        // engine silently ignores unknown top-level keys.
        assert_eq!(wire["atom"]["node_type"], "claim");
        assert_eq!(wire["atom"]["text"], "Entropy is the arrow of time");
        assert_eq!(wire["atom"]["capsule"]["plain_statement"], "time has a direction");
        assert_eq!(wire["atom"]["capture"]["source_window"], "Obsidian");
        assert_eq!(wire["atom"]["capture"]["capture_hash"], "fnv1a64:dead");
    }

    #[test]
    fn empty_author_is_omitted_rather_than_sent_blank() {
        let req = CandidateRequest::from_capture(
            &sample_capture(),
            "claim",
            serde_json::Value::Null,
            "   ",
        );
        let wire = serde_json::to_value(&req).unwrap();
        assert!(wire.get("author").is_none());
    }

    #[tokio::test]
    async fn disabled_service_preserves_the_capture() {
        let cfg = CanonConfig {
            enabled: false,
            ..Default::default()
        };
        let req = CandidateRequest::from_capture(
            &Capture {
                text: "test".into(),
                ..Default::default()
            },
            "claim",
            serde_json::Value::Null,
            "tester",
        );
        let outcome = submit_candidate(&cfg, &req).await;
        // Never silently dropped, and never reported as staged.
        assert_eq!(outcome.state, "pending_local");
        assert!(outcome.candidate_id.is_empty());
        assert!(outcome.verdict.is_empty());
    }

    /// The atom is built field by field, never by serializing `Capture`.
    ///
    /// This is what keeps derived recognition metadata — domains, confidence,
    /// features — out of canonical identity. The engine hashes the atom whole,
    /// so a field added to `Capture` and silently forwarded here would change
    /// the digest of every future capture and make an improved classifier look
    /// like a different atom. Locking the key set turns that into a failing
    /// test rather than a silent change.
    #[test]
    fn derived_recognition_never_reaches_the_atom() {
        let mut capture = sample_capture();
        capture.domains = vec!["physics".into()];
        capture.confidence = Some(0.88);
        capture.features = vec!["contains_assertion".into()];

        let req = CandidateRequest::from_capture(&capture, "claim", serde_json::Value::Null, "d");
        let wire = serde_json::to_value(&req).unwrap();
        let block = wire["atom"]["capture"].as_object().unwrap();

        let mut keys: Vec<&str> = block.keys().map(|k| k.as_str()).collect();
        keys.sort();
        assert_eq!(
            keys,
            ["capture_hash", "captured_at", "client", "kinds", "source_window"],
            "the atom's capture block changed shape"
        );

        // The address depends on the text alone, whatever the classifier said.
        assert_eq!(wire["atom"]["capture"]["capture_hash"], "fnv1a64:dead");
    }

}
