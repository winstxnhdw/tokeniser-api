use crate::schemas::{
    DecodeRequestQueryParameter, DecodeRequestSchema, DecodeResponseSchema,
    EncodeRequestQueryParameter, EncodeRequestSchema, EncodeResponseSchema,
};
use axum::extract::Query;
use axum::{response, Json};

#[utoipa::path(post, path = "/api/v1/llama3/decode", responses(
    (status = 200, body = DecodeResponseSchema),
    (status = 400, body = ErrorResponseSchema)
))]
pub async fn decode(
    Query(query_parameters): Query<DecodeRequestQueryParameter>,
    Json(request): Json<DecodeRequestSchema>,
) -> impl response::IntoResponse {
    let response =
        crate::tokenisers::llama3::decode(&request.tokens, query_parameters.skip_special_tokens)
            .map(|text| DecodeResponseSchema { text });

    crate::utils::build_response(response, "Failed to decode tokens!")
}

#[utoipa::path(post, path = "/api/v1/llama3/encode", responses(
    (status = 200, body = DecodeResponseSchema),
    (status = 400, body = ErrorResponseSchema)
))]
pub async fn encode(
    Query(query_parameters): Query<EncodeRequestQueryParameter>,
    Json(request): Json<EncodeRequestSchema>,
) -> impl response::IntoResponse {
    let response =
        crate::tokenisers::llama3::encode(request.text, query_parameters.add_special_tokens)
            .map(|tokens| EncodeResponseSchema { tokens });

    crate::utils::build_response(response, "Failed to encode!")
}
