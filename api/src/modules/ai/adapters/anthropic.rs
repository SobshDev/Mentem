use std::sync::Arc;
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
