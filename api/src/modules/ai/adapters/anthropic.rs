use anthropic::client::ClientBuilder;
use async_trait::async_trait;
use futures::stream::BoxStream;
use std::fmt;
use std::sync::Arc;

use crate::modules::ai::domain::{
    ChatRequest, ChatResponse, Message, MessageContent, Role, StreamEvent,
};
use crate::modules::ai::error::AiError;
use crate::modules::ai::ports::{LlmClient, ModelConfig};

pub struct AnthropicClient
{
    client: anthropic::client::Client,
    config: Arc<dyn ModelConfig>,
}

impl fmt::Debug for AnthropicClient
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("AnthropicClient")
            .field("config", &"<anthropic client>")
            .finish()
    }
}

impl AnthropicClient
{
    pub fn new(config: Arc<dyn ModelConfig>) -> Result<Self, String>
    {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| "ANTHROPIC_API_KEY environment variable not set".to_string())?;

        let client = ClientBuilder::default()
            .api_key(api_key)
            .build()
            .map_err(|e| format!("Failed to build anthropic client: {}", e))?;

        Ok(Self { client, config })
    }

    fn map_anthropic_error(&self, error: anthropic::error::AnthropicError) -> AiError
    {
        match error {
            anthropic::error::AnthropicError::ApiError(api_error) => {
                match api_error.r#type.as_str() {
                    "insufficient_quota" => AiError::QuotaExceeded,
                    "rate_limit_error" | "overloaded_error" => AiError::ProviderRateLimited,
                    "authentication_error" | "unauthorized" => AiError::Unauthorized,
                    "invalid_request_error" => AiError::InvalidRequest(api_error.message),
                    _ => AiError::Internal(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("API error ({}): {}", api_error.r#type, api_error.message),
                    ))),
                }
            }
            anthropic::error::AnthropicError::InvalidArgument(msg) => AiError::InvalidRequest(msg),
            err => AiError::Internal(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err.to_string(),
            ))),
        }
    }

    fn convert_messages(
        &self,
        messages: &[crate::modules::ai::domain::Message],
    ) -> Vec<anthropic::types::Message>
    {
        messages
            .iter()
            .map(|msg| {
                let content = vec![match &msg.content {
                    crate::modules::ai::domain::MessageContent::Text(text) => {
                        anthropic::types::ContentBlock::Text { text: text.clone() }
                    }
                    _ => anthropic::types::ContentBlock::Text {
                        text: String::new(),
                    },
                }];

                anthropic::types::Message {
                    role: match msg.role {
                        crate::modules::ai::domain::Role::User => anthropic::types::Role::User,
                        crate::modules::ai::domain::Role::Assistant => {
                            anthropic::types::Role::Assistant
                        }
                        _ => anthropic::types::Role::User,
                    },
                    content,
                }
            })
            .collect()
    }

    fn convert_tools(
        &self,
        _tools: &[crate::modules::ai::domain::ToolDefinition],
    ) -> Vec<anthropic::types::Message>
    {
        todo!("Tool use not yet supported in anthropic v0.0.8")
    }
}

#[async_trait]
impl LlmClient for AnthropicClient
{
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AiError>
    {
        let model = request
            .params
            .model
            .as_deref()
            .unwrap_or(self.config.default_model());
        let max_tokens = request
            .params
            .max_tokens
            .unwrap_or(self.config.default_max_tokens());
        let temperature = request
            .params
            .temperature
            .unwrap_or(self.config.default_temperature());

        let messages = self.convert_messages(&request.messages);
        let _tools = self.convert_tools(&request.tools);

        let mut msg_req = anthropic::types::MessagesRequest::default();
        msg_req.model = model.to_string();
        msg_req.messages = messages;
        msg_req.max_tokens = max_tokens as usize;
        msg_req.temperature = Some(temperature as f64);

        let response = self
            .client
            .messages(msg_req)
            .await
            .map_err(|e| self.map_anthropic_error(e))?;

        let content = if let Some(content_block) = response.content.first() {
            match content_block {
                anthropic::types::ContentBlock::Text { text } => MessageContent::Text(text.clone()),
                _ => MessageContent::Text(String::new()),
            }
        } else {
            MessageContent::Text(String::new())
        };

        let stop_reason = match response.stop_reason {
            Some(anthropic::types::StopReason::EndTurn) => {
                crate::modules::ai::domain::StopReason::EndOfTurn
            }
            Some(anthropic::types::StopReason::MaxTokens) => {
                crate::modules::ai::domain::StopReason::MaxTokens
            }
            Some(anthropic::types::StopReason::StopSequence) => {
                crate::modules::ai::domain::StopReason::EndOfTurn
            }
            None => crate::modules::ai::domain::StopReason::EndOfTurn,
        };

        Ok(ChatResponse {
            message: Message {
                role: Role::Assistant,
                content,
            },
            stop_reason,
            usage: crate::modules::ai::domain::Usage {
                input_tokens: response.usage.input_tokens as u32,
                output_tokens: response.usage.output_tokens as u32,
            },
        })
    }

    async fn stream_chat(
        &self,
        request: ChatRequest,
    ) -> Result<BoxStream<'static, Result<StreamEvent, AiError>>, AiError>
    {
        let model = request
            .params
            .model
            .as_deref()
            .unwrap_or(self.config.default_model());
        let max_tokens = request
            .params
            .max_tokens
            .unwrap_or(self.config.default_max_tokens());
        let temperature = request
            .params
            .temperature
            .unwrap_or(self.config.default_temperature());

        let messages = self.convert_messages(&request.messages);
        let _tools = self.convert_tools(&request.tools);

        let mut msg_req = anthropic::types::MessagesRequest::default();
        msg_req.model = model.to_string();
        msg_req.messages = messages;
        msg_req.max_tokens = max_tokens as usize;
        msg_req.temperature = Some(temperature as f64);
        msg_req.stream = true;

        let stream = self
            .client
            .messages_stream(msg_req)
            .await
            .map_err(|e| self.map_anthropic_error(e))?;

        use futures::StreamExt;

        let boxed = Box::pin(stream.then(|event_result| async move {
            match event_result {
                Ok(event) => match event {
                    anthropic::types::MessagesStreamEvent::ContentBlockDelta { delta, .. } => {
                        match delta {
                            anthropic::types::ContentBlockDelta::TextDelta { text } => {
                                Ok(StreamEvent::TextDelta(text))
                            }
                        }
                    }
                    anthropic::types::MessagesStreamEvent::MessageStop => Ok(StreamEvent::Done {
                        stop_reason: crate::modules::ai::domain::StopReason::EndOfTurn,
                        usage: crate::modules::ai::domain::Usage {
                            input_tokens: 0,
                            output_tokens: 0,
                        },
                    }),
                    _ => Ok(StreamEvent::TextDelta(String::new())),
                },
                Err(e) => Err(AiError::Internal(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                )))),
            }
        }));

        Ok(boxed)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    struct MockModelConfig;

    impl crate::modules::ai::ports::ModelConfig for MockModelConfig
    {
        fn default_model(&self) -> &str
        {
            "claude-3-5-sonnet-20241022"
        }
        fn default_max_tokens(&self) -> u32
        {
            1024
        }
        fn default_temperature(&self) -> f32
        {
            0.7
        }
    }

    #[test]
    fn test_new_requires_api_key()
    {
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
    fn test_new_succeeds_with_api_key()
    {
        unsafe {
            std::env::set_var("ANTHROPIC_API_KEY", "test-key-12345");
        }
        let config = Arc::new(MockModelConfig);
        let result = AnthropicClient::new(config);
        assert!(result.is_ok());
    }
}
