use super::LLAMA3_TOKENISER;

pub fn encode(text: String, add_special_tokens: bool) -> Option<Vec<String>> {
    let tokens = LLAMA3_TOKENISER
        .encode(text, add_special_tokens)
        .ok()?
        .get_tokens()
        .to_vec();

    Some(tokens)
}

pub fn decode(tokens: &[u32], skip_special_tokens: bool) -> Option<String> {
    LLAMA3_TOKENISER.decode(tokens, skip_special_tokens).ok()
}
