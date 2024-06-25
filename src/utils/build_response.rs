use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::to_string;

pub fn build_response<T: Serialize>(body: Option<T>, error_message: &str) -> impl IntoResponse {
    match body {
        Some(body) => Response::builder()
            .status(200)
            .header(CONTENT_TYPE, "application/json")
            .body(to_string(&body).unwrap())
            .unwrap(),
        _ => Response::builder()
            .status(400)
            .header(CONTENT_TYPE, "application/json")
            .body(
                to_string(&crate::schemas::ErrorResponseSchema {
                    error: error_message.to_string(),
                })
                .unwrap(),
            )
            .unwrap(),
    }
}
