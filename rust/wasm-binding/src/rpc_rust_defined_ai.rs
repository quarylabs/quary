use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

pub async fn get_predicted_query(
    database_prefix: &str,
    natural_language_query: &str,
    schema: &str,
    database_type: &str,
    key: &str,
) -> Result<String, String> {
    let prompt = format!(
        "Given you are a {} database, 
with the following schema {}, 
and the following natural language query: {}, 
what is the SQL query that you would write to answer the question?
In the returned query can you prefix any reference to a table with the schema prefix {} 
like the table were in the schema {}, only do so though in the FROM or JOIN clauses of the query, giving them aliases and using that alias anywhere else.",
        database_type,
        schema,
        natural_language_query,
        database_prefix,
        database_prefix
    );

    make_completion_request_to_openai(prompt.as_str(), key).await
}

pub async fn get_predicted_model_name(
    natural_language_query: &str,
    key: &str,
) -> Result<String, String> {
    let prompt = format!(
        "Given the following natural language query: \"{}\" for an sql model, give me a short name for the model without special characters, all lowercase and words spaced with a underscore?",
        natural_language_query
    );

    make_completion_request_to_openai(prompt.as_str(), key).await
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn fetch(request: &Request) -> js_sys::Promise;
}

#[derive(Serialize)]
struct CompletionRequest {
    model: String,
    max_tokens: Option<i32>,
    temperature: Option<i32>,
    prompt: String,
}

#[derive(Deserialize)]
struct CompletionResponse {
    pub choices: Vec<CompletionChoice>,
}

#[derive(Deserialize)]
struct CompletionChoice {
    pub text: String,
}

async fn make_completion_request_to_openai(prompt: &str, key: &str) -> Result<String, String> {
    let request = CompletionRequest {
        model: String::from("text-davinci-003"),
        max_tokens: Some(512),
        temperature: Some(0),
        prompt: prompt.to_string(),
    };

    let body = serde_json::to_string(&request)
        .map_err(|e| format!("Failed to serialize request: {}", e))?;

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(&body)));

    let headers = Headers::new().map_err(|_| "Failed to create headers".to_string())?;
    headers
        .append("Authorization", &format!("Bearer {}", key))
        .map_err(|_| "Failed to append auth header".to_string())?;
    headers
        .append("Content-Type", "application/json")
        .map_err(|_| "Failed to append content type header".to_string())?;

    opts.headers(&headers);
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("https://api.openai.com/v1/completions", &opts)
        .map_err(|_| "Failed to create request".to_string())?;

    // let window = web_sys::window();
    // if let Some(window) = window {
    let resp_value = JsFuture::from(fetch(&request))
        .await
        .map_err(|_| "Failed to fetch request".to_string())?;
    let resp: Response = resp_value
        .dyn_into()
        .map_err(|err| format!("Failed to cast response into Response: {:?}", err))?;
    if !resp.ok() {
        return Err("Failed to get predicted query".to_string());
    }
    let json = resp
        .json()
        .map_err(|err| format!("Failed to get json from response: {:?}", err))?;
    let json = JsFuture::from(json)
        .await
        .map_err(|_| "Failed to get predicted query".to_string())?;
    let example: CompletionResponse = serde_wasm_bindgen::from_value(json)
        .map_err(|e| format!("Failed to deserialize response from openai: {}", e))?;

    Ok(example
        .choices
        .first()
        .ok_or("Failed to get first choice from openai response".to_string())?
        .text
        .to_string())
}
