pub mod index;
pub mod llama3;

use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/v1", get(index::index))
        .route("/v1/llama3/encode", post(llama3::encode))
        .route("/v1/llama3/decode", post(llama3::decode))
}
