pub mod domain;

mod adapters;
mod error;
mod ports;
mod service;

pub use domain::{
    ChatRequest, ChatResponse, Conversation, ConversationId, GenerationParams, Message,
    MessageContent, Role, StopReason, StreamEvent, ToolCall, ToolDefinition, ToolResult, Usage,
};
pub use service::AiService;
