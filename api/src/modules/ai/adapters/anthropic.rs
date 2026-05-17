use std::sync::Arc;
use async_trait::async_trait;
use futures::stream::BoxStream;

use crate::modules::ai::ports::{ModelConfig, LlmClient};
use crate::modules::ai::domain::{ChatRequest, ChatResponse, StreamEvent};
use crate::modules::ai::error::AiError;

pub struct AnthropicClient {
    config: Arc<dyn ModelConfig>,
}

impl AnthropicClient {
    pub fn new(config: Arc<dyn ModelConfig>) -> Result<Self, String> {
        Ok(AnthropicClient { config })
    }
}

#[async_trait]
impl LlmClient for AnthropicClient {
    async fn chat(&self, _request: ChatRequest) -> Result<ChatResponse, AiError> {
        todo!("AnthropicClient::chat implementation")
    }

    async fn stream_chat(
        &self,
        _request: ChatRequest,
    ) -> Result<BoxStream<'static, Result<StreamEvent, AiError>>, AiError> {
        todo!("AnthropicClient::stream_chat implementation")
    }
}
