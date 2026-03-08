use std::collections::HashMap;

use crate::llm_probe::ProbeResult;
use crate::types::{DnaReport, KeywordScore};

/// Extract keywords and sentiment from LLM responses about a brand.
/// Uses TF-IDF-like scoring: frequency across responses, weighted by proximity to brand mention.
pub fn extract_dna(brand: &str, results: &[ProbeResult]) -> DnaReport {
    let brand_lower = brand.to_lowercase();

    // Collect text from responses that mention the brand
    let brand_texts: Vec<&str> = results
        .iter()
        .filter(|r| r.error.is_none() && r.response.to_lowercase().contains(&brand_lower))
        .map(|r| r.response.as_str())
        .collect();

    let your_brand = extract_keywords(&brand_lower, &brand_texts);

    // Extract competitor DNA from all responses
    let mut competitor_mentions: HashMap<String, Vec<&str>> = HashMap::new();
    for r in results.iter().filter(|r| r.error.is_none()) {
        // Find competitor names mentioned (words that appear as proper nouns near "vs", "or", "compared to")
        // Simple heuristic: capitalized words that aren't the brand and appear multiple times
        for word in extract_proper_nouns(&r.response) {
            if word.to_lowercase() != brand_lower && word.len() > 2 {
                competitor_mentions
                    .entry(word.clone())
                    .or_default()
                    .push(&r.response);
            }
        }
    }

    // Keep only competitors mentioned in 3+ responses
    let competitors: HashMap<String, Vec<KeywordScore>> = competitor_mentions
        .into_iter()
        .filter(|(_, texts)| texts.len() >= 3)
        .map(|(name, texts)| {
            let name_lower = name.to_lowercase();
            let keywords = extract_keywords(&name_lower, &texts);
            (name, keywords)
        })
        .take(5) // Top 5 competitors
        .collect();

    DnaReport {
        your_brand,
        competitors,
    }
}

/// Extract keywords associated with a brand from a set of texts.
fn extract_keywords(brand: &str, texts: &[&str]) -> Vec<KeywordScore> {
    let mut word_freq: HashMap<String, usize> = HashMap::new();
    let mut word_sentiment: HashMap<String, (i32, i32)> = HashMap::new(); // (pos, neg)

    let stop_words: std::collections::HashSet<&str> = [
        "the", "a", "an", "is", "are", "was", "were", "be", "been", "being", "have", "has", "had",
        "do", "does", "did", "will", "would", "could", "should", "may", "might", "shall", "can",
        "need", "dare", "ought", "used", "to", "of", "in", "for", "on", "with", "at", "by", "from",
        "as", "into", "through", "during", "before", "after", "above", "below", "between", "out",
        "off", "over", "under", "again", "further", "then", "once", "here", "there", "when",
        "where", "why", "how", "all", "both", "each", "few", "more", "most", "other", "some",
        "such", "no", "nor", "not", "only", "own", "same", "so", "than", "too", "very", "just",
        "don", "now", "and", "but", "or", "if", "while", "that", "this", "it", "its", "they",
        "their", "them", "we", "our", "you", "your", "he", "she", "his", "her", "what", "which",
        "who", "whom", "also", "about", "up", "like",
    ]
    .into_iter()
    .collect();

    let positive_words: std::collections::HashSet<&str> = [
        "best",
        "great",
        "excellent",
        "top",
        "leading",
        "popular",
        "innovative",
        "trusted",
        "reliable",
        "easy",
        "intuitive",
        "strong",
        "recommended",
        "pioneer",
        "free",
        "fast",
        "secure",
        "modern",
        "powerful",
        "comprehensive",
        "convenient",
        "affordable",
    ]
    .into_iter()
    .collect();

    let negative_words: std::collections::HashSet<&str> = [
        "limited",
        "expensive",
        "complex",
        "slow",
        "risky",
        "controversial",
        "outdated",
        "difficult",
        "poor",
        "lacking",
        "confusing",
        "unreliable",
        "basic",
        "restrictive",
    ]
    .into_iter()
    .collect();

    for text in texts {
        let lower = text.to_lowercase();

        // Find words near brand mentions (within 50 words)
        if let Some(idx) = lower.find(brand) {
            let mut context_start = idx.saturating_sub(300);
            let mut context_end = (idx + brand.len() + 300).min(lower.len());
            // Ensure we slice on char boundaries (important for multi-byte UTF-8)
            while context_start > 0 && !lower.is_char_boundary(context_start) {
                context_start -= 1;
            }
            while context_end < lower.len() && !lower.is_char_boundary(context_end) {
                context_end += 1;
            }
            let context = &lower[context_start..context_end];

            for word in context.split(|c: char| !c.is_alphanumeric()) {
                let word = word.trim();
                if word.len() < 3 || stop_words.contains(word) || word == brand {
                    continue;
                }

                *word_freq.entry(word.to_string()).or_default() += 1;

                let entry = word_sentiment.entry(word.to_string()).or_default();
                if positive_words.contains(word) {
                    entry.0 += 1;
                }
                if negative_words.contains(word) {
                    entry.1 += 1;
                }
            }
        }
    }

    // Sort by frequency, take top 20
    let mut keywords: Vec<_> = word_freq.into_iter().collect();
    keywords.sort_by(|a, b| b.1.cmp(&a.1));

    keywords
        .into_iter()
        .take(20)
        .map(|(word, freq)| {
            let (pos, neg) = word_sentiment.get(&word).copied().unwrap_or((0, 0));
            let sentiment = if pos > neg {
                "positive"
            } else if neg > pos {
                "negative"
            } else {
                "neutral"
            };

            KeywordScore {
                keyword: word,
                score: freq as f64,
                sentiment: sentiment.to_string(),
            }
        })
        .collect()
}

/// Extract proper nouns (capitalized words) from text.
fn extract_proper_nouns(text: &str) -> Vec<String> {
    let mut nouns = Vec::new();
    for word in text.split_whitespace() {
        let clean: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
        if clean.len() > 2
            && clean
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
            && !clean.chars().all(|c| c.is_uppercase())
        // skip ALL CAPS
        {
            nouns.push(clean);
        }
    }
    nouns
}

/// Comparison result between two brand DNAs
#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DnaComparison {
    /// Keywords unique to brand A
    pub unique_a: Vec<String>,
    /// Keywords unique to brand B
    pub unique_b: Vec<String>,
    /// Shared keywords with score difference (keyword, score_a, score_b)
    pub shared: Vec<(String, f64, f64)>,
    /// Cosine similarity between the two keyword vectors (0.0 - 1.0)
    pub similarity: f64,
}

/// Compare the DNA profiles of two brands.
/// Returns shared keywords, unique keywords, and overall similarity.
#[allow(dead_code)]
pub fn compare_dna(brand_a: &[KeywordScore], brand_b: &[KeywordScore]) -> DnaComparison {
    let map_a: HashMap<&str, f64> = brand_a.iter().map(|k| (k.keyword.as_str(), k.score)).collect();
    let map_b: HashMap<&str, f64> = brand_b.iter().map(|k| (k.keyword.as_str(), k.score)).collect();

    let keys_a: std::collections::HashSet<&str> = map_a.keys().copied().collect();
    let keys_b: std::collections::HashSet<&str> = map_b.keys().copied().collect();

    let unique_a: Vec<String> = keys_a.difference(&keys_b).map(|s| s.to_string()).collect();
    let unique_b: Vec<String> = keys_b.difference(&keys_a).map(|s| s.to_string()).collect();

    let shared: Vec<(String, f64, f64)> = keys_a
        .intersection(&keys_b)
        .map(|k| (k.to_string(), map_a[k], map_b[k]))
        .collect();

    // Cosine similarity over shared keyword space
    let all_keys: std::collections::HashSet<&str> = keys_a.union(&keys_b).copied().collect();
    let mut dot = 0.0_f64;
    let mut mag_a = 0.0_f64;
    let mut mag_b = 0.0_f64;
    for k in &all_keys {
        let a = map_a.get(k).copied().unwrap_or(0.0);
        let b = map_b.get(k).copied().unwrap_or(0.0);
        dot += a * b;
        mag_a += a * a;
        mag_b += b * b;
    }
    let similarity = if mag_a > 0.0 && mag_b > 0.0 {
        dot / (mag_a.sqrt() * mag_b.sqrt())
    } else {
        0.0
    };

    DnaComparison {
        unique_a,
        unique_b,
        shared,
        similarity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query_gen::{Query, QueryCategory};

    fn make_result(response: &str) -> ProbeResult {
        ProbeResult {
            query: Query {
                text: "test".to_string(),
                category: QueryCategory::CategorySearch,
            },
            model: "test".to_string(),
            response: response.to_string(),
            error: None,
        }
    }

    #[test]
    fn extracts_keywords_from_responses() {
        let results = vec![
            make_result("Robinhood is a popular trading app with commission-free stock trading. It is easy to use and great for beginners."),
            make_result("Robinhood offers innovative fractional shares and a user-friendly mobile trading experience."),
            make_result("When comparing brokers, Robinhood stands out for its intuitive interface and zero-commission trading model."),
        ];

        let dna = extract_dna("Robinhood", &results);
        assert!(!dna.your_brand.is_empty(), "should extract keywords");

        // Should find trading-related keywords
        let keywords: Vec<&str> = dna.your_brand.iter().map(|k| k.keyword.as_str()).collect();
        assert!(
            keywords.iter().any(|k| k.contains("trading")),
            "should find 'trading' keyword, got: {:?}",
            keywords
        );
    }

    #[test]
    fn handles_no_mentions() {
        let results = vec![make_result("Fidelity and Schwab are great brokers.")];

        let dna = extract_dna("Robinhood", &results);
        assert!(dna.your_brand.is_empty());
    }

    #[test]
    fn detects_sentiment() {
        let results = vec![make_result(
            "Robinhood is the best and most popular trading app, very innovative and trusted.",
        )];

        let dna = extract_dna("Robinhood", &results);
        let positive_count = dna
            .your_brand
            .iter()
            .filter(|k| k.sentiment == "positive")
            .count();
        assert!(
            positive_count > 0,
            "should detect positive sentiment keywords"
        );
    }

    #[test]
    fn compare_dna_finds_shared_and_unique() {
        let a = vec![
            KeywordScore { keyword: "trading".into(), score: 5.0, sentiment: "neutral".into() },
            KeywordScore { keyword: "free".into(), score: 3.0, sentiment: "positive".into() },
            KeywordScore { keyword: "mobile".into(), score: 2.0, sentiment: "neutral".into() },
        ];
        let b = vec![
            KeywordScore { keyword: "trading".into(), score: 4.0, sentiment: "neutral".into() },
            KeywordScore { keyword: "research".into(), score: 6.0, sentiment: "neutral".into() },
            KeywordScore { keyword: "mobile".into(), score: 1.0, sentiment: "neutral".into() },
        ];

        let cmp = compare_dna(&a, &b);
        assert_eq!(cmp.shared.len(), 2); // trading, mobile
        assert_eq!(cmp.unique_a.len(), 1); // free
        assert_eq!(cmp.unique_b.len(), 1); // research
        assert!(cmp.similarity > 0.0 && cmp.similarity < 1.0);
    }

    #[test]
    fn compare_dna_identical_is_one() {
        let a = vec![
            KeywordScore { keyword: "x".into(), score: 3.0, sentiment: "neutral".into() },
        ];
        let cmp = compare_dna(&a, &a);
        assert!((cmp.similarity - 1.0).abs() < 1e-10);
    }

    #[test]
    fn compare_dna_disjoint_is_zero() {
        let a = vec![KeywordScore { keyword: "x".into(), score: 3.0, sentiment: "neutral".into() }];
        let b = vec![KeywordScore { keyword: "y".into(), score: 3.0, sentiment: "neutral".into() }];
        let cmp = compare_dna(&a, &b);
        assert!((cmp.similarity - 0.0).abs() < 1e-10);
    }
}
