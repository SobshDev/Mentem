use async_trait::async_trait;
use futures::stream::BoxStream;

use crate::modules::auth::domain::UserId;

use super::domain::{
    ChatRequest, ChatResponse, Conversation, ConversationId, Message, StreamEvent, Usage,
};
use super::error::AiError;

#[async_trait]
pub trait LlmClient: Send + Sync
{
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AiError>;

    async fn stream_chat(
        &self,
        request: ChatRequest,
    ) -> Result<BoxStream<'static, Result<StreamEvent, AiError>>, AiError>;
}

#[async_trait]
pub trait ConversationRepository: Send + Sync
{
    async fn create(&self, owner: UserId) -> Result<Conversation, AiError>;
    async fn find_by_id(&self, id: &ConversationId) -> Result<Option<Conversation>, AiError>;
    async fn append_message(&self, id: &ConversationId, message: Message) -> Result<(), AiError>;
    async fn list_for_user(&self, user_id: &UserId) -> Result<Vec<Conversation>, AiError>;
}

pub trait ModelConfig: Send + Sync
{
    fn default_model(&self) -> &str;
    fn default_max_tokens(&self) -> u32;
    fn default_temperature(&self) -> f32;
}

#[async_trait]
pub trait AiQuota: Send + Sync
{
    async fn check(&self, user_id: &UserId, estimated_tokens: u32) -> Result<(), AiError>;
    async fn record_usage(&self, user_id: &UserId, usage: &Usage) -> Result<(), AiError>;
}
