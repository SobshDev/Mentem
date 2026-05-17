use std::sync::Arc;
use std::fmt;
use async_trait::async_trait;
use futures::stream::BoxStream;
use anthropic::client::ClientBuilder;

use crate::modules::ai::domain::{ChatRequest, ChatResponse, StreamEvent};
use crate::modules::ai::error::AiError;
use crate::modules::ai::ports::{LlmClient, ModelConfig};

pub struct AnthropicClient {
    client: anthropic::client::Client,
    config: Arc<dyn ModelConfig>,
}

impl fmt::Debug for AnthropicClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnthropicClient")
            .field("config", &"<anthropic client>")
            .finish()
    }
}

impl AnthropicClient {
    pub fn new(config: Arc<dyn ModelConfig>) -> Result<Self, String> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| "ANTHROPIC_API_KEY environment variable not set".to_string())?;

        let client = ClientBuilder::default()
            .api_key(api_key)
            .build()
            .map_err(|e| format!("Failed to build anthropic client: {}", e))?;

        Ok(Self { client, config })
    }
}

#[async_trait]
impl LlmClient for AnthropicClient {
    async fn chat(&self, _request: ChatRequest) -> Result<ChatResponse, AiError> {
        todo!("Implement chat")
    }

    async fn stream_chat(
        &self,
        _request: ChatRequest,
    ) -> Result<BoxStream<'static, Result<StreamEvent, AiError>>, AiError> {
        todo!("Implement stream_chat")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockModelConfig;

    impl crate::modules::ai::ports::ModelConfig for MockModelConfig {
        fn default_model(&self) -> &str {
            "claude-3-5-sonnet-20241022"
        }
        fn default_max_tokens(&self) -> u32 {
            1024
        }
        fn default_temperature(&self) -> f32 {
            0.7
        }
    }

    #[test]
    fn test_new_requires_api_key() {
        // Temporarily unset the env var if it exists
        let old_key = std::env::var("ANTHROPIC_API_KEY").ok();
        unsafe {
            std::env::remove_var("ANTHROPIC_API_KEY");
        }

        let config = Arc::new(MockModelConfig);
        let result = AnthropicClient::new(config);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("ANTHROPIC_API_KEY"));

        // Restore if it was set
        if let Some(key) = old_key {
            unsafe {
                std::env::set_var("ANTHROPIC_API_KEY", key);
            }
        }
    }

    #[test]
    fn test_new_succeeds_with_api_key() {
        unsafe {
            std::env::set_var("ANTHROPIC_API_KEY", "test-key-12345");
        }
        let config = Arc::new(MockModelConfig);
        let result = AnthropicClient::new(config);
        assert!(result.is_ok());
    }
}
