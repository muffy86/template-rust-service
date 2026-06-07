//! Provider-agnostic AI client for Rust services.
//!
//! The infra-automation workflow `ci-rust.yml` exercises this on every push.

use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Copy)]
pub enum Provider { Ollama, OpenAi, Anthropic, Venice, Groq, OpenRouter }

impl Provider {
    pub fn from_env() -> Self {
        match env::var("AI_PROVIDER").unwrap_or_else(|_| "ollama".into()).as_str() {
            "openai" => Self::OpenAi,
            "anthropic" => Self::Anthropic,
            "venice" => Self::Venice,
            "groq" => Self::Groq,
            "openrouter" => Self::OpenRouter,
            _ => Self::Ollama,
        }
    }
    pub fn base_url(&self) -> &'static str {
        match self {
            Self::Ollama => "http://localhost:11434/v1",
            Self::OpenAi => "https://api.openai.com/v1",
            Self::Anthropic => "https://api.anthropic.com/v1",
            Self::Venice => "https://api.venice.ai/v1",
            Self::Groq => "https://api.groq.com/openai/v1",
            Self::OpenRouter => "https://openrouter.ai/api/v1",
        }
    }
    pub fn default_model(&self) -> &'static str {
        match self {
            Self::Ollama => "llama3.1:8b",
            Self::OpenAi => "gpt-4o-mini",
            Self::Anthropic => "claude-3-5-sonnet-20241022",
            Self::Venice => "llama-3.1-405b",
            Self::Groq => "llama-3.1-70b-versatile",
            Self::OpenRouter => "meta-llama/llama-3.1-70b-instruct",
        }
    }
}

#[derive(Debug, Serialize)]
struct ChatReq<'a> {
    model: &'a str,
    messages: Vec<Msg<'a>>,
    temperature: f32,
    max_tokens: u32,
}
#[derive(Debug, Serialize)]
struct Msg<'a> { role: &'a str, content: &'a str }
#[derive(Debug, Deserialize)]
struct ChatResp { choices: Vec<Choice> }
#[derive(Debug, Deserialize)]
struct Choice { message: RespMsg }
#[derive(Debug, Deserialize)]
struct RespMsg { content: String }

pub async fn chat(prompt: &str) -> anyhow::Result<String> {
    let provider = Provider::from_env();
    let model = env::var("AI_MODEL").unwrap_or_else(|_| provider.default_model().into());
    let api_key = env::var("AI_API_KEY").unwrap_or_else(|_| "ollama".into());

    let url = format!("{}/chat/completions", env::var("AI_BASE_URL").unwrap_or_else(|_| provider.base_url().into()));
    let body = ChatReq {
        model: &model,
        messages: vec![Msg { role: "user", content: prompt }],
        temperature: 0.7,
        max_tokens: 1024,
    };
    let client = reqwest::Client::new();
    let resp = client.post(url)
        .bearer_auth(api_key)
        .json(&body)
        .send().await?
        .error_for_status()?
        .json::<ChatResp>().await?;
    Ok(resp.choices.into_iter().next().map(|c| c.message.content).unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn provider_roundtrip() {
        for p in ["ollama", "openai", "anthropic", "venice", "groq", "openrouter"] {
            std::env::set_var("AI_PROVIDER", p);
            let provider = Provider::from_env();
            assert!(provider.base_url().starts_with("http"));
            assert!(!provider.default_model().is_empty());
        }
    }
}
