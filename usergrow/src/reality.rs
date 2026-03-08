use anyhow::Result;

use crate::tavily::TavilyClient;
use crate::types::{CompetitorReality, RatingData, RealityCheckReport};

/// Check real-world ratings and reviews via Tavily, compare against AI perception.
pub async fn check_reality(
    tavily: &TavilyClient,
    brand: &str,
    competitors: &[String],
    industry: &str,
    city: &str,
) -> Result<RealityCheckReport> {
    // Search for brand ratings
    let your_rating = search_rating(tavily, brand, industry, city).await;

    // Search for competitor ratings
    let mut competitor_results = Vec::new();
    for comp in competitors {
        let rating = search_rating(tavily, comp, industry, city).await;
        competitor_results.push(CompetitorReality {
            name: comp.clone(),
            ai_rank: None, // filled in by caller from probe results
            real_rating: rating.as_ref().map(|r| r.score),
        });
    }

    // Calculate bias score: how much AI recommendations diverge from real-world ratings
    let bias_score = calculate_bias(&your_rating, &competitor_results);

    Ok(RealityCheckReport {
        your_rating,
        ai_rank: None, // filled in by caller
        competitors: competitor_results,
        bias_score,
    })
}

impl Default for RealityCheckReport {
    fn default() -> Self {
        Self {
            your_rating: None,
            ai_rank: None,
            competitors: Vec::new(),
            bias_score: 0.0,
        }
    }
}

/// Search for a brand's real-world rating using Tavily.
async fn search_rating(
    tavily: &TavilyClient,
    brand: &str,
    industry: &str,
    city: &str,
) -> Option<RatingData> {
    let query = if city.is_empty() {
        format!("{} {} rating reviews", brand, industry)
    } else {
        format!("{} {} {} rating reviews", brand, industry, city)
    };

    let resp = tavily.search(&query, 5).await.ok()?;

    // Try to extract a rating from search results
    // Look for patterns like "4.5/5", "4.5 stars", "rated 4.5"
    let mut best_score: Option<f64> = None;
    let mut source = String::new();

    for result in &resp.results {
        if let Some(score) = extract_rating(&result.content) {
            if best_score.is_none() || score > best_score.unwrap() {
                best_score = Some(score);
                source = result.url.clone();
            }
        }
    }

    // Also check the synthesized answer
    if let Some(ref answer) = resp.answer {
        if let Some(score) = extract_rating(answer) {
            if best_score.is_none() || score > best_score.unwrap() {
                best_score = Some(score);
                source = "Tavily Answer".to_string();
            }
        }
    }

    best_score.map(|score| RatingData {
        source,
        score,
        review_count: 0, // hard to extract reliably
    })
}

/// Extract a numeric rating from text.
/// Looks for patterns: "4.5/5", "4.5 stars", "rated 4.5", "4.5 out of 5"
fn extract_rating(text: &str) -> Option<f64> {
    let lower = text.to_lowercase();

    // Pattern: X.X/5 or X/5
    for pattern in &["/5", "out of 5"] {
        if let Some(idx) = lower.find(pattern) {
            let before = &lower[..idx].trim_end();
            if let Some(num_str) = before.split_whitespace().last() {
                if let Ok(val) = num_str.parse::<f64>() {
                    if (0.0..=5.0).contains(&val) {
                        return Some(val);
                    }
                }
            }
        }
    }

    // Pattern: X.X stars
    if let Some(idx) = lower.find("star") {
        let before = &lower[..idx].trim_end();
        if let Some(num_str) = before.split_whitespace().last() {
            if let Ok(val) = num_str.parse::<f64>() {
                if (0.0..=5.0).contains(&val) {
                    return Some(val);
                }
            }
        }
    }

    // Pattern: "rated X.X"
    if let Some(idx) = lower.find("rated") {
        let after = &lower[idx + 5..];
        for word in after.split_whitespace().take(2) {
            if let Ok(val) = word
                .trim_matches(|c: char| !c.is_numeric() && c != '.')
                .parse::<f64>()
            {
                if (0.0..=5.0).contains(&val) {
                    return Some(val);
                }
            }
        }
    }

    None
}

/// Calculate how much AI perception diverges from real-world ratings.
/// Returns 0-100: 0 = perfectly aligned, 100 = completely divergent.
fn calculate_bias(your_rating: &Option<RatingData>, competitors: &[CompetitorReality]) -> f64 {
    // If no real-world rating data, we can't calculate bias
    let your_score = match your_rating {
        Some(r) => r.score,
        None => return 0.0,
    };

    // Get competitor ratings that have real-world data
    let comp_ratings: Vec<f64> = competitors
        .iter()
        .filter_map(|c| c.real_rating)
        .collect();

    if comp_ratings.is_empty() {
        return 0.0;
    }

    // Calculate where our brand should rank based on real-world ratings
    let _better_count = comp_ratings.iter().filter(|&&r| r > your_score).count();

    // Estimate bias from rating gap
    let avg_comp = comp_ratings.iter().sum::<f64>() / comp_ratings.len() as f64;
    let rating_gap = (your_score - avg_comp).abs();

    // Normalize: 0 gap = 0 bias, 2.5 gap (half the 5-point scale) = 100 bias
    let bias = (rating_gap / 2.5 * 100.0).min(100.0);

    // Weight by how many competitors we have data for (more data = more confident)
    let confidence = (comp_ratings.len() as f64 / competitors.len().max(1) as f64).min(1.0);

    bias * confidence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_slash_rating() {
        assert_eq!(extract_rating("rated 4.5/5 by users"), Some(4.5));
        assert_eq!(extract_rating("score: 3.8/5"), Some(3.8));
    }

    #[test]
    fn extracts_star_rating() {
        assert_eq!(extract_rating("has 4.2 stars on Google"), Some(4.2));
        assert_eq!(extract_rating("4.7 star rating"), Some(4.7));
    }

    #[test]
    fn extracts_out_of_rating() {
        assert_eq!(extract_rating("rated 4.1 out of 5"), Some(4.1));
    }

    #[test]
    fn extracts_rated_pattern() {
        assert_eq!(extract_rating("it is rated 3.9 by customers"), Some(3.9));
    }

    #[test]
    fn rejects_invalid_ratings() {
        assert_eq!(extract_rating("no rating here"), None);
        assert_eq!(extract_rating("scored 7.5/5"), None); // out of range
    }

    #[test]
    fn default_report() {
        let report = RealityCheckReport::default();
        assert!(report.your_rating.is_none());
        assert_eq!(report.bias_score, 0.0);
    }

    #[test]
    fn calculates_bias_score() {
        let your_rating = Some(RatingData {
            source: "Google".to_string(),
            score: 4.8,
            review_count: 100,
        });
        let competitors = vec![
            CompetitorReality { name: "CompA".to_string(), ai_rank: None, real_rating: Some(3.5) },
            CompetitorReality { name: "CompB".to_string(), ai_rank: None, real_rating: Some(4.0) },
        ];
        let bias = calculate_bias(&your_rating, &competitors);
        assert!(bias > 0.0, "Should detect rating gap bias");
        assert!(bias <= 100.0, "Bias should be bounded");
    }

    #[test]
    fn bias_zero_without_data() {
        assert_eq!(calculate_bias(&None, &[]), 0.0);
    }
}
