use reqwest;
use reqwest::header::*;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GPT {
    model: String,
    prompt: String,
    max_tokens: u32,
    temperature: f32,
}
impl Default for GPT {
    fn default() -> GPT {
        GPT {
            model: "text-davinci-003".to_string(),
            prompt: String::new(),
            max_tokens: 256,
            temperature: 0.7,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GptPrompt {
    prompt: String,
}
impl Default for GptPrompt {
    fn default() -> GptPrompt {
        GptPrompt {
            prompt: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GptChoices {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GptResponse {
    choices: Vec<GptChoices>,
}

#[get("/v1/test/<text>")]
pub fn test(text: String) -> String {
    format!("This is your text: {}", text)
}

#[post("/v1/req_gpt", format = "json", data = "<gpt_prompt>")]
pub async fn send_rq(gpt_prompt: Json<GptPrompt>) -> String {
    let mut gpt_instance = GPT::default();
    gpt_instance.prompt = gpt_prompt.prompt.to_owned();
    let client = reqwest::Client::new();
    let api_key: String = env::var("OPENAI_KEY").unwrap();
    let api_header = format!("Bearer {}", api_key);
    let response = client
        .post("https://api.openai.com/v1/completions")
        .header(AUTHORIZATION, api_header)
        .header(CONTENT_TYPE, "application/json")
        .json(&gpt_instance)
        .send()
        .await
        .unwrap()
        .json::<GptResponse>()
        .await;

    match response {
        Ok(text) => format!("{:?}", text.choices[0].text),
        Err(error) => format!("{:?}", error),
    }
}
