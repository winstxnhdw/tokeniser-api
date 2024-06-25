# tokeniser-api

[![main.yml](https://github.com/winstxnhdw/tokeniser-api/actions/workflows/main.yml/badge.svg)](https://github.com/winstxnhdw/tokeniser-api/actions/workflows/main.yml)
[![deploy.yml](https://github.com/winstxnhdw/tokeniser-api/actions/workflows/deploy.yml/badge.svg)](https://github.com/winstxnhdw/tokeniser-api/actions/workflows/deploy.yml)
[![warmer.yml](https://github.com/winstxnhdw/tokeniser-api/actions/workflows/warmer.yml/badge.svg)](https://github.com/winstxnhdw/tokeniser-api/actions/workflows/warmer.yml)
[![dependabot.yml](https://github.com/winstxnhdw/tokeniser-api/actions/workflows/dependabot.yml/badge.svg)](https://github.com/winstxnhdw/tokeniser-api/actions/workflows/dependabot.yml)

[![Open in Spaces](https://huggingface.co/datasets/huggingface/badges/raw/main/open-in-hf-spaces-md-dark.svg)](https://huggingface.co/spaces/winstxnhdw/tokeniser-api)
[![Open a Pull Request](https://huggingface.co/datasets/huggingface/badges/raw/main/open-a-pr-md-dark.svg)](https://github.com/winstxnhdw/tokeniser-api/compare)

A high-performance [axum](https://github.com/tokio-rs/axum) API for serving [Hugging Face's Tokenizers](https://crates.io/crates/tokenizers).

## Usage

If your tokeniser is not listed below, please open an issue or a pull request.

### Meta LLama 3

To encode, simply cURL the endpoint like in the following.

```bash
curl -O 'https://winstxnhdw-tokeniser-api.hf.space/v1/llama3/encode' \
     -H 'Content-Type: application/json' \
     -d '{ "text": "Hello, world!" }'
```

To decode, simply cURL the endpoint like in the following.

```bash
curl -O 'https://winstxnhdw-tokeniser-api.hf.space/v1/llama3/decode' \
     -H 'Content-Type: application/json' \
     -d '{ "tokens": [ 0 ] }'
```
