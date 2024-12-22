use crate::schemas::{
    DecodeRequestQueryParameter, DecodeRequestSchema, DecodeResponseSchema,
    EncodeRequestQueryParameter, EncodeRequestSchema, EncodeResponseSchema, ErrorResponseSchema,
};

use axum::extract::{Query, State};
use axum::{response, Json};

use crate::state::AppState;
use crate::tokenisers;

#[utoipa::path(post, path = "/api/v1/llama3/decode", responses(
    (status = 200, body = DecodeResponseSchema),
    (status = 400, body = ErrorResponseSchema)
))]
pub async fn decode(
    State(state): State<AppState>,
    Query(query_parameters): Query<DecodeRequestQueryParameter>,
    Json(request): Json<DecodeRequestSchema>,
) -> impl response::IntoResponse {
    let response = tokenisers::llama3::decode(
        state.llama3_tokeniser,
        &request.tokens,
        query_parameters.skip_special_tokens,
    );

    crate::utils::build_response(
        response.map(|text| DecodeResponseSchema { text }),
        "Failed to decode tokens!",
    )
}

#[utoipa::path(post, path = "/api/v1/llama3/encode", responses(
    (status = 200, body = DecodeResponseSchema),
    (status = 400, body = ErrorResponseSchema)
))]
pub async fn encode(
    State(state): State<AppState>,
    Query(query_parameters): Query<EncodeRequestQueryParameter>,
    Json(request): Json<EncodeRequestSchema>,
) -> impl response::IntoResponse {
    let response = tokenisers::llama3::encode(
        state.llama3_tokeniser,
        request.text,
        query_parameters.add_special_tokens,
    );

    crate::utils::build_response(
        response.map(|tokens| EncodeResponseSchema { tokens }),
        "Failed to encode!",
    )
}
