use std::net::SocketAddr;

#[get("/v1/test/<text>")]
pub fn test(remote_addr: SocketAddr, text: String) -> String {
    format!("This is your text: {:?} and text: {}", remote_addr, text)
}