# UserGrow — AI Brand Visibility Intelligence

**Know how AI sees your brand.** UserGrow analyzes brand visibility across major AI models (ChatGPT, Claude, Gemini, GLM) and provides actionable optimization strategies.

## Features

- 🔍 **Multi-Model Probing** — Query 5 AI models simultaneously to measure brand mention rates
- 📊 **Visibility Scoring** — Composite score (0-20) based on share-of-voice across models
- 🧠 **Knowledge Graph** — Entity-relation mapping of brands, industries, keywords, competitors
- 🌐 **D3 Visualization** — Interactive force-directed graph of the knowledge graph
- 🎯 **Persona Analysis** — Understand which user personas trigger brand mentions
- 📈 **GEO Recommendations** — Generative Engine Optimization strategies

## Architecture

- **Backend**: Rust / Axum — high-performance async API server
- **Frontend**: Vanilla JS + D3.js — lightweight, fast
- **Memory**: SQLite via [ironclaw-engram](./ironclaw-engram/) — cognitive memory with ACT-R activation
- **Knowledge Graph**: Structured entity/relation/snapshot storage with auto-ingestion

## Quick Start

```bash
# Set API keys
export OPENAI_API_KEY=...
export ANTHROPIC_API_KEY=...
export GEMINI_API_KEY=...
export GLM_API_KEY=...
export TAVILY_API_KEY=...

# Build and run
cd usergrow
cargo run
```

Server starts at `http://localhost:3000`

## API Endpoints

| Endpoint | Description |
|----------|-------------|
| `POST /api/analyze` | Analyze a brand's AI visibility |
| `GET /api/reports` | List all analysis reports |
| `GET /api/kg/graph` | Full knowledge graph (entities + relations) |
| `GET /api/kg/entities/{type}` | Entities by type (brand, industry, keyword...) |
| `GET /api/kg/entity/{id}/relations` | Relations for an entity |

## Live Demo

🌐 [usergrow.net](https://usergrow.net)

## License

MIT
