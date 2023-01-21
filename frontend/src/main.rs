use yew::prelude::*;
use reqwasm::http::Request;
use web_sys::HtmlInputElement;
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

#[function_component(App)]
fn app() -> Html {
    let input_key_ref = NodeRef::default();
    let input_prompt_ref = NodeRef::default();

    let input_key = input_key_ref.clone();
    let input_prompt = input_prompt_ref.clone();

    let request_result = use_state(|| None);
    let _request_result = request_result.clone();
    let onclick = Callback::from(move |_| {
            let key = input_key.cast::<HtmlInputElement>().unwrap().value();
            let prompt = input_prompt.cast::<HtmlInputElement>().unwrap().value();
            let request_result = request_result.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let backend_url = "http://127.0.0.1:8000/api/v1/req_gpt".to_string();
                let backend_msg = Request::get(&backend_url).send().await.unwrap().text().await.unwrap();                
                web_sys::console::log_1(&JsString::from(backend_msg.clone()));
                if backend_msg == String::from("welcome!") {
                    request_result.set(Some(true));
                }
                else {
                    request_result.set(Some(false));
                }
            });
        });

    html!{
        <div>
            <p>{"key"}</p>
            <input ref = {input_key_ref} type="text"/>
            <p>{"password"}</p>
            <input ref = {input_prompt_ref} type="text"/>
            <button onclick={onclick}>{"request"}</button
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}