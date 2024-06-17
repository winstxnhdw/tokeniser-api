pub mod index;
pub mod llama3;

use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/v1", get(index::index))
        .route("/v1/llama3/encode", post(llama3::llama3))
}
