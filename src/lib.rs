pub mod schemas;
mod tokenisers;
mod utils;
mod v1;

use axum::routing::get;
use axum::Router;
use utoipa::OpenApi;

#[utoipauto::utoipauto]
#[derive(OpenApi)]
#[openapi(info(description = "An API for tokenising strings"))]
struct ApiSpecification;

fn on_startup() {
    _ = &*tokenisers::LLAMA3_TOKENISER;
}

pub fn app() -> Router {
    on_startup();

    Router::new().route("/", get({})).merge(v1::router()).merge(
        utoipa_swagger_ui::SwaggerUi::new("/docs")
            .url("/api-docs/openapi.json", ApiSpecification::openapi()),
    )
}
