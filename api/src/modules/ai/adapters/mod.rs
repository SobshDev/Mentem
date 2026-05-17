mod anthropic;

pub use anthropic::AnthropicClient;

use crate::modules::ai::ports::LlmClient;
use crate::modules::ai::ports::ModelConfig;
use std::sync::Arc;

pub fn build_llm_client(config: Arc<dyn ModelConfig>) -> Result<Arc<dyn LlmClient>, String>
{
    Ok(Arc::new(AnthropicClient::new(config)?))
}
