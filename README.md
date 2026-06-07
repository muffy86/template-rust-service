# template-rust-service

A Rust + Axum + Tokio service template, with CI delegated to
[muffy86/infra-automation](https://github.com/muffy86/infra-automation).

## Quick start

```bash
cargo run
curl http://localhost:8080/health
# {"status":"ok","service":"template-rust-service","version":"0.1.0"}
```

## What's wired up

- `Cargo.toml` — axum 0.7, tokio, tracing, tower-http
- `src/main.rs` — `/` and `/health` endpoints
- `src/ai/mod.rs` — provider-agnostic AI client (Ollama/OpenAI/Anthropic/Venice/Groq/OpenRouter)
- `tests/integration.rs` — smoke test
- CI delegates to `muffy86/infra-automation/.github/workflows/ci-rust.yml@main`
