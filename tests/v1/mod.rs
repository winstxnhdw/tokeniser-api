use anyhow::Result;
use httpc_test::Client;

pub mod index;
pub const API_VERSION: &str = "v1";

pub async fn get_client() -> Result<Client> {
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| String::from("5555"));
    let client = httpc_test::new_client(format!("http://localhost:{port}"));

    Ok(client.unwrap())
}
