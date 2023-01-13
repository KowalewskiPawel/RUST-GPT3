use crate::appconfig;
use sqlite::State;
use reqwest;
use reqwest::header::*;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestId {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddKey {
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keys {
    key: String,
    tokens_left: String
}
impl Default for Keys {
    fn default() -> Keys {
        Keys {
            key: String::new(),
            tokens_left: String::new()
        }
    }
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

#[post("/v1/add_key", format = "json", data = "<add_key>")]
pub fn create_key(add_key: Json<AddKey>) -> String {
    let conn = sqlite::open(appconfig::DATABASE_FILE).expect("Database not readable!");
    
    let admin_key = add_key.password.to_owned();
    let api_key: String = env::var("ADMIN_KEY").unwrap();

    if admin_key != api_key {
        return "Wrong admin key!".to_string();
    }
    let id = Uuid::new_v4();

    let result: String = "SUCCESS".to_string();
    let _statement = match conn.execute(format!(
        "INSERT INTO keys values ('{}', '{}')", id, 10 )) {
        Ok(statement) => statement,
        Err(e) => return format!("Problem running query: {:?}", e),
    };

    result
}

#[get("/v1/query_all", format = "json", data = "<admin_key>")]
pub fn query_all(admin_key: Json<AddKey>) -> Json<Vec<Keys>> {
    appconfig::check_dbfile(appconfig::DATABASE_FILE);

    let conn = sqlite::open(appconfig::DATABASE_FILE).expect("Database not readable!"); //we can unwrap we checked the file exists
    let admin_key = admin_key.password.to_owned();
    let api_key: String = env::var("ADMIN_KEY").unwrap();
    let mut result: Vec<Keys> = Vec::new();

    if admin_key != api_key {
        result.push(Keys { key: "Wrong Admin key!".to_string(), tokens_left: "0".to_string() });
        return Json(result);
    }

    

    let query = "SELECT * FROM keys";

    let mut keys_new = Keys::default();

    conn.iterate(query, |pairs| {
        for &(name, value) in pairs.iter() {
            let current_value = value.unwrap();
            match name {
                "id" => keys_new.key = String::from(current_value),
                "left" => {
                    keys_new.tokens_left = String::from(current_value);
                    let keys_copy = Keys {
                        key: keys_new.key.clone(),
                        tokens_left: keys_new.tokens_left.clone(),
                    };
                    result.push(keys_copy);
                }
                &_ => (),
            }
        }
        true
    })
    .unwrap();

    Json(result)
}