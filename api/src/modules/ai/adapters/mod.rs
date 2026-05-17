mod anthropic;

pub use anthropic::AnthropicClient;

use std::sync::Arc;
use crate::modules::ai::ports::ModelConfig;
use crate::modules::ai::ports::LlmClient;

pub fn build_llm_client(config: Arc<dyn ModelConfig>) -> Result<Arc<dyn LlmClient>, String> {
    Ok(Arc::new(AnthropicClient::new(config)?))
}
