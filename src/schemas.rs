use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct EncodeResponseSchema {
    #[schema(example = r#"["Hello", ",", "Ġworld", "!"]"#)]
    pub tokens: Vec<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct EncodeRequestSchema {
    #[schema(example = "Hello, world!")]
    pub text: String,
}

#[derive(Deserialize, ToSchema)]
pub struct EncodeRequestQueryParameter {
    #[serde(default)]
    pub add_special_tokens: bool,
}

#[derive(Serialize, ToSchema)]
pub struct DecodeResponseSchema {
    #[schema(example = "Hello, world!")]
    pub text: String,
}

#[derive(Deserialize, ToSchema)]
pub struct DecodeRequestSchema {
    pub tokens: Vec<u32>,
}

#[derive(Deserialize, ToSchema)]
pub struct DecodeRequestQueryParameter {
    #[serde(default)]
    pub skip_special_tokens: bool,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponseSchema {
    #[schema(example = "Failed to decode!")]
    pub error: String,
}
