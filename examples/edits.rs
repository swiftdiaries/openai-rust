use std::env;

use eyre::Result;
use reqwest::Error;

use openai_rust::data::edit::{EditRequestParams, EditResponseParams, EDIT_ENDPOINT};
use openai_rust::{OpenAIAPIRequest, Session};

async fn get_edit_response(
    api_key: String,
    model: String,
    input: String,
    instruction: String,
) -> Result<EditResponseParams, Error> {
    let session = Session::builder().api_key(api_key).build();
    let edit_request_params = EditRequestParams::builder()
        .model(model.to_string())
        .input(input)
        .instruction(instruction)
        .build();

    session
        .make_edit_request(EDIT_ENDPOINT, edit_request_params)
        .await
}

#[tokio::main]
async fn main() {
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(val) => val,
        Err(e) => panic!("OPENAI_API_KEY is not set. Error: {}", e),
    };
    let model = String::from("text-davinci-edit-001");
    let input = String::from("What day of the wek is it");
    let instruction = String::from("Fix the spelling mistakes");
    let edit_response = get_edit_response(api_key, model, input, instruction)
        .await
        .unwrap();
    println!("{:#?}", edit_response);
}
