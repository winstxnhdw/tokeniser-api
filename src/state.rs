use tokenizers::tokenizer::Tokenizer;

#[derive(Clone)]
pub struct AppState {
    pub llama3_tokeniser: Tokenizer,
}
