# UserGrow — Design Document

> **Generative Engine Optimization**: Measure and improve how AI models perceive, describe, and recommend your brand.

## 1. Product Overview

### Problem
AI assistants (ChatGPT, Gemini, Perplexity) are becoming the new front door for consumer decisions. When a user asks "recommend a cardiologist in New York," the AI's answer replaces a Google search — but brands have **zero visibility** into what AI recommends, to whom, or why.

### Solution
UserGrow is an **AI brand visibility intelligence platform** that:
1. Probes multiple LLMs with demographically diverse personas
2. Maps brand visibility across models, languages, and user segments
3. Compares AI recommendations against real-world data
4. Learns over time to provide increasingly precise optimization advice

### One-Line Pitch
*"We X-ray how AI sees your brand — across models, personas, languages, and time."*

---

## 2. Core Features

### 2.1 🔍 AI Visibility Score
- Input: brand name + industry + location
- Probe ChatGPT, Gemini, and Perplexity with category-relevant queries
- Output: 0–100 composite score + per-model Share of Model (SOM) breakdown

### 2.2 🎭 Persona Probing
- Pre-configured persona matrix based on US Census demographics
- Each persona probes the same intent from a different identity
- Output: heatmap showing brand visibility per persona × model × language
- **Key insight**: Discover which demographic segments can't find you in AI

### 2.3 🌍 Multi-Lingual Analysis
- Same queries issued in English, Spanish, Chinese, Hindi
- Detects brand visibility gaps in non-English AI responses
- Critical for brands serving diverse or immigrant communities

### 2.4 🧬 Brand DNA Extraction
- Extract keywords and phrases AI associates with your brand
- Compare your semantic fingerprint vs competitors
- Output: word cloud / force-directed graph of brand associations

### 2.5 ⭐ Reality Check (Tavily-Powered)
- Pull real-world ratings and reviews (Healthgrades, Zocdoc, Yelp, industry sources)
- Compare AI recommendations against ground truth
- Quantify AI bias: "You're rated 4.9★ but AI doesn't recommend you"

### 2.6 🧠 Engram Memory Layer
- Store scan results across runs (episodic memory)
- Drift detection: alert when visibility changes week-over-week
- Cross-brand learning: patterns discovered across scans improve future advice
- Causal memory: store and recall why specific visibility gaps exist

### 2.7 💡 Actionable Recommendations
- Synthesized from all dimensions above
- Informed by Engram's cross-brand pattern knowledge
- Specific and actionable: "Add Spanish-language FAQ," "Get listed on X directory"

---

## 3. Persona System Design

### 3.1 Academic Foundation

Our persona probing methodology is grounded in current research:

- **arXiv 2025** — "Revealing Potential Biases in LLM-Based Recommender Systems": Confirms LLMs exhibit consistent biases across recommendation domains (music, movies, colleges) based on user demographics including gender and cultural stereotypes.
- **ACL 2025** — Systematic Literature Review on Demographic Representation in LLMs: Validates demographic persona probing as a methodologically sound approach for evaluating LLM behavior.
- **Columbia/NYU 2025** — "LLM Generated Persona is a Promise with a Catch": Demonstrates that LLM responses vary significantly based on persona attributes, serving as "silicon samples" for population-level analysis.

**Our innovation**: Academia studies bias as a phenomenon. We productize it — letting brands diagnose how bias specifically impacts *their* visibility.

### 3.2 Persona Construction

Personas are constructed from **US Census Bureau** demographic distributions to ensure statistical representativeness:

| Dimension | Categories | Source |
|-----------|-----------|--------|
| **Age** | 18-24 (12%), 25-34 (18%), 35-54 (26%), 55-64 (13%), 65+ (17%) | Census ACS |
| **Race/Ethnicity** | White (58%), Hispanic (19%), Black (12%), Asian (6%), Other (5%) | Census 2020 |
| **Income** | <$30K, $30-75K, $75-150K, $150K+ | Census ACS |
| **Language** | English, Spanish, Chinese, Hindi | Census Language Use |
| **Immigration** | Native-born, Naturalized citizen, Recent immigrant | Census CPS |

### 3.3 Representative Persona Pool

Rather than exhaustive combinations (thousands), we select **representative personas** that each cover a key demographic segment:

| Persona | Profile | Rationale |
|---------|---------|-----------|
| **Maria, 32** | Mexican immigrant, Spanish-primary, middle income | Largest US immigrant group (24% of foreign-born) |
| **James, 67** | White retiree, high net worth, English | Core financial/healthcare consumer |
| **Wei, 28** | Chinese international student/new immigrant, bilingual | Fastest-growing Asian demographic |
| **Aisha, 45** | Black woman, middle income, native-born | Historically underrepresented in AI training data |
| **Tyler, 22** | White college student, low income, English | Gen Z, AI-native user behavior |
| **Priya, 38** | Indian-American IT professional, high income, English | High-spending immigrant professional class |
| **Carlos, 55** | Puerto Rican, bilingual, middle income | US-born Hispanic, distinct from immigrant Hispanic |
| **Sarah, 41** | White suburban mother, middle income, English | Key healthcare decision-maker demographic |

### 3.4 Prompt Template

```
I am a {age}-year-old {ethnicity} {gender}, {immigration_status}.
I live in {city} and work as {occupation}. My household income is
approximately {income_range}. {additional_context}

Can you recommend {service_type} for someone like me?
```

Same template, same intent — different identity → different AI response → measurable bias.

### 3.5 Localization

Persona probes are issued with **city-level context** because AI recommendations vary dramatically by location:

- Same persona in NYC vs Houston vs San Francisco → different local brands
- Spanish-language probe in Miami vs English probe in Miami → different results
- Chinese-language probe in Flushing, NY vs English → completely different providers

Implementation: User selects city → all persona prompts include location context.

---

## 4. Causal Analysis Framework

### 4.1 From "What" to "Why"

Most GEO tools stop at observation ("you're not mentioned"). We go further with causal inference:

```
OBSERVATION
  Maria (Spanish, immigrant) → Brand not mentioned
  James (English, retiree) → Brand mentioned #2
                ↓
HYPOTHESIS GENERATION
  H1: Brand lacks Spanish-language web content
  H2: No Spanish-language reviews/articles about brand
  H3: Brand not listed in Hispanic community directories
                ↓
EVIDENCE GATHERING (Tavily)
  H1: Tavily search "{brand} sitio web español" → No results ✅ Confirmed
  H2: Tavily search "{brand} reseñas español" → 0 articles ✅ Confirmed
  H3: Tavily search "{brand} directorio hispano" → Not listed ✅ Confirmed
                ↓
CAUSAL CHAIN
  No Spanish content → Not in Spanish training data → AI can't recommend to Spanish speakers
                ↓
RECOMMENDATION
  "Create Spanish-language website sections and get listed in Hispanic community directories"
```

### 4.2 Engram Integration

Causal relationships are stored in Engram as `relational` memories:
- Cross-brand patterns emerge: "Brands without multilingual content consistently show 0% visibility in non-English personas"
- Confidence increases with more observations
- Hebbian learning: co-occurring factors strengthen connections automatically

---

## 5. Technical Architecture

### 5.1 Module Map

```
Frontend (HTML/JS/Tailwind)
    ↓ HTTP
web.rs (axum server)
    ↓
┌─────────────────────────────────────────┐
│            Analysis Pipeline            │
│                                         │
│  query_gen.rs ← persona.rs              │
│       ↓         (persona matrix)        │
│  llm_probe.rs                           │
│  (GPT / Gemini / Perplexity, parallel)  │
│       ↓                                 │
│  analysis.rs + brand_dna.rs             │
│  (SOM, heatmap, DNA extraction)         │
│       ↓                                 │
│  reality.rs ← tavily.rs                 │
│  (real-world data comparison)           │
│       ↓                                 │
│  engram.rs                              │
│  (memory storage, drift, causal)        │
└─────────────────────────────────────────┘
    ↓ JSON + SSE
Frontend renders dashboard
```

### 5.2 Modules

| Module | Status | Description |
|--------|--------|-------------|
| `main.rs` | 🔄 Refactor | CLI → axum web server |
| `web.rs` | 🆕 New | Routes, SSE, CORS, static files |
| `query_gen.rs` | 🔄 Refactor | Add persona × language matrix |
| `persona.rs` | 🆕 New | Census-backed persona definitions, prompt templates |
| `llm_probe.rs` | 🔄 Refactor | Add Perplexity, parallel execution, progress callbacks |
| `analysis.rs` | 🔄 Refactor | Add persona heatmap aggregation |
| `brand_dna.rs` | 🆕 New | Keyword extraction, TF-IDF, competitor comparison |
| `tavily.rs` | ✅ Done | Web search and content extraction |
| `reality.rs` | 🆕 New | Real-world ratings, AI bias quantification |
| `engram.rs` | 🆕 New | Memory layer, drift detection, causal storage |
| `report.rs` | 🔄 Refactor | JSON API responses (frontend renders) |

### 5.3 API Endpoints

```
POST /api/analyze
  Body: { brand, industry, city, target_group? }
  Response: SSE stream of progress + final JSON report

GET  /api/health
  Response: { status: "ok", version, engram_stats }

GET  /api/history/{brand}
  Response: Previous scan results (from Engram)
```

### 5.4 Tech Stack

- **Backend**: Rust (tokio + axum + reqwest)
- **Frontend**: HTML + Tailwind CSS + Chart.js + D3.js
- **Memory**: Engram (Python CLI, called via subprocess)
- **Data**: Tavily API (search + extract)
- **LLMs**: OpenAI (GPT-4o), Google (Gemini 2.0 Flash), Perplexity
- **Deploy**: Vercel (frontend) + local/cloud (backend)

---

## 6. Data Flow Example

**User inputs**: "Dr. Smith Cardiology" + Healthcare + New York City

**Step 1 — Query Generation** (2s)
- 6 base queries × 8 personas × 3 languages × 3 models = ~432 probes
- Batched and parallelized, prioritized by persona relevance

**Step 2 — LLM Probing** (5-10s, parallel)
- SSE pushes: "Probing ChatGPT (EN)... ✅" "Probing Gemini (ZH)... ✅"
- Each response parsed for brand mentions, position, sentiment, associated keywords

**Step 3 — Analysis** (1s)
- SOM calculation per model/persona/language
- Persona heatmap aggregation
- Brand DNA keyword extraction + TF-IDF
- Competitor identification and ranking

**Step 4 — Reality Check** (2s)
- Tavily searches: ratings, reviews, directory listings
- AI recommendation rank vs real-world rating comparison
- Bias score calculation

**Step 5 — Causal Analysis** (1s)
- Pattern matching against Engram's stored causal relationships
- New hypotheses generated and verified via Tavily
- Results stored in Engram for future learning

**Step 6 — Report** (<1s)
- JSON response with all dimensions
- Frontend renders: score, heatmap, DNA graph, bias chart, recommendations

**Total: ~15-20 seconds** (with SSE progress updates throughout)

---

## 7. Competitive Differentiation

| Feature | Gushwork.ai | Promptwatch | **UserGrow** |
|---------|------------|-------------|---------------|
| Multi-model probing | ✅ | ✅ | ✅ |
| Persona-based testing | ❌ | ❌ | ✅ Census-backed |
| Multi-lingual | ❌ | ❌ | ✅ 4 languages |
| Brand DNA extraction | ❌ | ❌ | ✅ |
| Real data comparison | ❌ | ❌ | ✅ Tavily-powered |
| Learning memory | ❌ | ❌ | ✅ Engram |
| Causal analysis | ❌ | ❌ | ✅ |
| Self-service | ❌ (agency) | ✅ | ✅ |
| Time to result | 90-150 days | Manual | **~20 seconds** |

---

## 8. Demo Plan

### Live Demo Flow (3 minutes)

1. **Open landing page** — explain the problem (30s)
2. **Type "Dr. Smith Cardiology, New York"** — hit Analyze (5s)
3. **Watch real-time probing** — SSE progress animation (15s)
4. **Tab 1: Visibility Score** — "23/100, that's bad" (20s)
5. **Tab 2: Persona Heatmap** — "Invisible to Spanish speakers and retirees" (30s)
6. **Tab 3: Brand DNA** — "AI thinks you're 'expensive' but competitors are 'trusted'" (20s)
7. **Tab 4: Reality Check** — "You're rated 4.9★ but AI recommends 4.2★ competitors" (20s)
8. **Tab 5: Recommendations** — specific actions (15s)
9. **Mention Engram** — "Run this weekly, track drift, agent gets smarter" (15s)
10. **Second search** — different brand, different industry, to show generality (30s)

### Pre-Demo Preparation
- Pre-verify that chosen brand shows interesting persona variation
- Have 2-3 backup brands ready
- Ensure API keys are valid and rate limits won't hit during demo

---

## 9. References

1. "Revealing Potential Biases in LLM-Based Recommender Systems" — arXiv:2508.20401, 2025
2. "The Effects of Demographic Instructions on LLM Personas" — arXiv:2505.11795, 2025
3. "LLM Generated Persona is a Promise with a Catch" — Columbia/NYU, OpenReview 2025
4. "A Systematic Literature Review on the Demographic Representation in LLMs" — ACL Findings 2025
5. US Census Bureau — American Community Survey (ACS) 2023
6. NAICS — North American Industry Classification System (census.gov/naics)

---

## 10. Next Steps (Post-Hackathon Roadmap)

**For demo: "Here's where we're going next"**

### Data Layer
- Direct API integrations for structured data: Zocdoc, Healthgrades, Yelp Fusion, Google Places → precise ratings (to decimal), review counts, appointment volumes
- Census API for dynamic persona pool updates as demographics shift
- SimilarWeb / SEMrush for website traffic data to enrich causal analysis

### AI Coverage
- Add Claude, Copilot, Siri, Alexa as probe targets — cover the full AI assistant ecosystem
- Voice assistant probing: test how Siri/Alexa respond to the same queries (spoken AI is the next frontier)

### Product Features
- **Scheduled monitoring**: weekly automated scans with Engram drift alerts via email/Slack
- **A/B content testing**: "If you add this FAQ page, we predict your visibility will increase by X%" — use Engram's causal patterns to simulate outcomes
- **White-label reports**: PDF export with client branding for agencies/consultants
- **Industry benchmarks**: "Your visibility score is 23 — the average for healthcare in NYC is 45"

### Scale
- Multi-tenant SaaS platform with dashboard login
- API access for enterprise clients to integrate into their own marketing tools
- Engram shared knowledge base: anonymized cross-client learnings (with consent) improve recommendations for everyone
