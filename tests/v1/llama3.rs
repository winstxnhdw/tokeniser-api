use crate::{get_client, v1::API_VERSION};
use anyhow::Result;
use tokeniser_api::schemas;

#[tokio::test]
pub async fn encode() -> Result<()> {
    let client = get_client();
    let response: schemas::EncodeResponseSchema = client
        .post(format!("/api/{API_VERSION}/llama3/encode").as_str())
        .json(&schemas::EncodeRequestSchema {
            text: "Hello, world!".to_string(),
        })
        .await
        .json();

    assert_eq!(response.tokens, ["Hello", ",", "Ġworld", "!"]);
    Ok(())
}

#[tokio::test]
pub async fn decode() -> Result<()> {
    let client = get_client();
    let response: schemas::DecodeResponseSchema = client
        .post(format!("/api/{API_VERSION}/llama3/decode").as_str())
        .json(&schemas::DecodeRequestSchema {
            tokens: [0].to_vec(),
        })
        .await
        .json();

    assert_eq!(response.text, "!");
    Ok(())
}
