use crate::config::{AiProvider, AiWorkflow};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

/// A single message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String, // "user", "assistant", "system"
    pub content: String,
}

/// Expand the placeholders a prompt template may use.
///
/// `{{input}}` is whatever the caller passed — selection or clipboard,
/// depending on how the action was triggered. `{{selection}}` and
/// `{{clipboard}}` name the two sources explicitly, so one prompt can refer to
/// both at once instead of collapsing them into a single slot.
///
/// A placeholder whose source is empty expands to an empty string rather than
/// being left as literal `{{clipboard}}` text, which would otherwise reach the
/// model as an instruction-looking artefact.
fn render_template(template: &str, user_input: &str) -> String {
    let selection = crate::selection::last_capture().text;
    let clipboard = crate::clipboard::current_text().unwrap_or_default();

    template
        .replace("{{input}}", user_input)
        .replace("{{selection}}", &selection)
        .replace("{{clipboard}}", &clipboard)
}

/// Send a message to an AI provider and get a response
pub async fn chat(
    provider: &AiProvider,
    workflow: Option<&AiWorkflow>,
    messages: &[ChatMessage],
    user_input: &str,
) -> Result<String> {
    let system_prompt = workflow
        .map(|w| w.system_prompt.as_str())
        .unwrap_or("You are a helpful assistant.");

    let max_tokens = workflow.map(|w| w.max_tokens).unwrap_or(4096);
    let temperature = workflow.map(|w| w.temperature).unwrap_or(0.7);

    // Apply user_template if workflow has one
    let final_input = if let Some(wf) = workflow {
        if !wf.user_template.is_empty() {
            render_template(&wf.user_template, user_input)
        } else {
            user_input.to_string()
        }
    } else {
        user_input.to_string()
    };

    match provider.provider_type.as_str() {
        "claude" => {
            chat_claude(provider, system_prompt, messages, &final_input, max_tokens, temperature)
                .await
        }
        // Ollama and LM Studio both expose an OpenAI-compatible route, so they
        // reuse this path rather than each getting a bespoke client. They differ
        // only in default endpoint, which `default_endpoint` resolves.
        "openai" | "local" | "ollama" => {
            chat_openai_compat(
                provider,
                system_prompt,
                messages,
                &final_input,
                max_tokens,
                temperature,
            )
            .await
        }
        other => bail!("Unknown provider type: {}", other),
    }
}

/// Claude (Anthropic) API
async fn chat_claude(
    provider: &AiProvider,
    system_prompt: &str,
    history: &[ChatMessage],
    user_input: &str,
    max_tokens: u32,
    temperature: f64,
) -> Result<String> {
    if provider.api_key.is_empty() {
        bail!("Claude API key not configured. Go to Settings to add it.");
    }

    let endpoint = if provider.endpoint.is_empty() {
        "https://api.anthropic.com/v1/messages"
    } else {
        &provider.endpoint
    };

    let model = if provider.model.is_empty() {
        "claude-sonnet-4-20250514"
    } else {
        &provider.model
    };

    // Build messages array for Claude API
    let mut api_messages: Vec<serde_json::Value> = history
        .iter()
        .filter(|m| m.role != "system")
        .map(|m| {
            serde_json::json!({
                "role": m.role,
                "content": m.content,
            })
        })
        .collect();

    api_messages.push(serde_json::json!({
        "role": "user",
        "content": user_input,
    }));

    let body = serde_json::json!({
        "model": model,
        "max_tokens": max_tokens,
        "temperature": temperature,
        "system": system_prompt,
        "messages": api_messages,
    });

    info!("Claude API call: model={}, messages={}", model, api_messages.len());

    let client = reqwest::Client::new();
    let resp = client
        .post(endpoint)
        .header("x-api-key", &provider.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await?;

    let status = resp.status();
    let text = resp.text().await?;

    if !status.is_success() {
        error!("Claude API error {}: {}", status, text);
        bail!("Claude API error {}: {}", status, &text[..text.len().min(500)]);
    }

    let json: serde_json::Value = serde_json::from_str(&text)?;

    // Extract text from content blocks
    let content = json
        .get("content")
        .and_then(|c| c.as_array())
        .and_then(|arr| {
            arr.iter()
                .filter_map(|block| {
                    if block.get("type").and_then(|t| t.as_str()) == Some("text") {
                        block.get("text").and_then(|t| t.as_str()).map(String::from)
                    } else {
                        None
                    }
                })
                .reduce(|a, b| format!("{}{}", a, b))
        })
        .unwrap_or_else(|| "No response content".into());

    Ok(content)
}

/// OpenAI-compatible API (works with OpenAI, local LLMs like LM Studio, Ollama, etc.)
/// Default chat endpoint for the OpenAI-compatible providers.
///
/// The two local runtimes listen on different ports and neither is a sensible
/// fallback for the other: sending an Ollama request to LM Studio's port fails
/// as a connection error, which reads like "the service is down" rather than
/// "the wrong service was addressed".
fn default_endpoint(provider_type: &str) -> &'static str {
    match provider_type {
        "ollama" => "http://127.0.0.1:11434/v1/chat/completions",
        "local" => "http://127.0.0.1:1234/v1/chat/completions",
        _ => "https://api.openai.com/v1/chat/completions",
    }
}

async fn chat_openai_compat(
    provider: &AiProvider,
    system_prompt: &str,
    history: &[ChatMessage],
    user_input: &str,
    max_tokens: u32,
    temperature: f64,
) -> Result<String> {
    let endpoint = if provider.endpoint.is_empty() {
        default_endpoint(&provider.provider_type)
    } else {
        &provider.endpoint
    };

    // Ollama serves whatever has been pulled locally, so there is no sensible
    // default model name — guessing one produces a 404 that reads like a
    // connection fault. An empty model is reported as the configuration gap it
    // is instead.
    if provider.model.is_empty() && provider.provider_type == "ollama" {
        bail!(
            "No Ollama model set. Run `ollama list` and put one of the names \
             (for example `gemma3:4b`) in the provider's model field."
        );
    }

    let model = if provider.model.is_empty() {
        "gpt-4o"
    } else {
        &provider.model
    };

    // Build messages array
    let mut api_messages: Vec<serde_json::Value> = vec![serde_json::json!({
        "role": "system",
        "content": system_prompt,
    })];

    for msg in history {
        if msg.role != "system" {
            api_messages.push(serde_json::json!({
                "role": msg.role,
                "content": msg.content,
            }));
        }
    }

    api_messages.push(serde_json::json!({
        "role": "user",
        "content": user_input,
    }));

    let body = serde_json::json!({
        "model": model,
        "max_tokens": max_tokens,
        "temperature": temperature,
        "messages": api_messages,
    });

    info!(
        "OpenAI-compat API call: endpoint={}, model={}, messages={}",
        endpoint,
        model,
        api_messages.len()
    );

    let client = reqwest::Client::new();
    let mut req = client
        .post(endpoint)
        .header("content-type", "application/json");

    // Only add auth header if API key is set (local models may not need one)
    if !provider.api_key.is_empty() {
        req = req.header("authorization", format!("Bearer {}", provider.api_key));
    }

    let resp = req.json(&body).send().await?;

    let status = resp.status();
    let text = resp.text().await?;

    if !status.is_success() {
        error!("OpenAI-compat API error {}: {}", status, text);
        bail!("API error {}: {}", status, &text[..text.len().min(500)]);
    }

    let json: serde_json::Value = serde_json::from_str(&text)?;

    let content = json
        .get("choices")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str())
        .unwrap_or("No response content")
        .to_string();

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_runtimes_do_not_share_a_default_endpoint() {
        // Ollama and LM Studio listen on different ports. Falling back to the
        // wrong one surfaces as a connection error, which misreports a
        // misconfiguration as an outage.
        assert!(default_endpoint("ollama").contains("11434"));
        assert!(default_endpoint("local").contains("1234"));
        assert_ne!(default_endpoint("ollama"), default_endpoint("local"));
    }

    #[test]
    fn unknown_provider_types_fall_back_to_openai() {
        assert!(default_endpoint("openai").contains("api.openai.com"));
        assert!(default_endpoint("something-else").contains("api.openai.com"));
    }

    #[test]
    fn input_placeholder_still_expands() {
        // {{selection}} and {{clipboard}} read live OS state, so only the
        // caller-supplied placeholder is asserted here.
        assert_eq!(render_template("say: {{input}}", "hello"), "say: hello");
    }

    #[test]
    fn a_template_without_placeholders_is_unchanged() {
        assert_eq!(render_template("no slots here", "ignored"), "no slots here");
    }
}
