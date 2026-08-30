//! Stratum action runner.
//!
//! Stratum already defines a clean action contract: every module in
//! `08_actions/` exposes `process(data: dict) -> str`, where `data` carries
//! `selection`, `clipboard`, and `timestamp`. Nerve runs those modules
//! directly rather than reimplementing them, so `04_config/actions.json`
//! stays the single source of truth for what actions exist.
//!
//! Actions run in a short-lived Python subprocess. Nerve does not import
//! Stratum's registry: the registry configures logging and caches processors
//! for a long-lived popup process, neither of which applies here.

use crate::config::ScriptsConfig;
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tracing::{info, warn};

/// One entry from Stratum's `04_config/actions.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptAction {
    pub id: String,
    pub name: String,
    pub module: String,
    #[serde(default)]
    pub description: String,
}

/// Read the action catalogue from Stratum's own config file.
///
/// Returns an empty list rather than an error when Stratum is not installed —
/// a missing optional integration should not break the rest of the app.
pub fn list_actions(cfg: &ScriptsConfig) -> Vec<ScriptAction> {
    if !cfg.enabled {
        return Vec::new();
    }
    match load_actions(cfg) {
        Ok(actions) => actions,
        Err(e) => {
            warn!("scripts: could not load Stratum actions: {}", e);
            Vec::new()
        }
    }
}

fn load_actions(cfg: &ScriptsConfig) -> Result<Vec<ScriptAction>> {
    let path = PathBuf::from(&cfg.stratum_root)
        .join("04_config")
        .join("actions.json");
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path.display()))?;
    let actions: Vec<ScriptAction> =
        serde_json::from_str(&raw).with_context(|| format!("parsing {}", path.display()))?;
    Ok(actions)
}

/// Run one Stratum action against a selection and return its text result.
pub fn run_action(
    cfg: &ScriptsConfig,
    action_id: &str,
    selection: &str,
    clipboard: &str,
) -> Result<String> {
    if !cfg.enabled {
        bail!("Stratum script actions are disabled in settings.");
    }

    let actions = load_actions(cfg)?;
    let action = actions
        .iter()
        .find(|a| a.id == action_id)
        .ok_or_else(|| anyhow!("Unknown Stratum action '{}'", action_id))?;

    let root = PathBuf::from(&cfg.stratum_root);
    let module_path = root.join("08_actions").join(format!("{}.py", action.module));
    if !module_path.exists() {
        bail!(
            "Action module not found for '{}': {}",
            action_id,
            module_path.display()
        );
    }

    let python = resolve_python(cfg)?;

    // The payload goes in on stdin as JSON. Command-line arguments mangle
    // newlines and quoting, which is the same reason Stratum's own AHK glue
    // passes the selection through a temp file.
    let payload = serde_json::json!({
        "selection": selection,
        "clipboard": clipboard,
        "timestamp": iso_now(),
    });

    info!("scripts: running Stratum action '{}'", action_id);

    let mut child = new_hidden_command(&python)
        .arg("-c")
        .arg(RUNNER)
        .arg(module_path.to_string_lossy().to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("launching {}", python.display()))?;

    {
        use std::io::Write;
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow!("could not open stdin for the action process"))?;
        stdin.write_all(payload.to_string().as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = stderr.trim();
        // Surface the located reason, not a bare failure.
        bail!(
            "Action '{}' failed: {}",
            action_id,
            if detail.is_empty() {
                "no error output".to_string()
            } else {
                last_lines(detail, 6)
            }
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim_end().to_string())
}

/// Loads the action module by path and runs `process(data)`.
///
/// Kept deliberately small: it imports one file, calls one function, and
/// writes the result to stdout. Anything else belongs in the action itself.
const RUNNER: &str = r#"
import importlib.util, json, sys

module_path = sys.argv[1]
payload = json.loads(sys.stdin.read() or "{}")

spec = importlib.util.spec_from_file_location("nerve_stratum_action", module_path)
if spec is None or spec.loader is None:
    raise SystemExit("Could not import action module: " + module_path)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)

process = getattr(module, "process", None)
if not callable(process):
    raise SystemExit("Action module must expose process(data: dict) -> str: " + module_path)

result = process(payload)
sys.stdout.write("" if result is None else str(result))
"#;

/// Find a usable Python. An explicit configured path wins; otherwise fall back
/// to whatever is on PATH.
fn resolve_python(cfg: &ScriptsConfig) -> Result<PathBuf> {
    if !cfg.python_exe.is_empty() {
        let path = PathBuf::from(&cfg.python_exe);
        if path.exists() {
            return Ok(path);
        }
        warn!(
            "scripts: configured python '{}' not found, falling back to PATH",
            cfg.python_exe
        );
    }
    Ok(PathBuf::from("python.exe"))
}

/// Spawn without flashing a console window.
fn new_hidden_command(program: &Path) -> Command {
    let mut cmd = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}

fn last_lines(text: &str, count: usize) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines.len().saturating_sub(count);
    lines[start..].join(" | ")
}

fn iso_now() -> String {
    // Stratum's registry only defaults this field when absent and never parses
    // it, so seconds-since-epoch precision is sufficient and keeps Nerve free
    // of a date-formatting dependency.
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("unix:{}", secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cfg() -> ScriptsConfig {
        ScriptsConfig::default()
    }

    #[test]
    fn disabled_scripts_refuse_to_run() {
        let cfg = ScriptsConfig {
            enabled: false,
            ..ScriptsConfig::default()
        };
        let err = run_action(&cfg, "grammar_fix", "x", "").unwrap_err();
        assert!(err.to_string().contains("disabled"));
    }

    #[test]
    fn missing_stratum_yields_no_actions_not_a_panic() {
        let cfg = ScriptsConfig {
            enabled: true,
            stratum_root: r"D:\definitely\not\here".into(),
            ..ScriptsConfig::default()
        };
        assert!(list_actions(&cfg).is_empty());
    }

    /// Runs only where Stratum is actually installed.
    #[test]
    fn runs_a_real_stratum_action_when_present() {
        let cfg = test_cfg();
        let actions = list_actions(&cfg);
        if actions.is_empty() {
            eprintln!("skipping: Stratum not installed at {}", cfg.stratum_root);
            return;
        }
        assert!(actions.iter().any(|a| a.id == "grammar_fix"));

        // grammar_fix collapses runs of whitespace and pulls punctuation back
        // onto the preceding word. It strips a single space before punctuation,
        // not an arbitrary run, so the input here is spaced accordingly.
        let out = run_action(&cfg, "grammar_fix", "hello ,  world .", "").unwrap();
        assert_eq!(out, "hello, world.");
    }

    /// Stratum actions read `selection` first, then `clipboard`. Nerve used to
    /// pass an empty string for clipboard, so an action run with nothing
    /// selected silently received no input at all.
    #[test]
    fn clipboard_is_used_when_nothing_is_selected() {
        let cfg = test_cfg();
        if list_actions(&cfg).is_empty() {
            eprintln!("skipping: Stratum not installed at {}", cfg.stratum_root);
            return;
        }
        let out = run_action(&cfg, "grammar_fix", "", "grace ,  is  prior .").unwrap();
        assert_eq!(out, "grace, is prior.");
    }

    #[test]
    fn unknown_action_names_itself_in_the_error() {
        let cfg = test_cfg();
        if list_actions(&cfg).is_empty() {
            return;
        }
        let err = run_action(&cfg, "no_such_action", "x", "").unwrap_err();
        assert!(err.to_string().contains("no_such_action"));
    }
}
