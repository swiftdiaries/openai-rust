use crate::Usage;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub const COMPLETION_ENDPOINT: &str = "https://api.openai.com/v1/completions";

#[derive(Debug, Serialize, TypedBuilder, Clone)]
pub struct CompletionRequestParams {
    pub model: String,
    #[builder(default)]
    pub prompt: String,
    #[builder(default = 16)]
    pub max_tokens: i64,
    #[builder(default = 1.0)]
    pub temperature: f64,
    #[builder(default = 1.0)]
    pub top_p: f64,
    #[builder(default = 1)]
    pub n: u64,
    #[builder(default = false)]
    pub stream: bool,
    #[builder(setter(strip_option), default)]
    pub logprobs: Option<i64>,
    #[builder(setter(strip_option), default)]
    pub stop: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResponseParams {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
pub struct Choice {
    pub text: String,
    pub index: u64,
    #[builder(setter(strip_option))]
    pub logprobs: Option<i64>,
    pub finish_reason: String,
}
