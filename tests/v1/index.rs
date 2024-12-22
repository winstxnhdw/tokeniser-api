use crate::v1::API_VERSION;

#[tokio::test]
pub async fn index() -> anyhow::Result<()> {
    let client = crate::get_client();
    let response = client
        .get(format!("/api/{API_VERSION}").as_str())
        .await
        .text();

    assert_eq!(response, format!("Welcome to {API_VERSION} of the API!"));

    Ok(())
}
