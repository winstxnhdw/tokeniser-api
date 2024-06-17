use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
pub struct ResponseSchema {
    pub tokens: Vec<String>,
}

#[derive(serde::Deserialize, ToSchema)]
pub struct RequestSchema {
    #[schema(example = "Hello, world!")]
    pub text: String,
}
