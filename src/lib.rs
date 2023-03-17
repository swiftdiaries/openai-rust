pub mod data;
use crate::data::completion::CompletionRequestParams;

use async_trait::async_trait;
use data::{
    completion::CompletionResponseParams,
    moderations::{ModerationRequestParams, ModerationResponseParams},
};
use eyre::Result;
use reqwest::{header::CONTENT_TYPE, header::USER_AGENT};
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct Session {
    pub api_key: String,
    #[builder(default = "Personal".to_string())]
    pub org_id: String,
    #[builder(default = reqwest::Client::new())]
    pub client: reqwest::Client,
}

#[async_trait]
pub trait OpenAIAPIRequest {
    async fn make_completion_request(
        &self,
        endpoint: &str,
        completion_params: CompletionRequestParams,
    ) -> Result<CompletionResponseParams, reqwest::Error>;
    async fn make_moderation_request(
        &self,
        endpoint: &str,
        moderation_params: ModerationRequestParams,
    ) -> Result<ModerationResponseParams, reqwest::Error>;
}

#[async_trait]
impl OpenAIAPIRequest for Session {
    async fn make_completion_request(
        &self,
        endpoint: &str,
        completion_params: CompletionRequestParams,
    ) -> Result<CompletionResponseParams, reqwest::Error> {
        let completion_response = match self
            .client
            .post(endpoint)
            .bearer_auth(&self.api_key)
            .header(CONTENT_TYPE, "application/json")
            .header(USER_AGENT, "openai-rust/1")
            .json(&completion_params)
            .send()
            .await
        {
            Ok(completion_response) => completion_response,
            Err(e) => panic!("Error getting response from API, {}", e),
        };
        let completion_response_params =
            match completion_response.json::<CompletionResponseParams>().await {
                Ok(completion_response_params) => completion_response_params,
                Err(e) => panic!("Error unwrapping JSON to Response struct, {}", e),
            };
        Ok(completion_response_params)
    }

    async fn make_moderation_request(
        &self,
        endpoint: &str,
        moderation_params: ModerationRequestParams,
    ) -> Result<ModerationResponseParams, reqwest::Error> {
        let moderation_response = match self
            .client
            .post(endpoint)
            .bearer_auth(&self.api_key)
            .header(CONTENT_TYPE, "application/json")
            .header(USER_AGENT, "openai-rust/1")
            .json(&moderation_params)
            .send()
            .await
        {
            Ok(modertation_response) => modertation_response,
            Err(e) => panic!("Error getting response from API: {}", e),
        };
        let moderation_response_params =
            match moderation_response.json::<ModerationResponseParams>().await {
                Ok(moderation_response_params) => moderation_response_params,
                Err(e) => panic!("Error unwrapping JSON to Response struct: {}", e),
            };
        Ok(moderation_response_params)
    }
}
