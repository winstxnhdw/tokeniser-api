use crate::schemas::{DecodeRequestQueryParameter, DecodeRequestSchema};
use axum::extract::Query;
use axum::{response, Json};

#[utoipa::path(post, path = "/v1/llama3/decode", responses(
    (status = 200, body = DecodeResponseSchema),
    (status = 400, body = ErrorResponseSchema)
))]
pub async fn decode(
    Query(query_parameters): Query<DecodeRequestQueryParameter>,
    Json(request): Json<DecodeRequestSchema>,
) -> impl response::IntoResponse {
    crate::utils::build_response(
        crate::tokenisers::llama3::decode(&request.tokens, query_parameters.skip_special_tokens),
        "Failed to decode tokens!",
    )
}
