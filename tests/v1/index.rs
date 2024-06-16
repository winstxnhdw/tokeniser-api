use crate::v1::{get_client, API_VERSION};
use anyhow::Result;

#[tokio::test]
async fn index() -> Result<()> {
    let client = get_client().await?;
    let response = client.do_get(format!("/{API_VERSION}/").as_str()).await?;

    assert_eq!(
        response.text_body().unwrap(),
        format!("Welcome to {API_VERSION} of the API!")
    );

    Ok(())
}
