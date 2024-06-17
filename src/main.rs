mod schemas;
mod tokenisers;
mod v1;

use utoipa::OpenApi;

#[utoipauto::utoipauto]
#[derive(OpenApi)]
#[openapi(info(description = "An API for tokenising strings"))]
struct ApiSpecification;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = axum::Router::new().merge(v1::router()).merge(
        utoipa_swagger_ui::SwaggerUi::new("/docs")
            .url("/api-docs/openapi.json", ApiSpecification::openapi()),
    );

    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| String::from("5555"));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    axum::serve(listener, app.into_make_service()).await
}
