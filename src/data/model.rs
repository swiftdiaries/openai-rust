use eyre::Result;
use reqwest::{header::CONTENT_TYPE, header::USER_AGENT, Client};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct Session {
    pub api_key: String,
    #[builder(default = "Personal".to_string())]
    pub org_id: String,
    #[builder(default = reqwest::Client::new())]
    pub client: reqwest::Client,
}

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

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
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
    #[builder(setter(strip_option), default)]
    pub logprobs: Option<i64>,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

pub async fn make_request(
    client: Client,
    api_key: &str,
    endpoint: &str,
    params: CompletionRequestParams,
) -> Result<reqwest::Response, reqwest::Error> {
    let response = client
        .post(endpoint)
        .bearer_auth(api_key)
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "openai-rust/1")
        .json(&params)
        .send()
        .await;
    response
}
