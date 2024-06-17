use super::LLAMA3_TOKENISER;

pub fn llama3_encode(text: String) -> Option<Vec<String>> {
    let tokens = LLAMA3_TOKENISER
        .encode(text, false)
        .ok()?
        .get_tokens()
        .to_vec();

    Some(tokens)
}
