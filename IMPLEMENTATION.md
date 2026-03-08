# Implementation Plan

## Team Assignment
- **Frontend**: Xenos, Yurica
- **Backend**: potato oneB, Publius (parallel development)
- **AI Assistant**: Architecture, scaffolding, integration

---

## 🖥️ Frontend Implementation Plan

**Tech**: Single-page HTML + Tailwind CSS + Chart.js + D3.js  
**Deploy**: Vercel (https://usergrow-two.vercel.app)  
**Source**: `docs/index.html` (can split into multiple files later)

### F1. Search Page (Hero)
- [x] Brand name input + Analyze button (done)
- [ ] Add **Industry dropdown** (Healthcare, Finance, Legal, Restaurant, SaaS, Education, Retail, Other)
- [ ] Add **City input** (optional, with autocomplete if time permits)
- [ ] Add **Target Group selector** (optional: dropdown of persona categories)
- [ ] Connect search button to `POST /api/analyze` backend

### F2. Loading/Progress Screen
- [ ] SSE listener: connect to `GET /api/analyze/stream/{job_id}`
- [ ] Animated progress steps:
  - "🎭 Generating persona probes..." 
  - "🤖 Probing ChatGPT..." / "Probing Gemini..." / "Probing Perplexity..."
  - "📊 Analyzing responses..."
  - "🔍 Gathering real-world data..."
  - "🧠 Consulting memory..."
- [ ] Radar/scan animation during loading (CSS keyframes)
- [ ] Each step flips to ✅ when SSE reports completion

### F3. Tab 1 — AI Visibility Score
- [ ] Large animated number (0→score counter animation)
- [ ] Color-coded: red (<30), orange (30-60), green (60+)
- [ ] Text label: "Critical" / "Needs Work" / "Good" / "Excellent"
- [ ] **Share of Model bar chart** (Chart.js horizontal bars)
  - One bar per model (ChatGPT / Gemini / Perplexity)
  - Show percentage + color

### F4. Tab 2 — Persona Heatmap 🎭
- [ ] Grid/table: rows = personas, columns = model × language
- [ ] Each cell: color-coded (green/yellow/red/gray) + rank or "—"
- [ ] Hover shows detail: "Maria asked in Spanish → ChatGPT recommended competitors X, Y, Z — you were not mentioned"
- [ ] **Target Group highlight**: if user selected target group, highlight those rows with a border/badge

### F5. Tab 3 — Brand DNA 🧬
- [ ] **D3.js force-directed graph** (most visually impressive)
  - Center node = your brand
  - Surrounding nodes = keywords AI associates with you
  - Node size = frequency
  - Color = sentiment (green positive, red negative, gray neutral)
- [ ] Side-by-side: your DNA vs top competitor's DNA
- [ ] Fallback: word cloud if D3 is too complex

### F6. Tab 4 — Reality Check ⭐
- [ ] Side-by-side comparison:
  - Left: "What AI Recommends" (ranked list with bars)
  - Right: "Real-World Ratings" (from Tavily data)
- [ ] **AI Bias Score**: big number showing divergence
- [ ] Highlight mismatches in red: "Rated 4.9★ but AI ranks you #8"

### F7. Tab 5 — Recommendations 💡
- [ ] Numbered list of actionable recommendations
- [ ] Each tagged by urgency: 🔴 Critical / 🟡 Important / 🟢 Nice-to-have
- [ ] Each linked to the evidence (which tab/data point generated this recommendation)

### F8. General UI
- [x] Particle background (done)
- [x] Glassmorphism dark theme (done)
- [ ] Tab navigation (sticky top bar within report section)
- [ ] Mobile responsive
- [ ] "Download PDF" button (stretch goal)
- [ ] Engram badge: "🧠 Agent has analyzed X brands. Confidence: High"

### Frontend API Contract

```typescript
// POST /api/analyze
Request: {
  brand: string,
  industry: string,
  city?: string,
  target_group?: string  // persona category name
}
Response: { job_id: string }

// GET /api/analyze/stream/{job_id}  (SSE)
Events:
  { step: "persona_gen", status: "complete", count: 24 }
  { step: "probe_gpt", status: "progress", done: 8, total: 24 }
  { step: "probe_gemini", status: "complete" }
  { step: "analysis", status: "complete" }
  { step: "reality_check", status: "complete" }
  { step: "done", data: <FullReport JSON> }

// FullReport JSON shape:
{
  brand: string,
  industry: string,
  city: string,
  visibility_score: number,          // 0-100
  share_of_model: {
    chatgpt: number,                 // 0-100 percentage
    gemini: number,
    perplexity: number
  },
  persona_heatmap: [
    {
      persona: { name, age, background, language },
      results: {
        "chatgpt_en": { mentioned: bool, rank: number|null },
        "chatgpt_zh": { mentioned: bool, rank: number|null },
        "gemini_en": ...,
        ...
      }
    }
  ],
  brand_dna: {
    your_brand: { keyword: string, score: number, sentiment: string }[],
    competitors: {
      [name: string]: { keyword: string, score: number, sentiment: string }[]
    }
  },
  reality_check: {
    your_rating: { source: string, score: number, review_count: number },
    ai_rank: number,
    competitors: [{ name, ai_rank, real_rating }],
    bias_score: number
  },
  recommendations: [
    { priority: "critical"|"important"|"nice", text: string, evidence: string }
  ],
  engram: {
    total_scans: number,
    drift: { direction: "up"|"down"|"stable", delta: number }|null,
    confidence: string
  }
}
```

---

## ⚙️ Backend Implementation Plan

**Tech**: Rust (tokio + axum + reqwest + serde)  
**Source**: `usergrow/src/`  
**Existing**: 1798 lines across 6 modules

### Module Dependency Graph

```
                    ┌──────────┐
                    │  web.rs  │ ← HTTP layer (axum)
                    └────┬─────┘
                         │
                    ┌────┴─────┐
                    │ main.rs  │ ← orchestrator
                    └────┬─────┘
                         │
          ┌──────────────┼──────────────┐
          │              │              │
    ┌─────┴──────┐ ┌────┴─────┐ ┌─────┴──────┐
    │persona.rs  │ │tavily.rs │ │ engram.rs  │
    │(new)       │ │(done ✅) │ │ (new)      │
    └─────┬──────┘ └────┬─────┘ └─────┬──────┘
          │              │              │
    ┌─────┴──────┐       │              │
    │query_gen.rs│←──────┘              │
    │(refactor)  │                      │
    └─────┬──────┘                      │
          │                             │
    ┌─────┴──────┐                      │
    │llm_probe.rs│                      │
    │(refactor)  │                      │
    └─────┬──────┘                      │
          │                             │
    ┌─────┴──────────────┐              │
    │  analysis.rs       │              │
    │  + brand_dna.rs    │              │
    │  + reality.rs      │──────────────┘
    │  (refactor + new)  │
    └────────────────────┘
```

### 🔀 Parallel Development Plan

**Key insight**: Modules that share no types/imports can be built simultaneously.

---

#### 🅰️ potato oneB 的模块 (Pipeline核心)

##### B1. `persona.rs` — 新模块 ⭐ 优先级最高
无依赖，可立即开始。

```rust
// persona.rs — 定义persona + 生成prompt

pub struct Persona {
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub ethnicity: String,
    pub income_bracket: String,
    pub immigration_status: String,
    pub language: Language,
    pub occupation: String,
}

pub enum Language {
    English,
    Spanish,
    Chinese,
    Hindi,
}

/// 返回预定义的Census-backed persona池
pub fn default_personas() -> Vec<Persona> { ... }

/// 为一个persona生成probe prompt
pub fn build_prompt(persona: &Persona, service_type: &str, city: &str) -> String { ... }

/// 生成完整的probe矩阵: personas × languages × base_queries
pub fn build_probe_matrix(
    personas: &[Persona],
    brand: &str,
    industry: &str,
    city: &str,
) -> Vec<PersonaProbe> { ... }

pub struct PersonaProbe {
    pub persona: Persona,
    pub prompt: String,
    pub language: Language,
}
```

##### B2. `query_gen.rs` — 改造
依赖: persona.rs 的类型  
改造点: 接收 `PersonaProbe` 列表，为每个probe生成完整query

##### B3. `llm_probe.rs` — 改造
依赖: query_gen 类型  
改造点:
- 并发执行 (`tokio::spawn` / `futures::join_all`)
- 添加进度回调 (用 `tokio::sync::mpsc` 发SSE事件)
- 添加 Perplexity API

---

#### 🅱️ Publius 的模块 (Web层 + 分析层)

##### B4. `web.rs` — 新模块 ⭐ 优先级最高
无依赖核心pipeline，可立即开始。

```rust
// web.rs — axum HTTP server

use axum::{Router, routing::{get, post}, Json, extract::State};
use tokio::sync::mpsc;

pub struct AppState {
    pub tavily_key: String,
    pub openai_key: Option<String>,
    pub gemini_key: Option<String>,
    pub perplexity_key: Option<String>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/analyze", post(start_analysis))
        .route("/api/analyze/stream/:job_id", get(stream_progress))
        .route("/api/history/:brand", get(get_history))
        .nest_service("/", tower_http::services::ServeDir::new("static"))
        .with_state(state)
}

// POST /api/analyze → spawn analysis task, return job_id
// GET /api/analyze/stream/:job_id → SSE stream
// GET /api/health → { status, version }
```

##### B5. `brand_dna.rs` — 新模块
依赖: `ProbeResult` 类型 (from llm_probe.rs, 但只需要类型定义)

```rust
// brand_dna.rs — 关键词提取 + 语义图谱

pub struct BrandDna {
    pub brand: String,
    pub keywords: Vec<KeywordScore>,
}

pub struct KeywordScore {
    pub keyword: String,
    pub frequency: usize,
    pub tfidf_score: f64,
    pub sentiment: Sentiment,
}

/// 从LLM responses中提取品牌关联关键词
pub fn extract_dna(brand: &str, responses: &[ProbeResult]) -> BrandDna { ... }

/// 对比两个品牌的DNA
pub fn compare_dna(brand_a: &BrandDna, brand_b: &BrandDna) -> DnaComparison { ... }
```

##### B6. `reality.rs` — 新模块
依赖: `tavily.rs` (已完成)

```rust
// reality.rs — Tavily真实数据对比

pub struct RealityCheck {
    pub brand_rating: Option<RatingData>,
    pub competitor_ratings: Vec<(String, RatingData)>,
    pub ai_vs_reality_bias: f64,
}

pub struct RatingData {
    pub source: String,
    pub score: f64,
    pub review_count: u32,
}

/// 用Tavily搜品牌真实评分
pub async fn check_reality(
    tavily: &TavilyClient,
    brand: &str,
    competitors: &[String],
    industry: &str,
    city: &str,
) -> Result<RealityCheck> { ... }
```

##### B7. `engram.rs` — 新模块
无Rust依赖（调subprocess）

```rust
// engram.rs — Engram memory integration

pub struct EngramClient { db_path: String }

impl EngramClient {
    /// 存储扫描结果
    pub async fn store_scan(&self, brand: &str, result: &serde_json::Value) -> Result<()> { ... }
    
    /// 查询历史扫描
    pub async fn recall_history(&self, brand: &str) -> Result<Vec<serde_json::Value>> { ... }
    
    /// 检测drift
    pub async fn detect_drift(&self, brand: &str, current: &serde_json::Value) -> Result<Option<Drift>> { ... }
    
    /// 内部: 调用 neuromem CLI
    async fn run_cmd(&self, args: &[&str]) -> Result<String> { ... }
}
```

---

#### 🔗 B8. `main.rs` — 最后整合 (合并后一起改)
改造: CLI入口 → 启动axum server  
依赖: web.rs + 所有其他模块  
**在两人的模块都完成后一起整合**

---

### ⏱ 并行开发时间线

```
Hour 1-2:
  potato oneB → B1 persona.rs (无依赖，立即开始)
  Publius    → B4 web.rs (无依赖，立即开始)

Hour 2-3:  
  potato oneB → B2 query_gen.rs 改造 (依赖B1)
  Publius    → B5 brand_dna.rs + B6 reality.rs (只依赖类型定义)

Hour 3-4:
  potato oneB → B3 llm_probe.rs 改造 (并发+SSE回调)
  Publius    → B7 engram.rs

Hour 4-5:
  一起       → B8 main.rs 整合 + 联调
```

### 共享类型约定 (先定义好，两人都用)

两人开始前先在 `types.rs` 定义共享类型：

```rust
// types.rs — 共享数据结构

use serde::{Deserialize, Serialize};

/// SSE进度事件
#[derive(Serialize)]
pub struct ProgressEvent {
    pub step: String,
    pub status: String,  // "progress" | "complete" | "error"
    pub detail: Option<String>,
    pub done: Option<u32>,
    pub total: Option<u32>,
}

/// 最终分析报告 (返回给前端的JSON)
#[derive(Serialize)]
pub struct FullReport {
    pub brand: String,
    pub industry: String,
    pub city: String,
    pub visibility_score: u32,
    pub share_of_model: ShareOfModel,
    pub persona_heatmap: Vec<PersonaResult>,
    pub brand_dna: DnaReport,
    pub reality_check: RealityCheck,
    pub recommendations: Vec<Recommendation>,
    pub engram: EngramStatus,
}
```

### 新增 Cargo 依赖

```toml
# Cargo.toml additions
axum = { version = "0.8", features = ["macros"] }
tower-http = { version = "0.6", features = ["cors", "fs"] }
tokio-stream = "0.1"
uuid = { version = "1", features = ["v4"] }
```

---

## Checklist

### Backend
- [ ] B1 `persona.rs` (potato oneB)
- [ ] B2 `query_gen.rs` refactor (potato oneB)
- [ ] B3 `llm_probe.rs` refactor (potato oneB)
- [ ] B4 `web.rs` (Publius)
- [ ] B5 `brand_dna.rs` (Publius)
- [ ] B6 `reality.rs` (Publius)
- [ ] B7 `engram.rs` (Publius)
- [ ] B8 `main.rs` integration (together)
- [ ] `types.rs` shared types (define first)

### Frontend
- [ ] F1 Search page enhancements (Xenos/Yurica)
- [ ] F2 Loading/progress SSE (Xenos/Yurica)
- [ ] F3 Visibility Score tab (Xenos/Yurica)
- [ ] F4 Persona Heatmap tab (Xenos/Yurica)
- [ ] F5 Brand DNA D3 graph (Xenos/Yurica)
- [ ] F6 Reality Check tab (Xenos/Yurica)
- [ ] F7 Recommendations tab (Xenos/Yurica)
- [ ] F8 General UI polish (Xenos/Yurica)
