mod v1;

use axum::routing::get;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(v1::index::index), info(description = "An API"))]
struct ApiSpecification;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = axum::Router::new().route("/v1/", get(v1::index)).merge(
        utoipa_swagger_ui::SwaggerUi::new("/docs")
            .url("/api-docs/openapi.json", ApiSpecification::openapi()),
    );

    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| String::from("5555"));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app.into_make_service()).await
}
