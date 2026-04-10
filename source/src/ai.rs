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
            wf.user_template.replace("{{input}}", user_input)
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
        "openai" | "local" => {
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
async fn chat_openai_compat(
    provider: &AiProvider,
    system_prompt: &str,
    history: &[ChatMessage],
    user_input: &str,
    max_tokens: u32,
    temperature: f64,
) -> Result<String> {
    let endpoint = if provider.endpoint.is_empty() {
        "https://api.openai.com/v1/chat/completions"
    } else {
        &provider.endpoint
    };

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
