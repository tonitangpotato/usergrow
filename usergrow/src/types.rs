use serde::{Deserialize, Serialize};

// ── Web API Types ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeRequest {
    pub brand: String,
    pub industry: String,
    pub city: Option<String>,
    pub target_group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeResponse {
    pub job_id: String,
}

// ── Progress & Report Types ────────────────────────────────────────────

/// Progress event for SSE streaming to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEvent {
    pub step: String,
    pub status: String, // "progress" | "complete" | "error"
    pub detail: Option<String>,
    pub done: Option<u32>,
    pub total: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// The complete report returned to frontend as JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullReport {
    pub brand: String,
    pub industry: String,
    pub city: String,
    pub visibility_score: u32,
    pub share_of_model: ShareOfModel,
    pub persona_heatmap: Vec<PersonaHeatmapEntry>,
    pub brand_dna: DnaReport,
    pub reality_check: RealityCheckReport,
    pub recommendations: Vec<Recommendation>,
    pub engram: EngramStatus,
    pub evidence: Evidence, // NEW: evidence and citations
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShareOfModel {
    #[serde(default)]
    pub chatgpt: f64,
    #[serde(default)]
    pub gemini: f64,
    #[serde(default)]
    pub perplexity: f64,
    #[serde(default)]
    pub claude: f64,
    #[serde(default, rename = "glm_47")]
    pub glm_47: f64,
    #[serde(default, rename = "glm_45")]
    pub glm_45: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaHeatmapEntry {
    pub persona_name: String,
    pub persona_description: String,
    pub results: std::collections::HashMap<String, PersonaCellResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaCellResult {
    pub mentioned: bool,
    pub rank: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnaReport {
    pub your_brand: Vec<KeywordScore>,
    pub competitors: std::collections::HashMap<String, Vec<KeywordScore>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordScore {
    pub keyword: String,
    pub score: f64,
    pub sentiment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityCheckReport {
    pub your_rating: Option<RatingData>,
    pub ai_rank: Option<u32>,
    pub competitors: Vec<CompetitorReality>,
    pub bias_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingData {
    pub source: String,
    pub score: f64,
    pub review_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorReality {
    pub name: String,
    pub ai_rank: Option<u32>,
    pub real_rating: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub priority: String, // "critical", "important", "nice"
    pub text: String,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngramStatus {
    pub total_scans: u32,
    pub drift: Option<DriftInfo>,
    pub confidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftInfo {
    pub direction: String,
    pub delta: f64,
}

// ── Evidence & Citations Types ─────────────────────────────────────────

/// Evidence collected during analysis (LLM responses + Tavily sources)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub llm_responses: Vec<LLMEvidence>,
    pub tavily_sources: Vec<TavilySource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMEvidence {
    pub model: String,
    pub query: String,
    pub response_snippet: String, // first ~500 chars
    pub full_response: String,    // full text for modal/expansion
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TavilySource {
    pub url: String,
    pub title: String,
    pub snippet: String,
}
