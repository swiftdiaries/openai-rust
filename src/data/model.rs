use async_trait::async_trait;
use eyre::Result;
use reqwest::{header::CONTENT_TYPE, header::USER_AGENT, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Session {
    pub api_key: String,
    pub org_id: String,
    pub client: reqwest::Client,
}

/*
{
  "model": "text-davinci-003",
  "prompt": "Say this is a test",
  "max_tokens": 7,
  "temperature": 0,
  "top_p": 1,
  "n": 1,
  "stream": false,
  "logprobs": null,
  "stop": "\n"
}
*/
#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
pub struct CompletionRequestParams {
    pub model: String,
    pub prompt: Option<Vec<String>>,
    pub max_tokens: Option<i64>,
    pub temperature: Option<i64>,
    pub top_p: Option<i64>,
    pub n: Option<i64>,
    pub stream: Option<bool>,
    pub logprobs: Option<i64>,
    pub stop: Option<Vec<String>>,
    pub user: Option<String>,
}

#[async_trait]
pub trait Request {
    fn new(api_key: &str, org_id: &str) -> Self;
}

#[async_trait]
impl Request for Session {
    fn new(api_key: &str, org_id: &str) -> Session {
        let client = reqwest::Client::new();
        Self {
            api_key: api_key.to_string(),
            org_id: org_id.to_string(),
            client,
        }
    }
}

pub async fn make_request(
    client: Client,
    api_key: &str,
    endpoint: &str,
    params: &CompletionRequestParams,
) -> Result<reqwest::Response, reqwest::Error> {
    let response = client
        .post(endpoint)
        .bearer_auth(api_key)
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "openai-rust/1")
        .json(params)
        .send()
        .await;
    return response;
}
