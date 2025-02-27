use llm_chain::traits;
use serde::{Deserialize, Serialize};

/// The `Model` enum represents the available ChatGPT models that you can use through the OpenAI API. These models have different capabilities and performance characteristics, allowing you to choose the one that best suits your needs.
///
/// Currently, the available models are:
/// - `ChatGPT3_5Turbo`: A high-performance and versatile model that offers a great balance of speed, quality, and affordability.
/// - `GPT4`: A high-performance model that offers the best quality, but is slower and more expensive than the `ChatGPT3_5Turbo` model.
/// - `Other(String)`: A variant that allows you to specify a custom model name as a string, in case new models are introduced or you have access to specialized models.
///
/// # Example
///
/// ```
/// use llm_chain_openai::chatgpt::Model;
///
/// let turbo_model = Model::ChatGPT3_5Turbo;
/// let custom_model = Model::Other("your_custom_model_name".to_string());
/// ```
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Model {
    ChatGPT3_5Turbo,
    GPT4,
    Other(String),
}

impl Default for Model {
    fn default() -> Self {
        Self::ChatGPT3_5Turbo
    }
}

/// The `Model` enum implements the `ToString` trait, allowing you to easily convert it to a string.
impl ToString for Model {
    fn to_string(&self) -> String {
        match &self {
            Self::ChatGPT3_5Turbo => "gpt-3.5-turbo".to_string(),
            Self::GPT4 => "gpt-4".to_string(),
            Self::Other(model) => model.to_string(),
        }
    }
}

/// The `Model` enum implements the `From<String>` trait, allowing you to easily convert a string to a `Model`.
impl From<String> for Model {
    fn from(s: String) -> Self {
        match s.as_str() {
            "gpt-3.5-turbo" => Self::ChatGPT3_5Turbo,
            "gpt-4" => Self::GPT4,
            _ => Self::Other(s),
        }
    }
}

/// The `PerInvocation` struct contains options that can be specified for each ChatGPT invocation.
/// Currently, it only supports specifying a `Model`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerInvocation {
    pub(crate) model: Option<Model>,
}

impl PerInvocation {
    /// Creates a new `PerInvocation` struct with default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the `Model` for the `PerInvocation` struct.
    pub fn for_model(self: Self, model: Model) -> Self {
        Self {
            model: Some(model),
            ..self
        }
    }
}

impl traits::Options for PerInvocation {}

/// The `PerExecutor` struct contains options that can be specified for the ChatGPT `Executor`.
/// Currently, it only supports specifying an `api_key`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerExecutor {
    pub api_key: Option<String>,
}

impl traits::Options for PerExecutor {}
