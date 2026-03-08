use crate::analysis::AnalysisResult;
use anyhow::Result;

pub fn generate_html(brand: &str, industry: &str, analysis: &AnalysisResult) -> Result<String> {
    let overall_mention_rate = if analysis.brand_mentions.is_empty() {
        0.0
    } else {
        analysis
            .brand_mentions
            .iter()
            .filter(|m| m.mentioned)
            .count() as f64
            / analysis.brand_mentions.len() as f64
    };

    let mut model_rows = String::new();
    for som in &analysis.som_by_model {
        model_rows.push_str(&format!(
            r#"<tr><td>{}</td><td>{:.1}%</td><td>{:.1}</td><td>{}</td></tr>"#,
            som.model,
            som.mention_rate * 100.0,
            som.avg_position,
            som.total_probed
        ));
    }

    let mut category_rows = String::new();
    for som in &analysis.som_by_category {
        let color = if som.mention_rate > 0.6 {
            "#22c55e"
        } else if som.mention_rate > 0.3 {
            "#eab308"
        } else {
            "#ef4444"
        };
        category_rows.push_str(&format!(
            r#"<tr><td>{}</td><td><span style="color:{}; font-weight:bold">{:.1}%</span></td><td>{}</td></tr>"#,
            som.category,
            color,
            som.mention_rate * 100.0,
            som.total_probed
        ));
    }

    let mut competitor_rows = String::new();
    for comp in &analysis.competitor_scores {
        competitor_rows.push_str(&format!(
            r#"<tr><td>{}</td><td>{:.1}%</td><td>{:.1}</td><td>{:.1}%</td></tr>"#,
            comp.name,
            comp.mention_rate * 100.0,
            comp.avg_position,
            comp.positive_rate * 100.0
        ));
    }

    let mut insight_cards = String::new();
    for insight in &analysis.insights {
        let border_color = match insight.severity {
            crate::analysis::InsightSeverity::Critical => "#ef4444",
            crate::analysis::InsightSeverity::Warning => "#eab308",
            crate::analysis::InsightSeverity::Info => "#3b82f6",
            crate::analysis::InsightSeverity::Strength => "#22c55e",
        };
        insight_cards.push_str(&format!(
            r#"<div class="insight-card" style="border-left: 4px solid {}">
                <div class="insight-severity">{}</div>
                <div class="insight-category">{}</div>
                <div class="insight-finding">{}</div>
                <div class="insight-rec">💡 {}</div>
            </div>"#,
            border_color,
            insight.severity,
            insight.category,
            insight.finding,
            insight.recommendation
        ));
    }

    // Top missed queries
    let missed: Vec<&str> = analysis
        .brand_mentions
        .iter()
        .filter(|m| !m.mentioned)
        .take(10)
        .map(|m| m.query_text.as_str())
        .collect();

    let mut missed_list = String::new();
    for q in &missed {
        missed_list.push_str(&format!("<li>{}</li>", q));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>GEO Report: {brand}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: #0f172a;
            color: #e2e8f0;
            line-height: 1.6;
        }}
        .container {{ max-width: 1200px; margin: 0 auto; padding: 2rem; }}
        .header {{
            text-align: center;
            padding: 3rem 0;
            background: linear-gradient(135deg, #1e293b, #0f172a);
            border-bottom: 1px solid #334155;
        }}
        .header h1 {{ font-size: 2.5rem; color: #f8fafc; margin-bottom: 0.5rem; }}
        .header .subtitle {{ color: #94a3b8; font-size: 1.1rem; }}
        .score-hero {{
            display: flex;
            justify-content: center;
            gap: 3rem;
            padding: 2rem 0;
            margin: 2rem 0;
        }}
        .score-card {{
            text-align: center;
            padding: 1.5rem 2.5rem;
            background: #1e293b;
            border-radius: 12px;
            border: 1px solid #334155;
        }}
        .score-card .value {{
            font-size: 3rem;
            font-weight: 800;
            background: linear-gradient(135deg, #3b82f6, #8b5cf6);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }}
        .score-card .label {{ color: #94a3b8; font-size: 0.9rem; margin-top: 0.25rem; }}
        .section {{ margin: 2.5rem 0; }}
        .section h2 {{
            font-size: 1.5rem;
            color: #f8fafc;
            margin-bottom: 1rem;
            padding-bottom: 0.5rem;
            border-bottom: 2px solid #334155;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            background: #1e293b;
            border-radius: 8px;
            overflow: hidden;
        }}
        th {{ background: #334155; padding: 0.75rem 1rem; text-align: left; font-weight: 600; }}
        td {{ padding: 0.75rem 1rem; border-bottom: 1px solid #1e293b; }}
        tr:hover {{ background: #262f40; }}
        .insight-card {{
            background: #1e293b;
            padding: 1.25rem;
            border-radius: 8px;
            margin-bottom: 1rem;
        }}
        .insight-severity {{ font-size: 0.8rem; margin-bottom: 0.25rem; }}
        .insight-category {{ font-weight: 600; font-size: 1.1rem; color: #f8fafc; }}
        .insight-finding {{ color: #cbd5e1; margin: 0.5rem 0; }}
        .insight-rec {{ color: #94a3b8; font-style: italic; }}
        .missed-queries {{
            background: #1e293b;
            border-radius: 8px;
            padding: 1.5rem;
        }}
        .missed-queries li {{
            padding: 0.4rem 0;
            color: #ef4444;
            list-style: none;
        }}
        .missed-queries li::before {{ content: "✗ "; }}
        .footer {{
            text-align: center;
            color: #475569;
            padding: 2rem 0;
            font-size: 0.85rem;
        }}
    </style>
</head>
<body>
    <div class="header">
        <h1>🎯 GEO Report: {brand}</h1>
        <div class="subtitle">Generative Engine Optimization Analysis — {industry} vertical</div>
    </div>

    <div class="container">
        <div class="score-hero">
            <div class="score-card">
                <div class="value">{som_pct:.0}%</div>
                <div class="label">Share of Model (SOM)</div>
            </div>
            <div class="score-card">
                <div class="value">{total_queries}</div>
                <div class="label">Queries Tested</div>
            </div>
            <div class="score-card">
                <div class="value">{total_responses}</div>
                <div class="label">LLM Responses</div>
            </div>
        </div>

        <div class="section">
            <h2>📊 SOM by Model</h2>
            <table>
                <thead><tr><th>Model</th><th>Mention Rate</th><th>Avg Position</th><th>Queries</th></tr></thead>
                <tbody>{model_rows}</tbody>
            </table>
        </div>

        <div class="section">
            <h2>📂 SOM by Query Category</h2>
            <table>
                <thead><tr><th>Category</th><th>Mention Rate</th><th>Queries</th></tr></thead>
                <tbody>{category_rows}</tbody>
            </table>
        </div>

        <div class="section">
            <h2>⚔️ Competitor Comparison</h2>
            <table>
                <thead><tr><th>Competitor</th><th>Mention Rate</th><th>Avg Position</th><th>Positive %</th></tr></thead>
                <tbody>{competitor_rows}</tbody>
            </table>
        </div>

        <div class="section">
            <h2>💡 Key Insights & Recommendations</h2>
            {insight_cards}
        </div>

        <div class="section">
            <h2>❌ Top Missed Queries</h2>
            <p style="color: #94a3b8; margin-bottom: 1rem;">Queries where {brand} was NOT mentioned by any LLM:</p>
            <div class="missed-queries">
                <ul>{missed_list}</ul>
            </div>
        </div>

        <div class="footer">
            Generated by UserGrow 🦀 — Powered by Rust
        </div>
    </div>
</body>
</html>"#,
        brand = brand,
        industry = industry,
        som_pct = overall_mention_rate * 100.0,
        total_queries = analysis.total_queries,
        total_responses = analysis.total_responses,
        model_rows = model_rows,
        category_rows = category_rows,
        competitor_rows = competitor_rows,
        insight_cards = insight_cards,
        missed_list = missed_list,
    );

    let path = format!("geo_report_{}.html", brand.to_lowercase().replace(' ', "_"));
    std::fs::write(&path, &html)?;

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::*;

    #[test]
    fn generates_html_report() {
        let analysis = AnalysisResult {
            brand: "TestBrand".to_string(),
            total_queries: 10,
            total_responses: 20,
            brand_mentions: vec![],
            competitor_scores: vec![],
            som_by_model: vec![ModelSOM {
                model: "gpt-4o".to_string(),
                mention_rate: 0.5,
                avg_position: 2.0,
                total_probed: 10,
            }],
            som_by_category: vec![CategorySOM {
                category: "Category Search".to_string(),
                mention_rate: 0.6,
                total_probed: 5,
            }],
            insights: vec![Insight {
                category: "Overall".to_string(),
                finding: "Test finding".to_string(),
                recommendation: "Test rec".to_string(),
                severity: InsightSeverity::Info,
            }],
        };

        let path = generate_html("TestBrand", "finance", &analysis).unwrap();
        assert!(path.contains("testbrand"));

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("TestBrand"));
        assert!(content.contains("GEO Report"));
        assert!(content.contains("gpt-4o"));
        assert!(content.contains("Test finding"));

        // Cleanup
        std::fs::remove_file(&path).ok();
    }
}
