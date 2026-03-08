use crate::persona::PersonaProbe;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    pub text: String,
    pub category: QueryCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryCategory {
    DirectComparison, // "Robinhood vs Fidelity"
    CategorySearch,   // "best trading app 2026"
    AttributeSearch,  // "lowest fee brokerage"
    ScenarioSearch,   // "best app for beginner investor"
    Recommendation,   // "which broker should I use"
}

impl std::fmt::Display for QueryCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryCategory::DirectComparison => write!(f, "Direct Comparison"),
            QueryCategory::CategorySearch => write!(f, "Category Search"),
            QueryCategory::AttributeSearch => write!(f, "Attribute Search"),
            QueryCategory::ScenarioSearch => write!(f, "Scenario Search"),
            QueryCategory::Recommendation => write!(f, "Recommendation"),
        }
    }
}

/// Generate queries from persona probes (for persona-based analysis)
#[allow(dead_code)]
pub fn generate_persona_queries(probes: &[PersonaProbe]) -> Vec<Query> {
    probes
        .iter()
        .map(|p| Query {
            text: p.prompt.clone(),
            category: QueryCategory::Recommendation, // persona probes are recommendation-type
        })
        .collect()
}

/// Generate realistic customer queries for a brand in its industry
pub async fn generate_queries(
    brand: &str,
    industry: &str,
    competitors: &[String],
    city: Option<&str>, // NEW: optional city for localization
) -> Result<Vec<Query>> {
    let mut queries = Vec::new();

    // Direct Comparison queries
    for comp in competitors {
        queries.push(Query {
            text: format!("{} vs {}", brand, comp),
            category: QueryCategory::DirectComparison,
        });
        queries.push(Query {
            text: format!("{} vs {} which is better", brand, comp),
            category: QueryCategory::DirectComparison,
        });
        queries.push(Query {
            text: format!("{} or {} for beginners", brand, comp),
            category: QueryCategory::DirectComparison,
        });
    }

    // Category Search queries
    let category_templates: Vec<String> = match industry {
        "finance" => vec![
            "best trading app".to_string(),
            "best stock trading platform".to_string(),
            "best investment app".to_string(),
            "best brokerage account".to_string(),
            "best app for stocks".to_string(),
            "top online brokers".to_string(),
            "best mobile trading app".to_string(),
            "best free trading app".to_string(),
            "best app for day trading".to_string(),
            "best crypto trading platform".to_string(),
        ],
        _ => vec![
            format!("best {} platform", industry),
            format!("top {} tools", industry),
            format!("best {} service", industry),
        ],
    };

    for template in &category_templates {
        let mut query_text = format!("{} 2026", template);
        if let Some(c) = city {
            query_text = format!("{} in {}", query_text, c);
        }
        queries.push(Query {
            text: query_text,
            category: QueryCategory::CategorySearch,
        });
    }

    // Attribute Search queries
    let attribute_templates: Vec<String> = match industry {
        "finance" => vec![
            "lowest fee brokerage".to_string(),
            "commission free trading app".to_string(),
            "zero fee stock trading".to_string(),
            "cheapest options trading".to_string(),
            "best fractional shares app".to_string(),
            "easiest trading app to use".to_string(),
            "most secure trading platform".to_string(),
            "fastest trade execution app".to_string(),
            "best trading app with research tools".to_string(),
            "best trading app with no minimum deposit".to_string(),
        ],
        _ => vec![
            format!("cheapest {} service", industry),
            format!("easiest {} tool", industry),
            format!("most reliable {}", industry),
        ],
    };

    for template in &attribute_templates {
        queries.push(Query {
            text: template.to_string(),
            category: QueryCategory::AttributeSearch,
        });
    }

    // Scenario Search queries
    let scenario_templates: Vec<String> = match industry {
        "finance" => vec![
            "I want to start investing with $100".to_string(),
            "best app for a college student to invest".to_string(),
            "how to start trading stocks as a beginner".to_string(),
            "what app should I use to buy my first stock".to_string(),
            "best platform for passive investing".to_string(),
            "I want to trade options where do I start".to_string(),
            "best app for retirement savings".to_string(),
            "where to invest small amounts of money".to_string(),
            "best app for automated investing".to_string(),
            "how to invest in index funds easily".to_string(),
        ],
        _ => vec![
            format!("I need a {} solution for small business", industry),
            format!("getting started with {}", industry),
        ],
    };

    for template in &scenario_templates {
        let mut query_text = template.to_string();
        if let Some(c) = city {
            query_text = format!("{} in {}", query_text, c);
        }
        queries.push(Query {
            text: query_text,
            category: QueryCategory::ScenarioSearch,
        });
    }

    // Recommendation queries
    let rec_templates: Vec<String> = match industry {
        "finance" => vec![
            "which broker should I use".to_string(),
            "recommend me a trading app".to_string(),
            "what trading platform do you suggest".to_string(),
            "which investment app is the best right now".to_string(),
            "should I use Robinhood or something else".to_string(),
            "what do experts recommend for stock trading".to_string(),
            "best reviewed trading app".to_string(),
            "most popular trading app among millennials".to_string(),
            "which brokerage has the best mobile app".to_string(),
            "what trading app do most people use".to_string(),
        ],
        _ => vec![
            format!("recommend a {} tool", industry),
            format!("what {} should I use", industry),
        ],
    };

    for template in &rec_templates {
        let mut query_text = template.to_string();
        if let Some(c) = city {
            query_text = format!("{} in {}", query_text, c);
        }
        queries.push(Query {
            text: query_text,
            category: QueryCategory::Recommendation,
        });
    }

    // Brand-specific queries
    queries.push(Query {
        text: format!("is {} good", brand),
        category: QueryCategory::Recommendation,
    });
    queries.push(Query {
        text: format!("{} review", brand),
        category: QueryCategory::Recommendation,
    });
    queries.push(Query {
        text: format!("is {} safe to use", brand),
        category: QueryCategory::AttributeSearch,
    });
    queries.push(Query {
        text: format!("{} pros and cons", brand),
        category: QueryCategory::DirectComparison,
    });

    Ok(queries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn generates_queries_for_finance() {
        let competitors = vec!["Schwab".to_string(), "Fidelity".to_string()];
        let queries = generate_queries("Robinhood", "finance", &competitors, None)
            .await
            .unwrap();

        assert!(!queries.is_empty(), "should generate queries");
        assert!(
            queries.len() > 30,
            "should generate 30+ queries for finance"
        );

        // Should have all categories
        let has_comparison = queries
            .iter()
            .any(|q| matches!(q.category, QueryCategory::DirectComparison));
        let has_category = queries
            .iter()
            .any(|q| matches!(q.category, QueryCategory::CategorySearch));
        let has_attribute = queries
            .iter()
            .any(|q| matches!(q.category, QueryCategory::AttributeSearch));
        let has_scenario = queries
            .iter()
            .any(|q| matches!(q.category, QueryCategory::ScenarioSearch));
        let has_rec = queries
            .iter()
            .any(|q| matches!(q.category, QueryCategory::Recommendation));

        assert!(has_comparison, "should have comparison queries");
        assert!(has_category, "should have category queries");
        assert!(has_attribute, "should have attribute queries");
        assert!(has_scenario, "should have scenario queries");
        assert!(has_rec, "should have recommendation queries");
    }

    #[tokio::test]
    async fn generates_brand_specific_queries() {
        let queries = generate_queries("Robinhood", "finance", &[], None)
            .await
            .unwrap();

        let brand_queries: Vec<_> = queries
            .iter()
            .filter(|q| q.text.to_lowercase().contains("robinhood"))
            .collect();

        assert!(
            brand_queries.len() >= 3,
            "should have at least 3 brand-specific queries, got {}",
            brand_queries.len()
        );
    }

    #[tokio::test]
    async fn generates_competitor_comparisons() {
        let competitors = vec!["Schwab".to_string()];
        let queries = generate_queries("Robinhood", "finance", &competitors, None)
            .await
            .unwrap();

        let comparison_queries: Vec<_> = queries
            .iter()
            .filter(|q| {
                matches!(q.category, QueryCategory::DirectComparison) && q.text.contains("Schwab")
            })
            .collect();

        assert!(
            !comparison_queries.is_empty(),
            "should generate comparison queries with competitors"
        );
    }

    #[tokio::test]
    async fn works_with_non_finance_industry() {
        let queries = generate_queries("Acme", "logistics", &[], None)
            .await
            .unwrap();

        assert!(
            !queries.is_empty(),
            "should generate queries for any industry"
        );
    }

    #[tokio::test]
    async fn localizes_queries_with_city() {
        let queries = generate_queries("Robinhood", "finance", &[], Some("New York"))
            .await
            .unwrap();

        let localized = queries
            .iter()
            .filter(|q| q.text.contains("New York"))
            .count();

        assert!(
            localized > 0,
            "should localize some queries when city is provided"
        );
    }

    #[test]
    fn generates_persona_queries() {
        use crate::persona::{Language, Persona, PersonaProbe};

        let probes = vec![
            PersonaProbe {
                persona: Persona {
                    name: "Test".to_string(),
                    age: 30,
                    gender: "Female".to_string(),
                    ethnicity: "Asian".to_string(),
                    income_bracket: "$50k".to_string(),
                    immigration_status: "Native".to_string(),
                    language: Language::English,
                    occupation: "Engineer".to_string(),
                    description: "Test persona".to_string(),
                },
                prompt: "Can you recommend trading apps?".to_string(),
                language: Language::English,
            },
            PersonaProbe {
                persona: Persona {
                    name: "Test2".to_string(),
                    age: 40,
                    gender: "Male".to_string(),
                    ethnicity: "Hispanic".to_string(),
                    income_bracket: "$60k".to_string(),
                    immigration_status: "Immigrant".to_string(),
                    language: Language::Spanish,
                    occupation: "Teacher".to_string(),
                    description: "Test persona 2".to_string(),
                },
                prompt: "¿Puedes recomendar aplicaciones?".to_string(),
                language: Language::Spanish,
            },
        ];

        let queries = generate_persona_queries(&probes);

        assert_eq!(queries.len(), 2, "should generate query for each probe");
        assert_eq!(queries[0].text, "Can you recommend trading apps?");
        assert_eq!(queries[1].text, "¿Puedes recomendar aplicaciones?");
        assert!(queries
            .iter()
            .all(|q| matches!(q.category, QueryCategory::Recommendation)));
    }
}
