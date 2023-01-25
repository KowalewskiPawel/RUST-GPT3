use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo_net::http::{Request, Headers};
use serde::{Deserialize, Serialize};
use js_sys::JsString;

#[derive(PartialEq, Debug, Clone)]
struct InputPrompt {
    key : String,
    prompt : String    
}

#[derive(Properties, PartialEq)]
struct RequestResultProps {
    request_result: String
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RequestGpt {
    pub prompt: String,
    pub key: String,
}

impl RequestGpt {
    pub fn default() -> Self {
        Self {
            prompt: String::new(),
            key: String::new(),
        }
    }
}


#[function_component(App)]
fn app() -> Html {
    let input_key_ref = NodeRef::default();
    let input_prompt_ref = NodeRef::default();

    let input_key = input_key_ref.clone();
    let input_prompt = input_prompt_ref.clone();
    let request_result = use_state(|| "No result".to_string());
    let _request_result = request_result.clone();
    let onclick = Callback::from(move |_| {
            let key = input_key.cast::<HtmlInputElement>().unwrap().value();
            let prompt = input_prompt.cast::<HtmlInputElement>().unwrap().value();
            let mut new_body = RequestGpt::default();

            new_body.prompt = prompt.to_string();
            new_body.key = key.to_string();
        
            let body = serde_json::to_string(&new_body).unwrap();
        
            let request_result = request_result.clone();
            let req_body = body.clone();

            let headers = Headers::new();
            headers.append("Content-Type", "application/json");
            wasm_bindgen_futures::spawn_local(async move {
                let backend_url = "http://127.0.0.1:8000/api/v1/req_gpt".to_string();
                let backend_msg = Request::post(&backend_url).body(req_body).headers(headers).send().await.unwrap().text().await.unwrap();                
                web_sys::console::log_1(&JsString::from(backend_msg.clone()));
                request_result.set(backend_msg);
            });
        });

    html!{
        <div>
            <h2>{"Key"}</h2>
            <input ref = {input_key_ref} type="text"/>
            <h2>{"Prompt"}</h2>
            <input ref = {input_prompt_ref} type="text"/>
            <button onclick={onclick}>{"request"}</button>
            <h3>{"Result: "}</h3>
            <p>{format!("{:?}", *_request_result)}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}