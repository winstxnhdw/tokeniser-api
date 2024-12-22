use tokenizers::tokenizer::Tokenizer;

pub fn encode(tokeniser: Tokenizer, text: String, add_special_tokens: bool) -> Option<Vec<String>> {
    let tokens = tokeniser
        .encode(text, add_special_tokens)
        .ok()?
        .get_tokens()
        .to_vec();

    Some(tokens)
}

pub fn decode(tokeniser: Tokenizer, tokens: &[u32], skip_special_tokens: bool) -> Option<String> {
    tokeniser.decode(tokens, skip_special_tokens).ok()
}
