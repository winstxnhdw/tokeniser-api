pub mod llama3;

use std::sync::LazyLock;
use tokenizers::tokenizer::Tokenizer;

pub static LLAMA3_TOKENISER: LazyLock<Tokenizer> =
    LazyLock::new(|| Tokenizer::from_pretrained("winstxnhdw/llama3-tokeniser", None).unwrap());
