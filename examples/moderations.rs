use openai_rust::data::moderation::{
    ModerationRequestParams, ModerationResponseParams, MODERATION_ENDPOINT,
};

use eyre::Result;
use openai_rust::{OpenAIAPIRequest, Session};
use reqwest::Error;
use std::env;

async fn get_moderation_response(
    api_key: String,
    input: String,
) -> Result<ModerationResponseParams, Error> {
    let session = Session::builder().api_key(api_key).build();
    let moderation_params = ModerationRequestParams::builder().input(input).build();

    session
        .make_moderation_request(MODERATION_ENDPOINT, moderation_params)
        .await
}

#[tokio::main]
async fn main() {
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(val) => val,
        Err(e) => panic!("OPENAI_API_KEY is not set. Error: {}", e),
    };
    let input = String::from("I want to kill them");
    let moderation_response = get_moderation_response(api_key, input).await.unwrap();
    println!("{:#?}", moderation_response);
}
