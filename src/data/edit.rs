use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::Usage;

pub const EDIT_ENDPOINT: &str = "https://api.openai.com/v1/edits";

#[derive(Serialize, Deserialize, TypedBuilder, Clone)]
pub struct EditRequestParams {
    #[builder(default = "text-davinci-edit-001".to_string())]
    pub model: String,
    pub instruction: String,
    #[builder(default = "".to_string())]
    pub input: String,
    #[builder(default = 1)]
    pub n: i64,
    #[builder(default = 1.0)]
    pub temperature: f64,
    #[builder(default = 1.0)]
    pub top_p: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditResponseParams {
    pub object: String,
    pub created: u64,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub text: String,
    pub index: u64,
}
