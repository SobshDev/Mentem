use uuid::Uuid;

use crate::modules::auth::domain::UserId;

pub type ConversationId = Uuid;

#[derive(Debug, Clone)]
pub enum Role
{
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone)]
pub enum MessageContent
{
    Text(String),
    ToolCalls(Vec<ToolCall>),
    ToolResult(ToolResult),
}

#[derive(Debug, Clone)]
pub struct Message
{
    pub role: Role,
    pub content: MessageContent,
}

#[derive(Debug, Clone)]
pub struct ToolDefinition
{
    pub name: String,
    pub description: String,
    pub parameters_schema: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ToolCall
{
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ToolResult
{
    pub tool_call_id: String,
    pub content: String,
    pub is_error: bool,
}

#[derive(Debug, Clone)]
pub struct GenerationParams
{
    pub model: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct ChatRequest
{
    pub messages: Vec<Message>,
    pub tools: Vec<ToolDefinition>,
    pub params: GenerationParams,
}

#[derive(Debug, Clone)]
pub enum StopReason
{
    EndOfTurn,
    ToolUse,
    MaxTokens,
    Refusal,
    Other(String),
}

#[derive(Debug, Clone, Copy)]
pub struct Usage
{
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Clone)]
pub struct ChatResponse
{
    pub message: Message,
    pub stop_reason: StopReason,
    pub usage: Usage,
}

#[derive(Debug, Clone)]
pub enum StreamEvent
{
    TextDelta(String),
    ToolCall(ToolCall),
    Done
    {
        stop_reason: StopReason,
        usage: Usage,
    },
}

#[derive(Debug, Clone)]
pub struct Conversation
{
    pub id: ConversationId,
    pub user_id: UserId,
    pub messages: Vec<Message>,
    pub created_at: i64,
    pub updated_at: i64,
}
