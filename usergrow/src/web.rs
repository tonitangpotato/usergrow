use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use crate::types::*;

// ── App State ──────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct AppState {
    pub tavily_key: String,
    pub openai_key: Option<String>,
    pub gemini_key: Option<String>,
    pub anthropic_key: Option<String>,
    // Perplexity removed — not available for hackathon
    pub glm_key: Option<String>,
    pub engram: Option<Arc<crate::engram::EngramClient>>,
    pub kg: Option<Arc<crate::knowledge_graph::KnowledgeGraph>>,
    /// Active jobs: job_id → sender for SSE events
    pub jobs: Arc<Mutex<HashMap<String, mpsc::Sender<ProgressEvent>>>>,
    /// Completed reports: job_id → report
    pub reports: Arc<Mutex<HashMap<String, FullReport>>>,
}

// ── Router ─────────────────────────────────────────────────────────────

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/analyze", post(start_analysis))
        .route("/api/analyze/stream/{job_id}", get(stream_progress))
        .route("/api/report/{job_id}", get(get_report))
        .route("/api/kg/graph", get(kg_full_graph))
        .route("/api/kg/entities/{entity_type}", get(kg_entities))
        .route("/api/kg/entity/{entity_id}/relations", get(kg_relations))
        .route("/api/kg/entity/{entity_id}/snapshots", get(kg_snapshots))
        .route("/api/kg/backfill", post(kg_backfill))
        .route("/api/reports/list", get(list_reports))
        .fallback_service(ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

// ── Handlers ───────────────────────────────────────────────────────────

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn start_analysis(
    State(state): State<AppState>,
    Json(req): Json<AnalyzeRequest>,
) -> Json<AnalyzeResponse> {
    let job_id = uuid::Uuid::new_v4().to_string();
    let (tx, _rx) = mpsc::channel::<ProgressEvent>(64);

    state.jobs.lock().await.insert(job_id.clone(), tx.clone());

    let jid = job_id.clone();
    let st = state.clone();
    tokio::spawn(async move {
        run_analysis(st, jid, req).await;
    });

    Json(AnalyzeResponse {
        job_id: job_id.clone(),
    })
}

async fn stream_progress(
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> impl IntoResponse {
    let (tx, rx) = mpsc::channel::<ProgressEvent>(64);
    state.jobs.lock().await.insert(job_id.clone(), tx);

    let stream = ReceiverStream::new(rx).map(|evt| {
        let data = serde_json::to_string(&evt).unwrap_or_default();
        Ok::<_, std::convert::Infallible>(Event::default().data(data))
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn get_report(
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> impl IntoResponse {
    match state.reports.lock().await.get(&job_id) {
        Some(report) => Json(serde_json::to_value(report).unwrap()).into_response(),
        None => (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Report not found"})),
        )
            .into_response(),
    }
}

// ── Knowledge Graph Handlers ───────────────────────────────────────────

async fn kg_full_graph(State(state): State<AppState>) -> impl IntoResponse {
    let Some(ref kg) = state.kg else {
        return (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Knowledge Graph not available"})),
        )
            .into_response();
    };
    let kg = Arc::clone(kg);
    match tokio::task::spawn_blocking(move || kg.full_graph()).await {
        Ok(Ok((entities, relations))) => Json(serde_json::json!({
            "entities": entities,
            "relations": relations,
        }))
        .into_response(),
        _ => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Failed to query KG"})),
        )
            .into_response(),
    }
}

async fn kg_entities(
    State(state): State<AppState>,
    Path(entity_type): Path<String>,
) -> impl IntoResponse {
    let Some(ref kg) = state.kg else {
        return Json(serde_json::json!({"error": "KG not available"})).into_response();
    };
    let kg = Arc::clone(kg);
    match tokio::task::spawn_blocking(move || kg.entities_by_type(&entity_type)).await {
        Ok(Ok(entities)) => Json(serde_json::json!({"entities": entities})).into_response(),
        _ => Json(serde_json::json!({"error": "Query failed"})).into_response(),
    }
}

async fn kg_relations(
    State(state): State<AppState>,
    Path(entity_id): Path<String>,
) -> impl IntoResponse {
    let Some(ref kg) = state.kg else {
        return Json(serde_json::json!({"error": "KG not available"})).into_response();
    };
    let kg = Arc::clone(kg);
    match tokio::task::spawn_blocking(move || kg.relations_for(&entity_id)).await {
        Ok(Ok(relations)) => Json(serde_json::json!({"relations": relations})).into_response(),
        _ => Json(serde_json::json!({"error": "Query failed"})).into_response(),
    }
}

async fn kg_snapshots(
    State(state): State<AppState>,
    Path(entity_id): Path<String>,
) -> impl IntoResponse {
    let Some(ref kg) = state.kg else {
        return Json(serde_json::json!({"error": "KG not available"})).into_response();
    };
    let kg = Arc::clone(kg);
    match tokio::task::spawn_blocking(move || kg.snapshots_for(&entity_id)).await {
        Ok(Ok(snapshots)) => Json(serde_json::json!({"snapshots": snapshots})).into_response(),
        _ => Json(serde_json::json!({"error": "Query failed"})).into_response(),
    }
}

// ── Backfill & List ────────────────────────────────────────────────────

async fn kg_backfill(
    State(state): State<AppState>,
    Json(reports): Json<Vec<FullReport>>,
) -> impl IntoResponse {
    let Some(ref kg) = state.kg else {
        return Json(serde_json::json!({"error": "KG not available"}));
    };
    let kg = Arc::clone(kg);
    let count = reports.len();
    let result = tokio::task::spawn_blocking(move || {
        let mut ok = 0u32;
        let mut fail = 0u32;
        for r in &reports {
            match kg.ingest_report(r) {
                Ok(_) => ok += 1,
                Err(e) => {
                    eprintln!("⚠️  KG backfill error for {}: {}", r.brand, e);
                    fail += 1;
                }
            }
        }
        (ok, fail)
    })
    .await
    .unwrap_or((0, count as u32));
    Json(serde_json::json!({
        "total": count,
        "ingested": result.0,
        "failed": result.1
    }))
}

async fn list_reports(State(state): State<AppState>) -> Json<Vec<FullReport>> {
    let reports = state.reports.lock().await;
    Json(reports.values().cloned().collect())
}

// ── Analysis Pipeline ──────────────────────────────────────────────────

async fn run_analysis(state: AppState, job_id: String, req: AnalyzeRequest) {
    let send = |step: &str, status: &str, detail: Option<&str>| {
        let state = state.clone();
        let job_id = job_id.clone();
        let evt = ProgressEvent {
            step: step.to_string(),
            status: status.to_string(),
            detail: detail.map(|s| s.to_string()),
            done: None,
            total: None,
            data: None,
        };
        async move {
            if let Some(tx) = state.jobs.lock().await.get(&job_id) {
                let _ = tx.send(evt).await;
            }
        }
    };

    let city = req.city.as_deref().unwrap_or("");

    // Step 0: Discover competitors via Tavily
    send("competitors", "progress", Some("Discovering competitors...")).await;
    let tavily = crate::tavily::TavilyClient::new(state.tavily_key.clone());
    
    // Collect Tavily sources for evidence
    let mut tavily_sources = Vec::new();
    let query = if city.is_empty() {
        format!("top {} companies competitors to {} 2026", req.industry, req.brand)
    } else {
        format!("top {} companies competitors to {} in {} 2026", req.industry, req.brand, city)
    };
    
    let competitors = match tavily.search(&query, 10).await {
        Ok(search_resp) => {
            // Collect Tavily sources
            for result in &search_resp.results {
                tavily_sources.push(crate::types::TavilySource {
                    url: result.url.clone(),
                    title: result.title.clone(),
                    snippet: result.content.chars().take(200).collect(),
                });
            }
            
            // Extract competitor names
            let mut raw_texts = Vec::new();
            if let Some(ref answer) = search_resp.answer {
                raw_texts.push(answer.clone());
            }
            for result in &search_resp.results {
                raw_texts.push(result.content.clone());
            }
            extract_competitor_names(&raw_texts, &req.brand)
        }
        Err(_) => Vec::new(),
    };
    send("competitors", "complete", Some(&format!("{} competitors found", competitors.len()))).await;

    // Step 1: Generate queries
    send("query_gen", "progress", Some("Generating queries...")).await;
    let queries = match crate::query_gen::generate_queries(
        &req.brand,
        &req.industry,
        &competitors,
        req.city.as_deref(),
    )
    .await
    {
        Ok(q) => q,
        Err(_) => {
            send("query_gen", "error", Some("Failed to generate queries")).await;
            return;
        }
    };

    // Also generate persona probes
    let probes = crate::persona::build_probe_matrix(&req.brand, &req.industry, city);
    let persona_queries = crate::query_gen::generate_persona_queries(&probes);
    let all_queries: Vec<_> = queries
        .iter()
        .chain(persona_queries.iter())
        .cloned()
        .collect();

    send(
        "query_gen",
        "complete",
        Some(&format!(
            "{} queries + {} persona probes",
            queries.len(),
            persona_queries.len()
        )),
    )
    .await;

    // Step 2: Probe LLMs
    send("probe", "progress", Some("Probing LLMs...")).await;
    // Use real Anthropic key if available, otherwise fall back to GLM key via z.ai
    let anthropic_key = state.anthropic_key.clone().or_else(|| state.glm_key.clone());
    // Always use GLM key for GLM probes (z.ai Haiku mapping)
    let probe_results = match crate::llm_probe::probe_all(
        &all_queries,
        &state.openai_key,
        &state.gemini_key,
        &anthropic_key,
        &None, // perplexity not available
        &state.glm_key,
    )
    .await
    {
        Ok(r) => r,
        Err(_) => {
            send("probe", "error", Some("Probing failed")).await;
            return;
        }
    };
    send(
        "probe",
        "complete",
        Some(&format!("{} responses", probe_results.len())),
    )
    .await;
    
    // Collect LLM evidence from probe results
    let llm_evidence: Vec<crate::types::LLMEvidence> = probe_results
        .iter()
        .filter(|r| r.error.is_none())
        .take(20) // Limit to first 20 to avoid bloating payload
        .map(|r| crate::types::LLMEvidence {
            model: r.model.clone(),
            query: r.query.text.clone(),
            response_snippet: r.response.chars().take(500).collect(),
            full_response: r.response.clone(),
        })
        .collect();

    // Step 3: Analyze
    send("analysis", "progress", Some("Analyzing responses...")).await;
    let analysis = match crate::analysis::analyze(&req.brand, &competitors, &probe_results) {
        Ok(a) => a,
        Err(_) => {
            send("analysis", "error", Some("Analysis failed")).await;
            return;
        }
    };
    send("analysis", "complete", None).await;

    // Step 4: Brand DNA
    send("brand_dna", "progress", Some("Extracting brand DNA...")).await;
    let brand_dna = crate::brand_dna::extract_dna(&req.brand, &probe_results);
    send("brand_dna", "complete", None).await;

    // Step 5: Reality check
    send(
        "reality_check",
        "progress",
        Some("Checking real-world data..."),
    )
    .await;
    let reality = crate::reality::check_reality(
        &tavily,
        &req.brand,
        &competitors,
        &req.industry,
        city,
    )
    .await
    .unwrap_or_default();
    send("reality_check", "complete", None).await;

    // Step 6: Build recommendations (with schema suggestions)
    let recommendations = build_recommendations(&analysis, &brand_dna, &reality, &req.industry);

    // Calculate visibility score
    let mentioned = analysis
        .brand_mentions
        .iter()
        .filter(|m| m.mentioned)
        .count();
    let visibility_score = if analysis.brand_mentions.is_empty() {
        0
    } else {
        ((mentioned as f64 / analysis.brand_mentions.len() as f64) * 100.0) as u32
    };

    // Build SOM
    let mut share_of_model = ShareOfModel::default();
    for som in &analysis.som_by_model {
        match som.model.as_str() {
            "gpt-4o-mini" => share_of_model.chatgpt = som.mention_rate * 100.0,
            "gemini-2.0-flash" => share_of_model.gemini = som.mention_rate * 100.0,
            m if m.starts_with("claude") || m.starts_with("Claude") => share_of_model.claude = som.mention_rate * 100.0,
            "perplexity-sonar" => share_of_model.perplexity = som.mention_rate * 100.0,
            "GLM-4.7" => share_of_model.glm_47 = som.mention_rate * 100.0,
            "GLM-4.5" => share_of_model.glm_45 = som.mention_rate * 100.0,
            _ => {}
        }
    }

    // Step 7: Persona heatmap
    send(
        "persona_heatmap",
        "progress",
        Some("Building persona heatmap..."),
    )
    .await;
    let persona_heatmap = build_persona_heatmap(&req.brand, &probes, &probe_results);
    send("persona_heatmap", "complete", None).await;

    // Step 8: Engram memory
    send("engram", "progress", Some("Consulting memory...")).await;
    let engram_status = if let Some(ref engram) = state.engram {
        let summary = format!(
            "ChatGPT={:.1}% Gemini={:.1}% Perplexity={:.1}% Claude={:.1}%",
            share_of_model.chatgpt,
            share_of_model.gemini,
            share_of_model.perplexity,
            share_of_model.claude
        );

        let engram_clone = Arc::clone(engram);
        let brand = req.brand.clone();
        let industry = req.industry.clone();
        let city_owned = city.to_string();

        // Store + build status in blocking context (SQLite isn't Send)
        let status = tokio::task::spawn_blocking(move || {
            let _ = engram_clone.store_scan(&brand, &industry, &city_owned, visibility_score, &summary);
            engram_clone.build_status(&brand, visibility_score)
        })
        .await
        .unwrap_or(EngramStatus {
            total_scans: 1,
            drift: None,
            confidence: "initial".to_string(),
        });

        send(
            "engram",
            "complete",
            Some(&format!("{} total scans", status.total_scans)),
        )
        .await;
        status
    } else {
        send("engram", "complete", Some("Memory disabled")).await;
        EngramStatus {
            total_scans: 1,
            drift: None,
            confidence: "initial".to_string(),
        }
    };

    // Build final report with evidence
    let evidence = crate::types::Evidence {
        llm_responses: llm_evidence,
        tavily_sources,
    };
    
    let report = FullReport {
        brand: req.brand.clone(),
        industry: req.industry.clone(),
        city: req.city.unwrap_or_default(),
        visibility_score,
        share_of_model,
        persona_heatmap,
        brand_dna,
        reality_check: reality,
        recommendations,
        engram: engram_status,
        evidence,
    };

    // Ingest into Knowledge Graph
    if let Some(ref kg) = state.kg {
        let kg_clone = Arc::clone(kg);
        let report_clone = report.clone();
        let _ = tokio::task::spawn_blocking(move || {
            if let Err(e) = kg_clone.ingest_report(&report_clone) {
                eprintln!("⚠️  KG ingest error: {}", e);
            }
        })
        .await;
    }

    // Store report
    state
        .reports
        .lock()
        .await
        .insert(job_id.clone(), report.clone());

    // Send final event with full report
    if let Some(tx) = state.jobs.lock().await.get(&job_id) {
        let _ = tx
            .send(ProgressEvent {
                step: "done".to_string(),
                status: "complete".to_string(),
                detail: None,
                done: None,
                total: None,
                data: Some(serde_json::to_value(&report).unwrap_or_default()),
            })
            .await;
    }
}

// ── Persona Heatmap Builder ────────────────────────────────────────────

fn build_persona_heatmap(
    brand: &str,
    probes: &[crate::persona::PersonaProbe],
    probe_results: &[crate::llm_probe::ProbeResult],
) -> Vec<PersonaHeatmapEntry> {
    use std::collections::HashSet;

    let brand_lower = brand.to_lowercase();

    // Collect unique persona names
    let mut seen = HashSet::new();
    let mut entries: Vec<PersonaHeatmapEntry> = Vec::new();

    for probe in probes {
        let name = &probe.persona.name;
        if !seen.insert(name.clone()) {
            continue;
        }

        let desc = &probe.persona.description;
        let mut results: HashMap<String, PersonaCellResult> = HashMap::new();

        // Find all probe results matching this persona's prompts
        let persona_probes: Vec<&crate::persona::PersonaProbe> = probes
            .iter()
            .filter(|p| p.persona.name == *name)
            .collect();

        for pp in &persona_probes {
            let lang_code = match pp.language {
                crate::persona::Language::English => "EN",
                crate::persona::Language::Spanish => "ES",
                crate::persona::Language::Chinese => "ZH",
                crate::persona::Language::Hindi => "HI",
            };

            // Find matching probe results (match on query text)
            for result in probe_results {
                if result.error.is_some() || result.query.text != pp.prompt {
                    continue;
                }

                let model_name = match result.model.as_str() {
                    "gpt-4o-mini" => "GPT",
                    m if m.starts_with("gemini") => "Gemini",
                    m if m.starts_with("Claude") || m.starts_with("claude") => "Claude",
                    "GLM-4.7" => "GLM-4.7",
                    "GLM-4.5" => "GLM-4.5",
                    m if m.starts_with("GLM") => "GLM",
                    "perplexity-sonar" => "Perplexity",
                    other => other,
                };

                let key = format!("{}-{}", model_name, lang_code);
                let mentioned = result.response.to_lowercase().contains(&brand_lower);
                let rank = if mentioned {
                    crate::analysis::find_brand_position(&result.response, brand)
                        .map(|p| p as u32)
                } else {
                    None
                };

                results.insert(key, PersonaCellResult { mentioned, rank });
            }
        }

        entries.push(PersonaHeatmapEntry {
            persona_name: format!("{}, {}", name, probe.persona.age),
            persona_description: desc.clone(),
            results,
        });
    }

    entries
}

// ── Recommendations ────────────────────────────────────────────────────

fn build_recommendations(
    analysis: &crate::analysis::AnalysisResult,
    dna: &DnaReport,
    reality: &RealityCheckReport,
    industry: &str,
) -> Vec<Recommendation> {
    let mut recs: Vec<Recommendation> = analysis
        .insights
        .iter()
        .map(|i| {
            let priority = match i.severity {
                crate::analysis::InsightSeverity::Critical => "critical",
                crate::analysis::InsightSeverity::Warning => "important",
                _ => "nice",
            };
            Recommendation {
                priority: priority.to_string(),
                text: i.recommendation.clone(),
                evidence: i.finding.clone(),
            }
        })
        .collect();

    // Reality-based recommendations
    if let Some(ref rating) = reality.your_rating {
        if reality.bias_score > 30.0 {
            recs.push(Recommendation {
                priority: "critical".to_string(),
                text: format!(
                    "AI perception diverges significantly from your real-world rating ({:.1}/5). \
                     Build more structured data (schema.org, FAQ pages) to help AI models \
                     accurately represent your brand.",
                    rating.score
                ),
                evidence: format!("Bias score: {:.0}/100", reality.bias_score),
            });
        }
        // Check if competitors with lower ratings are ranked higher by AI
        for comp in &reality.competitors {
            if let Some(comp_rating) = comp.real_rating {
                if comp_rating < rating.score && comp.ai_rank.unwrap_or(99) < reality.ai_rank.unwrap_or(1) {
                    recs.push(Recommendation {
                        priority: "important".to_string(),
                        text: format!(
                            "{} has a lower rating ({:.1}\u{2605}) but AI ranks it higher. \
                             Study their online content strategy and digital footprint.",
                            comp.name, comp_rating
                        ),
                        evidence: format!(
                            "Your rating: {:.1}\u{2605} vs {}: {:.1}\u{2605}",
                            rating.score, comp.name, comp_rating
                        ),
                    });
                }
            }
        }
    }

    // DNA-based recommendations
    let negative_keywords: Vec<&str> = dna
        .your_brand
        .iter()
        .filter(|k| k.sentiment == "negative")
        .map(|k| k.keyword.as_str())
        .collect();
    if !negative_keywords.is_empty() {
        let top_neg: Vec<&str> = negative_keywords.into_iter().take(3).collect();
        recs.push(Recommendation {
            priority: "important".to_string(),
            text: format!(
                "AI associates negative terms with your brand: {}. \
                 Create positive content addressing these perceptions.",
                top_neg.join(", ")
            ),
            evidence: "Extracted from Brand DNA analysis across multiple AI models.".to_string(),
        });
    }

    if recs.is_empty() {
        recs.push(Recommendation {
            priority: "nice".to_string(),
            text: "Run a full scan with all LLM providers for comprehensive results.".to_string(),
            evidence: "No insights generated yet.".to_string(),
        });
    }
    
    // Add schema.org structured data recommendations
    let schema_rec = generate_schema_recommendation(industry);
    recs.push(schema_rec);

    recs
}

/// Generate schema.org structured data recommendation based on industry
fn generate_schema_recommendation(industry: &str) -> Recommendation {
    let (schema_type, example) = match industry.to_lowercase().as_str() {
        s if s.contains("restaurant") || s.contains("food") => (
            "LocalBusiness + Restaurant schema",
            r#"Add to your website: <script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Restaurant",
  "name": "Your Restaurant Name",
  "address": { "@type": "PostalAddress", "streetAddress": "...", "addressLocality": "...", "postalCode": "..." },
  "telephone": "+1-555-123-4567",
  "servesCuisine": "Italian",
  "priceRange": "$$",
  "aggregateRating": { "@type": "AggregateRating", "ratingValue": "4.5", "reviewCount": "250" }
}</script>"#
        ),
        s if s.contains("healthcare") || s.contains("medical") || s.contains("dental") => (
            "MedicalBusiness + Physician schema",
            r#"Add to your website: <script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "MedicalBusiness",
  "name": "Your Practice Name",
  "medicalSpecialty": "Cardiology",
  "address": { "@type": "PostalAddress", "streetAddress": "...", "addressLocality": "...", "postalCode": "..." },
  "telephone": "+1-555-123-4567",
  "aggregateRating": { "@type": "AggregateRating", "ratingValue": "4.8", "reviewCount": "150" }
}</script>"#
        ),
        s if s.contains("hotel") || s.contains("lodging") => (
            "Hotel + LodgingBusiness schema",
            r#"Add to your website: <script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Hotel",
  "name": "Your Hotel Name",
  "address": { "@type": "PostalAddress", "streetAddress": "...", "addressLocality": "...", "postalCode": "..." },
  "starRating": { "@type": "Rating", "ratingValue": "4" },
  "priceRange": "$$$",
  "amenityFeature": [{ "@type": "LocationFeatureSpecification", "name": "Free WiFi" }]
}</script>"#
        ),
        s if s.contains("retail") || s.contains("ecommerce") => (
            "Product + Offer schema",
            r#"Add to product pages: <script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Product",
  "name": "Product Name",
  "brand": { "@type": "Brand", "name": "Your Brand" },
  "offers": { "@type": "Offer", "price": "29.99", "priceCurrency": "USD", "availability": "https://schema.org/InStock" },
  "aggregateRating": { "@type": "AggregateRating", "ratingValue": "4.5", "reviewCount": "89" }
}</script>"#
        ),
        s if s.contains("legal") => (
            "ProfessionalService + LegalService schema",
            r#"Add to your website: <script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "LegalService",
  "name": "Your Firm Name",
  "address": { "@type": "PostalAddress", "streetAddress": "...", "addressLocality": "...", "postalCode": "..." },
  "telephone": "+1-555-123-4567",
  "areaServed": "New York"
}</script>"#
        ),
        s if s.contains("saas") || s.contains("software") || s.contains("tech") => (
            "SoftwareApplication + Organization schema",
            r#"Add to your website: <script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "SoftwareApplication",
  "name": "Your App Name",
  "applicationCategory": "BusinessApplication",
  "operatingSystem": "Web, iOS, Android",
  "offers": { "@type": "Offer", "price": "0", "priceCurrency": "USD" },
  "aggregateRating": { "@type": "AggregateRating", "ratingValue": "4.6", "reviewCount": "1200" }
}</script>"#
        ),
        s if s.contains("bank") || s.contains("finance") || s.contains("fintech") => (
            "FinancialService + Organization schema",
            r#"Add to your website: <script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "FinancialService",
  "name": "Your Company Name",
  "address": { "@type": "PostalAddress", "streetAddress": "...", "addressLocality": "...", "postalCode": "..." },
  "telephone": "+1-555-123-4567",
  "areaServed": "United States"
}</script>"#
        ),
        _ => (
            "Organization + LocalBusiness schema",
            r#"Add to your website: <script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Organization",
  "name": "Your Business Name",
  "address": { "@type": "PostalAddress", "streetAddress": "...", "addressLocality": "...", "postalCode": "..." },
  "telephone": "+1-555-123-4567",
  "sameAs": ["https://twitter.com/yourbrand", "https://linkedin.com/company/yourbrand"]
}</script>"#
        ),
    };
    
    Recommendation {
        priority: "important".to_string(),
        text: format!(
            "Add {} to your website. Structured data helps AI models accurately understand and cite your business. \
             This markup makes your business information machine-readable and improves visibility in AI responses.",
            schema_type
        ),
        evidence: format!("Schema.org structured data recommendation for {} industry. Example:\n{}", industry, example),
    }
}

/// Extract competitor brand names from raw Tavily discovery text.
fn extract_competitor_names(raw_texts: &[String], brand: &str) -> Vec<String> {
    let brand_lower = brand.to_lowercase();
    let mut names = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for text in raw_texts {
        for line in text.lines() {
            let line = line.trim();
            // Look for numbered list items: "1. Brand Name" or "- Brand Name"
            let candidate = line
                .trim_start_matches(|c: char| c.is_numeric() || c == '.' || c == '-' || c == '*' || c == ' ');
            let candidate = candidate.trim();
            if candidate.is_empty() || candidate.len() > 60 {
                continue;
            }
            // Skip if it's our brand
            if candidate.to_lowercase().contains(&brand_lower) {
                continue;
            }
            // Accept if it starts with uppercase (likely a brand name)
            if candidate.chars().next().is_some_and(|c| c.is_uppercase()) {
                let name = candidate.split(&[',', '(', '\u{2013}', '\u{2014}', ':'][..]).next().unwrap_or(candidate).trim().to_string();
                if name.len() >= 2 && name.len() <= 50 && seen.insert(name.to_lowercase()) {
                    names.push(name);
                }
            }
        }
    }

    names.into_iter().take(10).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_endpoint() {
        let state = AppState {
            tavily_key: "test".to_string(),
            openai_key: None,
            gemini_key: None,
            anthropic_key: None,
            // perplexity removed
            glm_key: None,
            engram: None,
            jobs: Arc::new(Mutex::new(HashMap::new())),
            reports: Arc::new(Mutex::new(HashMap::new())),
        };

        let app = router(state);
        let resp = axum::serve(
            tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap(),
            app,
        );
        drop(resp);
    }
}
