mod data;
use std::env;

use data::model::make_request;

use crate::data::model::{CompletionRequestParams, Session};

#[tokio::main]
async fn main() {
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(val) => val,
        Err(e) => panic!("OPENAI_API_KEY is not set. Error: {}", e),
    };
    let org_id = "Personal";
    let session: Session = data::model::Request::new(&api_key, org_id);
    let model = "text-davinci-003";
    let prompt1 = String::from("Say this is a test");
    let mut prompt = Vec::new();
    prompt.push(prompt1);
    let completion_params = CompletionRequestParams {
        model: model.to_string(),
        prompt,
        max_tokens: 7,
        temperature: 0,
    };
    let completion_response = make_request(
        session.client,
        &session.api_key,
        "https://api.openai.com/v1/completions",
        &completion_params,
    )
    .await;
    let completion_response_text = match completion_response {
        Ok(completion_response_text) => completion_response_text.text().await.unwrap(),
        Err(e) => panic!("Error unwrapping response: {}", e),
    };
    println!("{:#?}", completion_response_text);
}
