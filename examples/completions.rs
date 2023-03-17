use openai_rust::data::completion::{
    CompletionRequestParams, CompletionResponseParams, COMPLETION_ENDPOINT,
};
use reqwest::Error;

use std::env;

use eyre::Result;
use openai_rust::{OpenAIAPIRequest, Session};

async fn get_completion_response(
    api_key: String,
    model: String,
    prompt: String,
) -> Result<CompletionResponseParams, Error> {
    let session = Session::builder().api_key(api_key).build();
    let completion_params = CompletionRequestParams::builder()
        .model(model.to_string())
        .prompt(prompt)
        .max_tokens(7)
        .temperature(0.0)
        .build();

    session
        .make_completion_request(COMPLETION_ENDPOINT, completion_params)
        .await
}

#[tokio::main]
async fn main() {
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(val) => val,
        Err(e) => panic!("OPENAI_API_KEY is not set. Error: {}", e),
    };
    let model = String::from("text-davinci-003");
    let prompt = String::from("Say this is a test");
    let completion_response = get_completion_response(api_key, model, prompt)
        .await
        .unwrap();
    println!("{:#?}", completion_response);
}
