use std::sync::Arc;

use futures::stream::BoxStream;

use crate::modules::auth::domain::UserId;

use super::domain::{ChatResponse, ConversationId, StreamEvent, ToolDefinition};
use super::error::AiError;
use super::ports::{AiQuota, ConversationRepository, LlmClient, ModelConfig};

#[derive(Clone)]
pub struct AiService
{
    llm: Arc<dyn LlmClient>,
    conversations: Arc<dyn ConversationRepository>,
    config: Arc<dyn ModelConfig>,
    quota: Arc<dyn AiQuota>,
}

impl AiService
{
    pub fn new(
        llm: Arc<dyn LlmClient>,
        conversations: Arc<dyn ConversationRepository>,
        config: Arc<dyn ModelConfig>,
        quota: Arc<dyn AiQuota>,
    ) -> Self
    {
        Self {
            llm,
            conversations,
            config,
            quota,
        }
    }

    pub async fn complete(&self, _user: &UserId, _prompt: String) -> Result<ChatResponse, AiError>
    {
        todo!()
    }

    pub async fn chat(
        &self,
        _user: &UserId,
        _conv: &ConversationId,
        _msg: String,
        _tools: Vec<ToolDefinition>,
    ) -> Result<ChatResponse, AiError>
    {
        todo!()
    }

    pub async fn stream_chat(
        &self,
        _user: &UserId,
        _conv: &ConversationId,
        _msg: String,
        _tools: Vec<ToolDefinition>,
    ) -> Result<BoxStream<'static, Result<StreamEvent, AiError>>, AiError>
    {
        todo!()
    }
}
