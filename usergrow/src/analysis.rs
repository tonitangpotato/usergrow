use crate::llm_probe::ProbeResult;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandMention {
    pub query_text: String,
    pub query_category: String,
    pub model: String,
    pub mentioned: bool,
    pub position: Option<usize>, // 1st, 2nd, 3rd mentioned
    pub sentiment: Sentiment,
    pub has_link: bool,
    pub context_snippet: String, // surrounding text where brand appears
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sentiment {
    Positive,
    Neutral,
    Negative,
    NotMentioned,
}

impl std::fmt::Display for Sentiment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sentiment::Positive => write!(f, "Positive"),
            Sentiment::Neutral => write!(f, "Neutral"),
            Sentiment::Negative => write!(f, "Negative"),
            Sentiment::NotMentioned => write!(f, "Not Mentioned"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorScore {
    pub name: String,
    pub mention_count: usize,
    pub mention_rate: f64,
    pub avg_position: f64,
    pub positive_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub category: String,
    pub finding: String,
    pub recommendation: String,
    pub severity: InsightSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightSeverity {
    Critical,
    Warning,
    Info,
    Strength,
}

impl std::fmt::Display for InsightSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsightSeverity::Critical => write!(f, "🔴 Critical"),
            InsightSeverity::Warning => write!(f, "🟡 Warning"),
            InsightSeverity::Info => write!(f, "🔵 Info"),
            InsightSeverity::Strength => write!(f, "🟢 Strength"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub brand: String,
    pub total_queries: usize,
    pub total_responses: usize,
    pub brand_mentions: Vec<BrandMention>,
    pub competitor_scores: Vec<CompetitorScore>,
    pub som_by_model: Vec<ModelSOM>,
    pub som_by_category: Vec<CategorySOM>,
    pub insights: Vec<Insight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSOM {
    pub model: String,
    pub mention_rate: f64,
    pub avg_position: f64,
    pub total_probed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySOM {
    pub category: String,
    pub mention_rate: f64,
    pub total_probed: usize,
}

/// Simple keyword-based sentiment analysis around brand mentions
fn analyze_sentiment(text: &str, brand: &str) -> Sentiment {
    let lower = text.to_lowercase();
    let brand_lower = brand.to_lowercase();

    if !lower.contains(&brand_lower) {
        return Sentiment::NotMentioned;
    }

    // Find the context around the brand mention
    let positive_signals = [
        "best",
        "top",
        "leading",
        "excellent",
        "great",
        "popular",
        "recommend",
        "user-friendly",
        "intuitive",
        "innovative",
        "reliable",
        "trusted",
        "easy to use",
        "well-known",
        "strong",
        "advantage",
        "pioneer",
    ];
    let negative_signals = [
        "controversy",
        "criticism",
        "limitation",
        "drawback",
        "concern",
        "downside",
        "risk",
        "issue",
        "problem",
        "fee",
        "expensive",
        "outage",
        "hack",
        "lawsuit",
        "fine",
        "penalty",
        "worse",
    ];

    let pos_count = positive_signals
        .iter()
        .filter(|s| lower.contains(**s))
        .count();
    let neg_count = negative_signals
        .iter()
        .filter(|s| lower.contains(**s))
        .count();

    if pos_count > neg_count + 1 {
        Sentiment::Positive
    } else if neg_count > pos_count + 1 {
        Sentiment::Negative
    } else {
        Sentiment::Neutral
    }
}

/// Find the position (rank) of brand mention in the response
pub fn find_brand_position(text: &str, brand: &str) -> Option<usize> {
    let lower = text.to_lowercase();
    let brand_lower = brand.to_lowercase();

    if !lower.contains(&brand_lower) {
        return None;
    }

    // Count how many "sections" or numbered items appear before the brand
    let idx = lower.find(&brand_lower)?;
    let before = &lower[..idx];

    // Count numbered list items (1. 2. 3. etc) or bold headers
    let position = before.matches('\n').count().min(10) / 3 + 1;
    Some(position.max(1))
}

/// Extract a snippet of text around the brand mention
fn extract_snippet(text: &str, brand: &str, context_chars: usize) -> String {
    let lower = text.to_lowercase();
    let brand_lower = brand.to_lowercase();

    if let Some(idx) = lower.find(&brand_lower) {
        // Find char-safe boundaries (don't slice in the middle of a multi-byte char)
        let start = lower[..idx]
            .char_indices()
            .rev()
            .nth(context_chars)
            .map(|(i, _)| i)
            .unwrap_or(0);
        let end = lower[idx + brand_lower.len()..]
            .char_indices()
            .nth(context_chars)
            .map(|(i, _)| idx + brand_lower.len() + i)
            .unwrap_or(text.len());
        format!("...{}...", &text[start..end])
    } else {
        String::new()
    }
}

pub fn analyze(
    brand: &str,
    competitors: &[String],
    probe_results: &[ProbeResult],
) -> Result<AnalysisResult> {
    let brand_lower = brand.to_lowercase();
    let mut brand_mentions = Vec::new();

    // Analyze each response for brand presence
    for result in probe_results {
        if result.error.is_some() {
            continue;
        }

        let response_lower = result.response.to_lowercase();
        let mentioned = response_lower.contains(&brand_lower);
        let position = find_brand_position(&result.response, brand);
        let sentiment = analyze_sentiment(&result.response, brand);
        let has_link = result.response.contains("http") && response_lower.contains(&brand_lower);
        let snippet = extract_snippet(&result.response, brand, 100);

        brand_mentions.push(BrandMention {
            query_text: result.query.text.clone(),
            query_category: result.query.category.to_string(),
            model: result.model.clone(),
            mentioned,
            position,
            sentiment,
            has_link,
            context_snippet: snippet,
        });
    }

    // Calculate competitor scores
    let mut competitor_scores = Vec::new();
    for comp in competitors {
        let comp_lower = comp.to_lowercase();
        let relevant: Vec<&ProbeResult> =
            probe_results.iter().filter(|r| r.error.is_none()).collect();
        let mentions: Vec<&ProbeResult> = relevant
            .iter()
            .filter(|r| r.response.to_lowercase().contains(&comp_lower))
            .copied()
            .collect();

        let mention_count = mentions.len();
        let mention_rate = if relevant.is_empty() {
            0.0
        } else {
            mention_count as f64 / relevant.len() as f64
        };

        let positions: Vec<usize> = mentions
            .iter()
            .filter_map(|r| find_brand_position(&r.response, comp))
            .collect();
        let avg_position = if positions.is_empty() {
            0.0
        } else {
            positions.iter().sum::<usize>() as f64 / positions.len() as f64
        };

        let positive_count = mentions
            .iter()
            .filter(|r| matches!(analyze_sentiment(&r.response, comp), Sentiment::Positive))
            .count();
        let positive_rate = if mention_count == 0 {
            0.0
        } else {
            positive_count as f64 / mention_count as f64
        };

        competitor_scores.push(CompetitorScore {
            name: comp.clone(),
            mention_count,
            mention_rate,
            avg_position,
            positive_rate,
        });
    }

    // SOM by model
    let models: Vec<String> = brand_mentions
        .iter()
        .map(|m| m.model.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let som_by_model: Vec<ModelSOM> = models
        .iter()
        .map(|model| {
            let model_mentions: Vec<&BrandMention> = brand_mentions
                .iter()
                .filter(|m| &m.model == model)
                .collect();
            let mentioned_count = model_mentions.iter().filter(|m| m.mentioned).count();
            let positions: Vec<usize> = model_mentions.iter().filter_map(|m| m.position).collect();

            ModelSOM {
                model: model.clone(),
                mention_rate: if model_mentions.is_empty() {
                    0.0
                } else {
                    mentioned_count as f64 / model_mentions.len() as f64
                },
                avg_position: if positions.is_empty() {
                    0.0
                } else {
                    positions.iter().sum::<usize>() as f64 / positions.len() as f64
                },
                total_probed: model_mentions.len(),
            }
        })
        .collect();

    // SOM by query category
    let categories: Vec<String> = brand_mentions
        .iter()
        .map(|m| m.query_category.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let som_by_category: Vec<CategorySOM> = categories
        .iter()
        .map(|cat| {
            let cat_mentions: Vec<&BrandMention> = brand_mentions
                .iter()
                .filter(|m| &m.query_category == cat)
                .collect();
            let mentioned_count = cat_mentions.iter().filter(|m| m.mentioned).count();

            CategorySOM {
                category: cat.clone(),
                mention_rate: if cat_mentions.is_empty() {
                    0.0
                } else {
                    mentioned_count as f64 / cat_mentions.len() as f64
                },
                total_probed: cat_mentions.len(),
            }
        })
        .collect();

    // Generate insights
    let mut insights = Vec::new();

    // Overall SOM insight
    let total_mentioned = brand_mentions.iter().filter(|m| m.mentioned).count();
    let overall_rate = if brand_mentions.is_empty() {
        0.0
    } else {
        total_mentioned as f64 / brand_mentions.len() as f64
    };

    if overall_rate < 0.3 {
        insights.push(Insight {
            category: "Overall Visibility".to_string(),
            finding: format!(
                "{} appears in only {:.0}% of LLM responses",
                brand,
                overall_rate * 100.0
            ),
            recommendation: "Significant content gap. Focus on creating authoritative content that LLMs can reference.".to_string(),
            severity: InsightSeverity::Critical,
        });
    } else if overall_rate < 0.6 {
        insights.push(Insight {
            category: "Overall Visibility".to_string(),
            finding: format!(
                "{} appears in {:.0}% of LLM responses",
                brand,
                overall_rate * 100.0
            ),
            recommendation:
                "Moderate visibility. Identify weak query categories and create targeted content."
                    .to_string(),
            severity: InsightSeverity::Warning,
        });
    } else {
        insights.push(Insight {
            category: "Overall Visibility".to_string(),
            finding: format!(
                "{} appears in {:.0}% of LLM responses — strong presence",
                brand,
                overall_rate * 100.0
            ),
            recommendation: "Good visibility. Focus on improving position and sentiment."
                .to_string(),
            severity: InsightSeverity::Strength,
        });
    }

    // Category-specific insights
    for cat_som in &som_by_category {
        if cat_som.mention_rate < 0.2 {
            insights.push(Insight {
                category: cat_som.category.clone(),
                finding: format!(
                    "Almost invisible in {} queries ({:.0}% mention rate)",
                    cat_som.category,
                    cat_som.mention_rate * 100.0
                ),
                recommendation: format!(
                    "Create content specifically targeting {} type queries about your brand",
                    cat_som.category
                ),
                severity: InsightSeverity::Critical,
            });
        }
    }

    // Competitor comparison insights
    for comp in &competitor_scores {
        if comp.mention_rate > overall_rate + 0.2 {
            insights.push(Insight {
                category: "Competitive Gap".to_string(),
                finding: format!(
                    "{} has {:.0}% mention rate vs your {:.0}%",
                    comp.name,
                    comp.mention_rate * 100.0,
                    overall_rate * 100.0
                ),
                recommendation: format!(
                    "Study {}'s content strategy. They are significantly more visible in LLM responses.",
                    comp.name
                ),
                severity: InsightSeverity::Warning,
            });
        }
    }

    // Sentiment insights
    let negative_count = brand_mentions
        .iter()
        .filter(|m| matches!(m.sentiment, Sentiment::Negative))
        .count();
    if negative_count > 0 {
        let neg_rate = negative_count as f64 / total_mentioned.max(1) as f64;
        if neg_rate > 0.3 {
            insights.push(Insight {
                category: "Sentiment".to_string(),
                finding: format!(
                    "{:.0}% of mentions have negative sentiment",
                    neg_rate * 100.0
                ),
                recommendation: "Address negative narratives with positive content, case studies, and PR efforts.".to_string(),
                severity: InsightSeverity::Critical,
            });
        }
    }

    Ok(AnalysisResult {
        brand: brand.to_string(),
        total_queries: probe_results.len() / 2, // rough estimate
        total_responses: probe_results.iter().filter(|r| r.error.is_none()).count(),
        brand_mentions,
        competitor_scores,
        som_by_model,
        som_by_category,
        insights,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm_probe::ProbeResult;
    use crate::query_gen::{Query, QueryCategory};

    fn make_probe(query: &str, model: &str, response: &str) -> ProbeResult {
        ProbeResult {
            query: Query {
                text: query.to_string(),
                category: QueryCategory::CategorySearch,
            },
            model: model.to_string(),
            response: response.to_string(),
            error: None,
        }
    }

    #[test]
    fn calculates_som_with_mentions() {
        let results = vec![
            make_probe(
                "best trading app",
                "gpt-4o",
                "Robinhood is a great trading app with zero commissions.",
            ),
            make_probe(
                "best trading app",
                "gemini",
                "The best apps include Fidelity and Schwab.",
            ),
            make_probe(
                "cheapest broker",
                "gpt-4o",
                "Robinhood offers commission-free trading and is very popular.",
            ),
        ];

        let analysis = analyze(
            "Robinhood",
            &["Fidelity".to_string(), "Schwab".to_string()],
            &results,
        )
        .unwrap();

        // Robinhood mentioned in 2/3 responses
        let mentioned = analysis
            .brand_mentions
            .iter()
            .filter(|m| m.mentioned)
            .count();
        assert_eq!(mentioned, 2);
    }

    #[test]
    fn detects_sentiment() {
        let positive = analyze_sentiment(
            "Robinhood is the best and most popular trading app",
            "Robinhood",
        );
        assert!(matches!(positive, Sentiment::Positive));

        let not_mentioned = analyze_sentiment("Fidelity is great for retirement", "Robinhood");
        assert!(matches!(not_mentioned, Sentiment::NotMentioned));
    }

    #[test]
    fn finds_brand_position() {
        let text = "The top apps are:\n1. Fidelity\n2. Schwab\n3. Robinhood";
        let pos = find_brand_position(text, "Robinhood");
        assert!(pos.is_some());
        assert!(pos.unwrap() >= 1);
    }

    #[test]
    fn extracts_snippet() {
        let text = "Among trading apps, Robinhood stands out for commission-free trading.";
        let snippet = extract_snippet(text, "Robinhood", 20);
        assert!(snippet.contains("Robinhood"));
    }

    #[test]
    fn generates_insights_for_low_visibility() {
        let results = vec![
            make_probe("best app", "gpt-4o", "Fidelity and Schwab are great."),
            make_probe("top broker", "gpt-4o", "Consider Schwab or TD Ameritrade."),
            make_probe("trading platform", "gemini", "Fidelity is recommended."),
        ];

        let analysis = analyze("Robinhood", &["Fidelity".to_string()], &results).unwrap();

        // Should flag critical visibility issue
        assert!(
            analysis
                .insights
                .iter()
                .any(|i| matches!(i.severity, InsightSeverity::Critical)),
            "should generate critical insight for 0% visibility"
        );
    }

    #[test]
    fn handles_empty_results() {
        let analysis = analyze("Robinhood", &[], &[]).unwrap();
        assert!(analysis.brand_mentions.is_empty());
        assert_eq!(analysis.total_responses, 0);
    }

    #[test]
    fn skips_error_results() {
        let results = vec![ProbeResult {
            query: Query {
                text: "test".to_string(),
                category: QueryCategory::CategorySearch,
            },
            model: "gpt-4o".to_string(),
            response: String::new(),
            error: Some("API error".to_string()),
        }];

        let analysis = analyze("Robinhood", &[], &results).unwrap();
        assert_eq!(analysis.total_responses, 0);
    }
}
