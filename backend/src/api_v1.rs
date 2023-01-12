
#[get("/v1/test/<text>")]
pub fn test(text: String) -> String {
    format!("This is test: {}", text)
}