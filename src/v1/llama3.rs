use crate::{
    schemas::{RequestSchema, ResponseSchema},
    tokenisers::llama3::llama3_encode,
};
use axum::{response, Json};

#[utoipa::path(post, path = "/v1/llama3/encode", responses((status = 200, body = ResponseSchema)))]
pub async fn llama3(Json(request): Json<RequestSchema>) -> impl response::IntoResponse {
    let tokens = match llama3_encode(request.text) {
        Some(tokens) => ResponseSchema { tokens },
        None => ResponseSchema { tokens: vec![] },
    };

    response::Response::builder()
        .status(200)
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&tokens).unwrap())
        .unwrap()
}
