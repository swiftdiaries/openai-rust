mod data;
use data::model::make_request;
use std::env;
use std::error::Error;

use crate::data::model::{CompletionRequestParams, CompletionResponseParams, Session};

async fn get_completion_response(
    api_key: String,
    model: String,
    prompt: String,
) -> Result<CompletionResponseParams, Box<dyn Error + Send + Sync>> {
    let session = Session::builder().api_key(api_key).build();
    let completion_params = CompletionRequestParams::builder()
        .model(model.to_string())
        .prompt(prompt)
        .max_tokens(7)
        .temperature(0.0)
        .build();
    let completion_response = make_request(
        session.client,
        &session.api_key,
        "https://api.openai.com/v1/completions",
        completion_params,
    )
    .await;
    //let mut completion_response_params: CompletionResponseParams;
    let completion_response_body = match completion_response {
        Ok(completion_response_body) => completion_response_body.text().await.unwrap(),
        Err(e) => panic!("Error unwrapping response: {}", e),
    };
    let completion_response_params = serde_json::from_str(completion_response_body.as_str())?;
    Ok(completion_response_params)
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
