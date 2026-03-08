#![allow(dead_code)] // Types and methods used in tests + will be wired into pipeline

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

// ── Request types ──────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct SearchRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_depth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_answer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_raw_content: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ExtractRequest {
    pub urls: Vec<String>,
}

// ── Response types ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub query: String,
    #[serde(default)]
    pub answer: Option<String>,
    #[serde(default)]
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
    #[serde(default)]
    pub score: f64,
}

#[derive(Debug, Deserialize)]
pub struct ExtractResponse {
    pub results: Vec<ExtractResult>,
}

#[derive(Debug, Deserialize)]
pub struct ExtractResult {
    pub url: String,
    pub raw_content: String,
}

// ── Client ─────────────────────────────────────────────────────────────

/// Tavily API client for web search and content extraction.
///
/// Uses the REST API directly (portable, no MCP server dependency).
/// For MCP integration, use the endpoint:
///   `https://mcp.tavily.com/mcp/?tavilyApiKey=<key>`
pub struct TavilyClient {
    client: Client,
    api_key: String,
}

impl TavilyClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Search the web. Returns relevant results with content snippets.
    pub async fn search(&self, query: &str, max_results: u32) -> Result<SearchResponse> {
        let req = SearchRequest {
            query: query.to_string(),
            search_depth: Some("advanced".to_string()),
            max_results: Some(max_results),
            include_answer: Some(true),
            include_raw_content: Some(false),
        };

        self.client
            .post("https://api.tavily.com/search")
            .bearer_auth(&self.api_key)
            .json(&req)
            .send()
            .await
            .context("Tavily search request failed")?
            .error_for_status()
            .context("Tavily search returned error status")?
            .json::<SearchResponse>()
            .await
            .context("Failed to parse Tavily search response")
    }

    /// Extract clean content from URLs. Returns markdown/text.
    pub async fn extract(&self, urls: Vec<String>) -> Result<ExtractResponse> {
        let req = ExtractRequest { urls };

        self.client
            .post("https://api.tavily.com/extract")
            .bearer_auth(&self.api_key)
            .json(&req)
            .send()
            .await
            .context("Tavily extract request failed")?
            .error_for_status()
            .context("Tavily extract returned error status")?
            .json::<ExtractResponse>()
            .await
            .context("Failed to parse Tavily extract response")
    }

    /// Discover competitors for a brand in an industry.
    /// Returns a list of competitor names extracted from search results.
    pub async fn discover_competitors(&self, brand: &str, industry: &str, city: &str) -> Result<Vec<String>> {
        let query = if city.is_empty() {
            format!("top {} companies competitors to {} 2026", industry, brand)
        } else {
            format!("top {} companies competitors to {} in {} 2026", industry, brand, city)
        };
        let response = self.search(&query, 10).await?;

        let mut competitors = Vec::new();

        // Extract from Tavily's synthesized answer
        if let Some(ref answer) = response.answer {
            // The answer typically mentions competitor names directly
            competitors.push(answer.clone());
        }

        // Also collect from individual results for richer context
        for result in &response.results {
            competitors.push(result.content.clone());
        }

        Ok(competitors)
    }

    /// Enrich query generation with real-time web context.
    /// Searches for what people actually ask about a brand/industry.
    pub async fn research_queries(&self, brand: &str, industry: &str) -> Result<Vec<String>> {
        let searches = vec![
            format!("{} review reddit 2026", brand),
            format!("best {} alternatives to {}", industry, brand),
            format!("{} vs competitors comparison", brand),
        ];

        let mut context_snippets = Vec::new();
        for query in &searches {
            match self.search(query, 5).await {
                Ok(resp) => {
                    if let Some(answer) = resp.answer {
                        context_snippets.push(answer);
                    }
                    for r in resp.results.into_iter().take(3) {
                        context_snippets.push(r.content);
                    }
                }
                Err(e) => {
                    eprintln!("     ⚠ Tavily search warning: {}", e);
                }
            }
        }

        Ok(context_snippets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_request_serializes() {
        let req = SearchRequest {
            query: "test query".to_string(),
            search_depth: Some("advanced".to_string()),
            max_results: Some(5),
            include_answer: Some(true),
            include_raw_content: None,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test query"));
        assert!(json.contains("advanced"));
        assert!(!json.contains("include_raw_content")); // None should be skipped
    }

    #[test]
    fn search_response_deserializes() {
        let json = r#"{
            "query": "test",
            "answer": "The answer is 42",
            "results": [
                {
                    "title": "Test Result",
                    "url": "https://example.com",
                    "content": "Some content here",
                    "score": 0.95
                }
            ]
        }"#;

        let resp: SearchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.query, "test");
        assert_eq!(resp.answer.unwrap(), "The answer is 42");
        assert_eq!(resp.results.len(), 1);
        assert_eq!(resp.results[0].title, "Test Result");
        assert!((resp.results[0].score - 0.95).abs() < f64::EPSILON);
    }

    #[test]
    fn search_response_handles_missing_fields() {
        let json = r#"{"query": "test"}"#;
        let resp: SearchResponse = serde_json::from_str(json).unwrap();
        assert!(resp.answer.is_none());
        assert!(resp.results.is_empty());
    }

    #[test]
    fn extract_request_serializes() {
        let req = ExtractRequest {
            urls: vec!["https://example.com".to_string()],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("https://example.com"));
    }

    #[test]
    fn client_constructs() {
        let client = TavilyClient::new("test-key".to_string());
        assert_eq!(client.api_key, "test-key");
    }

    #[tokio::test]
    #[ignore] // Requires valid TAVILY_API_KEY
    async fn live_search() {
        let key = std::env::var("TAVILY_API_KEY").expect("TAVILY_API_KEY required");
        let client = TavilyClient::new(key);
        let resp = client.search("best trading app 2026", 3).await.unwrap();
        assert!(!resp.results.is_empty());
    }
}
