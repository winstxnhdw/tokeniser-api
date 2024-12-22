pub mod schemas;
mod state;
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

pub fn app() -> Router {
    let root_path = "/api";
    let shared_state = state::AppState {
        llama3_tokeniser: tokenizers::tokenizer::Tokenizer::from_pretrained(
            "winstxnhdw/llama3-tokeniser",
            None,
        )
        .unwrap(),
    };

    Router::new()
        .nest(
            root_path,
            Router::new()
                .route("/", get(()))
                .merge(v1::router(shared_state)),
        )
        .merge(
            utoipa_swagger_ui::SwaggerUi::new(format!("{}/docs", root_path))
                .url("/api-docs/openapi.json", ApiSpecification::openapi()),
        )
}
