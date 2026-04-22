//! LLM client factory — converts kreuzberg's LlmConfig to a liter-llm DefaultClient.

use std::time::Duration;

use liter_llm::client::{ClientConfigBuilder, DefaultClient};

use crate::core::config::LlmConfig;

/// Create a liter-llm [`DefaultClient`] from kreuzberg's [`LlmConfig`].
///
/// The `model` field is passed as a hint so liter-llm can resolve the correct
/// provider automatically. When `api_key` is not set in the config, this
/// function falls back to the provider's standard environment variable
/// (e.g. `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`). Use `KREUZBERG_LLM_API_KEY`
/// to set the key uniformly across all providers.
pub fn create_client(config: &LlmConfig) -> crate::Result<DefaultClient> {
    let mut api_key = config.api_key.clone().unwrap_or_default();

    if api_key.is_empty() {
        let model = config.model.to_lowercase();
        api_key = if model.starts_with("anthropic/") || model.starts_with("claude-") {
            std::env::var("ANTHROPIC_API_KEY").unwrap_or_default()
        } else if model.starts_with("gemini/") || model.starts_with("google_ai/") {
            std::env::var("GEMINI_API_KEY").unwrap_or_default()
        } else if model.starts_with("mistral/")
            || model.starts_with("mistral-")
            || model.starts_with("codestral-")
            || model.starts_with("pixtral-")
        {
            std::env::var("MISTRAL_API_KEY").unwrap_or_default()
        } else if model.starts_with("cohere/") || model.starts_with("command-") {
            std::env::var("COHERE_API_KEY").unwrap_or_default()
        } else {
            std::env::var("OPENAI_API_KEY").unwrap_or_default()
        };
    }

    let mut builder = ClientConfigBuilder::new(api_key);

    if let Some(ref base_url) = config.base_url {
        builder = builder.base_url(base_url.clone());
    }
    if let Some(timeout) = config.timeout_secs {
        builder = builder.timeout(Duration::from_secs(timeout));
    }
    if let Some(max_retries) = config.max_retries {
        builder = builder.max_retries(max_retries);
    }

    let client_config = builder.build();

    DefaultClient::new(client_config, Some(&config.model)).map_err(|e| {
        let msg = format!("Failed to build LLM client: {e}");
        crate::KreuzbergError::Validation {
            message: msg,
            source: Some(Box::new(e)),
        }
    })
}
