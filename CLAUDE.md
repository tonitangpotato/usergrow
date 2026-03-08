# CLAUDE.md — UserGrow Project

## Project
GEO (Generative Engine Optimization) agent: measures brand visibility in LLM-generated responses.

## Structure
```
usergrow/          Rust binary (tokio async)
├── src/
│   ├── main.rs     CLI entrypoint (clap)
│   ├── tavily.rs   Tavily Search/Extract client
│   ├── query_gen.rs Query generation (template + industry-aware)
│   ├── llm_probe.rs LLM probing engine (OpenAI, Gemini)
│   ├── analysis.rs  SOM metrics, sentiment, insights
│   └── report.rs   HTML dashboard generation
```

## Build & Test
```bash
cd usergrow
cargo build
cargo test
cargo clippy -- -D warnings
```

## Conventions
- **No `mod.rs` files** — use `foo.rs` alongside `foo/` (Rust 2018+ style)
- **No hardcoded credentials** — env vars only (`TAVILY_API_KEY`, `OPENAI_API_KEY`, `GEMINI_API_KEY`)
- **Error handling** — use `anyhow::Result`, `.context()` for actionable messages
- **Async** — tokio runtime, reqwest for HTTP

## Key Types
- `Query` / `QueryCategory` — categorized customer queries (5 types)
- `ProbeResult` — raw LLM response + metadata
- `BrandMention` — parsed mention with position, sentiment
- `AnalysisResult` — full SOM report with insights
- `TavilyClient` — web search and content extraction

## Testing
- Unit tests in each module (`#[cfg(test)]`)
- Integration tests need API keys (skip with `#[ignore]`)
- `cargo test` runs all non-ignored tests
