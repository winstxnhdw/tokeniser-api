use crate::schemas::{EncodeRequestQueryParameter, EncodeRequestSchema};
use axum::extract::Query;
use axum::{response, Json};

#[utoipa::path(post, path = "/v1/llama3/encode", responses(
    (status = 200, body = DecodeResponseSchema),
    (status = 400, body = ErrorResponseSchema)
))]
pub async fn encode(
    Query(query_parameters): Query<EncodeRequestQueryParameter>,
    Json(request): Json<EncodeRequestSchema>,
) -> impl response::IntoResponse {
    crate::utils::build_response(
        crate::tokenisers::llama3::encode(request.text, query_parameters.add_special_tokens),
        "Failed to encode!",
    )
}
