mod analysis;
mod brand_dna;
mod engram;
mod knowledge_graph;
mod llm_probe;
mod persona;
mod query_gen;
mod reality;
mod report;
mod tavily;
mod types;
mod web;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "geo-agent",
    about = "GEO (Generative Engine Optimization) Agent"
)]
struct Args {
    /// Brand name to analyze (required for CLI mode, optional for server)
    #[arg(short, long, default_value = "")]
    brand: String,

    /// Industry / vertical
    #[arg(short, long, default_value = "finance")]
    industry: String,

    /// Competitors (comma-separated)
    #[arg(short, long)]
    competitors: Option<String>,

    /// Tavily API key
    #[arg(long, env = "TAVILY_API_KEY")]
    tavily_key: Option<String>,

    /// OpenAI API key
    #[arg(long, env = "OPENAI_API_KEY")]
    openai_key: Option<String>,

    /// Google Gemini API key
    #[arg(long, env = "GEMINI_API_KEY")]
    gemini_key: Option<String>,

    /// Anthropic API key
    #[arg(long, env = "ANTHROPIC_API_KEY")]
    anthropic_key: Option<String>,

    /// Perplexity API key
    #[arg(long, env = "PERPLEXITY_API_KEY")]
    perplexity_key: Option<String>,

    /// GLM (Zhipu AI) API key
    #[arg(long, env = "GLM_API_KEY")]
    glm_key: Option<String>,

    /// Run as web server instead of CLI
    #[arg(long, default_value_t = false)]
    serve: bool,

    /// Port for web server
    #[arg(long, default_value_t = 3000)]
    port: u16,

    /// Engram memory database path
    #[arg(long, env = "ENGRAM_DB", default_value = "./geo_agent_memory.db")]
    engram_db: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize Engram
    let engram = match engram::EngramClient::new(&args.engram_db) {
        Ok(client) => {
            println!("🧠 Engram memory initialized: {}", args.engram_db);
            Some(std::sync::Arc::new(client))
        }
        Err(e) => {
            eprintln!("⚠️  Engram init failed (continuing without): {}", e);
            None
        }
    };

    // Initialize Knowledge Graph
    let kg = match knowledge_graph::KnowledgeGraph::new(&args.engram_db) {
        Ok(g) => {
            println!("🕸️  Knowledge Graph initialized");
            Some(std::sync::Arc::new(g))
        }
        Err(e) => {
            eprintln!("⚠️  KG init failed (continuing without): {}", e);
            None
        }
    };

    // Web server mode
    if args.serve {
        println!(
            "🌐 UserGrow — Starting web server on port {}...",
            args.port
        );
        let state = web::AppState {
            tavily_key: args.tavily_key.clone().unwrap_or_default(),
            openai_key: args.openai_key.clone(),
            gemini_key: args.gemini_key.clone(),
            anthropic_key: args.anthropic_key.clone(),
            // perplexity removed
            glm_key: args.glm_key.clone(),
            engram,
            kg,
            jobs: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
            reports: std::sync::Arc::new(tokio::sync::Mutex::new(
                std::collections::HashMap::new(),
            )),
        };
        let app = web::router(state);
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port)).await?;
        println!("✅ Listening on http://localhost:{}", args.port);
        axum::serve(listener, app).await?;
        return Ok(());
    }

    println!("🎯 UserGrow - Generative Engine Optimization");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Brand: {}", args.brand);
    println!("Industry: {}", args.industry);
    if let Some(ref comp) = args.competitors {
        println!("Competitors: {}", comp);
    }
    println!();

    let competitors: Vec<String> = args
        .competitors
        .map(|c| c.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    // Step 0: Tavily-powered competitor discovery (if key provided)
    if let Some(ref key) = args.tavily_key {
        println!("📡 Step 0: Discovering competitor landscape via Tavily...");
        let tavily = tavily::TavilyClient::new(key.clone());

        match tavily
            .discover_competitors(&args.brand, &args.industry, "")
            .await
        {
            Ok(context) => {
                println!("   Got {} context snippets from web", context.len());
            }
            Err(e) => eprintln!("   ⚠ Tavily discovery failed (continuing without): {}", e),
        }

        match tavily.research_queries(&args.brand, &args.industry).await {
            Ok(snippets) => {
                println!("   Got {} query research snippets\n", snippets.len());
            }
            Err(e) => eprintln!("   ⚠ Tavily research failed (continuing without): {}\n", e),
        }
    }

    // Step 1: Generate customer queries
    println!("📝 Step 1: Generating customer queries...");
    let queries =
        query_gen::generate_queries(&args.brand, &args.industry, &competitors, None).await?;
    println!("   Generated {} queries\n", queries.len());

    // Step 2: Probe LLMs
    println!("🔍 Step 2: Probing LLMs with queries...");
    let probe_results = llm_probe::probe_all(
        &queries,
        &args.openai_key,
        &args.gemini_key,
        &args.anthropic_key,
        &None, // perplexity removed
        &args.glm_key,
    )
    .await?;
    println!("   Collected {} responses\n", probe_results.len());

    // Step 3: Analyze results
    println!("📊 Step 3: Analyzing brand presence...");
    let insights = analysis::analyze(&args.brand, &competitors, &probe_results)?;
    println!("   Analysis complete\n");

    // Step 4: Generate report
    println!("📋 Step 4: Generating report...");
    let report_path = report::generate_html(&args.brand, &args.industry, &insights)?;
    println!("   Report saved to: {}\n", report_path);

    println!("✅ Done! Open the report to see your GEO analysis.");

    Ok(())
}
