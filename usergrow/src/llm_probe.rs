use crate::query_gen::Query;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResult {
    pub query: Query,
    pub model: String,
    pub response: String,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatResponse {
    choices: Option<Vec<OpenAIChoice>>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiPart {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
}

async fn probe_openai(client: &Client, query: &Query, api_key: &str) -> ProbeResult {
    let request = OpenAIChatRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![OpenAIMessage {
            role: "user".to_string(),
            content: query.text.clone(),
        }],
        max_tokens: 1024,
    };

    match client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await
    {
        Ok(resp) => match resp.json::<OpenAIChatResponse>().await {
            Ok(data) => {
                let text = data
                    .choices
                    .and_then(|c| c.first().map(|ch| ch.message.content.clone()))
                    .unwrap_or_default();
                ProbeResult {
                    query: query.clone(),
                    model: "gpt-4o-mini".to_string(),
                    response: text,
                    error: None,
                }
            }
            Err(e) => ProbeResult {
                query: query.clone(),
                model: "gpt-4o-mini".to_string(),
                response: String::new(),
                error: Some(format!("Parse error: {}", e)),
            },
        },
        Err(e) => ProbeResult {
            query: query.clone(),
            model: "gpt-4o-mini".to_string(),
            response: String::new(),
            error: Some(format!("Request error: {}", e)),
        },
    }
}

async fn probe_gemini(client: &Client, query: &Query, api_key: &str) -> ProbeResult {
    let request = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![GeminiPart {
                text: query.text.clone(),
            }],
        }],
    };

    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent";

    match client.post(url).header("x-goog-api-key", api_key).json(&request).send().await {
        Ok(resp) => match resp.json::<GeminiResponse>().await {
            Ok(data) => {
                let text = data
                    .candidates
                    .and_then(|c| {
                        c.first()
                            .and_then(|cand| cand.content.parts.first().map(|p| p.text.clone()))
                    })
                    .unwrap_or_default();
                ProbeResult {
                    query: query.clone(),
                    model: "gemini-2.0-flash".to_string(),
                    response: text,
                    error: None,
                }
            }
            Err(e) => ProbeResult {
                query: query.clone(),
                model: "gemini-2.0-flash".to_string(),
                response: String::new(),
                error: Some(format!("Parse error: {}", e)),
            },
        },
        Err(e) => ProbeResult {
            query: query.clone(),
            model: "gemini-2.0-flash".to_string(),
            response: String::new(),
            error: Some(format!("Request error: {}", e)),
        },
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<OpenAIMessage>,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Option<Vec<AnthropicContent>>,
}

#[derive(Debug, Deserialize)]
struct AnthropicContent {
    text: String,
}

async fn probe_glm(client: &Client, query: &Query, api_key: &str) -> ProbeResult {
    // Use z.ai Anthropic-compatible endpoint (maps to GLM-4.5-Air via Haiku)
    let request = AnthropicRequest {
        model: "claude-haiku-3-5-20241022".to_string(),
        max_tokens: 1024,
        messages: vec![OpenAIMessage {
            role: "user".to_string(),
            content: query.text.clone(),
        }],
    };

    match client
        .post("https://api.z.ai/api/anthropic/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&request)
        .send()
        .await
    {
        Ok(resp) => match resp.json::<AnthropicResponse>().await {
            Ok(data) => {
                let text = data
                    .content
                    .and_then(|c| c.first().map(|b| b.text.clone()))
                    .unwrap_or_default();
                ProbeResult {
                    query: query.clone(),
                    model: "GLM-4.5".to_string(),
                    response: text,
                    error: None,
                }
            }
            Err(e) => ProbeResult {
                query: query.clone(),
                model: "GLM-4.5".to_string(),
                response: String::new(),
                error: Some(format!("Parse error: {}", e)),
            },
        },
        Err(e) => ProbeResult {
            query: query.clone(),
            model: "GLM-4.5".to_string(),
            response: String::new(),
            error: Some(format!("Request error: {}", e)),
        },
    }
}

async fn probe_perplexity(client: &Client, query: &Query, api_key: &str) -> ProbeResult {
    let request = OpenAIChatRequest {
        model: "sonar".to_string(),
        messages: vec![OpenAIMessage {
            role: "user".to_string(),
            content: query.text.clone(),
        }],
        max_tokens: 1024,
    };

    match client
        .post("https://api.perplexity.ai/chat/completions")
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await
    {
        Ok(resp) => match resp.json::<OpenAIChatResponse>().await {
            Ok(data) => {
                let text = data
                    .choices
                    .and_then(|c| c.first().map(|ch| ch.message.content.clone()))
                    .unwrap_or_default();
                ProbeResult {
                    query: query.clone(),
                    model: "perplexity-sonar".to_string(),
                    response: text,
                    error: None,
                }
            }
            Err(e) => ProbeResult {
                query: query.clone(),
                model: "perplexity-sonar".to_string(),
                response: String::new(),
                error: Some(format!("Parse error: {}", e)),
            },
        },
        Err(e) => ProbeResult {
            query: query.clone(),
            model: "perplexity-sonar".to_string(),
            response: String::new(),
            error: Some(format!("Request error: {}", e)),
        },
    }
}

async fn probe_anthropic(client: &Client, query: &Query, api_key: &str) -> ProbeResult {
    // Detect z.ai key (contains dot) vs real Anthropic key
    let is_zai = api_key.contains('.');
    let (model_name, base_url, model_label) = if is_zai {
        ("claude-sonnet-4-20250514".to_string(), "https://api.z.ai/api/anthropic/v1/messages", "GLM-4.7")
    } else {
        // Use Haiku (cheapest) for real Anthropic keys
        ("claude-3-haiku-20240307".to_string(), "https://api.anthropic.com/v1/messages", "Claude-Haiku")
    };

    let request = AnthropicRequest {
        model: model_name,
        max_tokens: 1024,
        messages: vec![OpenAIMessage {
            role: "user".to_string(),
            content: query.text.clone(),
        }],
    };

    match client
        .post(base_url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&request)
        .send()
        .await
    {
        Ok(resp) => match resp.json::<AnthropicResponse>().await {
            Ok(data) => {
                let text = data
                    .content
                    .and_then(|c| c.first().map(|b| b.text.clone()))
                    .unwrap_or_default();
                ProbeResult {
                    query: query.clone(),
                    model: model_label.to_string(),
                    response: text,
                    error: None,
                }
            }
            Err(e) => ProbeResult {
                query: query.clone(),
                model: model_label.to_string(),
                response: String::new(),
                error: Some(format!("Parse error: {}", e)),
            },
        },
        Err(e) => ProbeResult {
            query: query.clone(),
            model: model_label.to_string(),
            response: String::new(),
            error: Some(format!("Request error: {}", e)),
        },
    }
}

/// Probe all configured LLMs with the given queries (concurrent execution)
pub async fn probe_all(
    queries: &[Query],
    openai_key: &Option<String>,
    gemini_key: &Option<String>,
    anthropic_key: &Option<String>,
    perplexity_key: &Option<String>,
    glm_key: &Option<String>,
) -> Result<Vec<ProbeResult>> {
    probe_all_with_progress(
        queries,
        openai_key,
        gemini_key,
        anthropic_key,
        perplexity_key,
        glm_key,
        None,
    )
    .await
}

/// Probe all configured LLMs with the given queries and optional progress callback
pub async fn probe_all_with_progress(
    queries: &[Query],
    openai_key: &Option<String>,
    gemini_key: &Option<String>,
    anthropic_key: &Option<String>,
    perplexity_key: &Option<String>,
    glm_key: &Option<String>,
    progress_tx: Option<tokio::sync::mpsc::Sender<crate::types::ProgressEvent>>,
) -> Result<Vec<ProbeResult>> {
    use futures::future::join_all;

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap_or_default();
    let mut all_tasks = Vec::new();

    // Helper to send progress updates
    let send_progress = |tx: &Option<tokio::sync::mpsc::Sender<crate::types::ProgressEvent>>,
                         step: String,
                         status: String,
                         done: Option<u32>,
                         total: Option<u32>| {
        if let Some(sender) = tx {
            let event = crate::types::ProgressEvent {
                step,
                status,
                detail: None,
                done,
                total,
                data: None,
            };
            let _ = sender.try_send(event);
        }
    };

    let total_queries = queries.len() as u32;

    // OpenAI probes
    if let Some(key) = openai_key {
        send_progress(
            &progress_tx,
            "probe_openai".to_string(),
            "progress".to_string(),
            Some(0),
            Some(total_queries),
        );

        for (i, query) in queries.iter().enumerate() {
            let client = client.clone();
            let query = query.clone();
            let key = key.clone();
            let tx = progress_tx.clone();

            let task = tokio::spawn(async move {
                // Small stagger to avoid immediate burst
                tokio::time::sleep(tokio::time::Duration::from_millis(i as u64 * 50)).await;
                let result = probe_openai(&client, &query, &key).await;

                if let Some(sender) = tx {
                    let _ = sender.try_send(crate::types::ProgressEvent {
                        step: "probe_openai".to_string(),
                        status: "progress".to_string(),
                        detail: Some(format!("Completed: {}", query.text)),
                        done: Some((i + 1) as u32),
                        total: None,
                        data: None,
                    });
                }

                result
            });
            all_tasks.push(task);
        }
    }

    // Gemini probes
    if let Some(key) = gemini_key {
        send_progress(
            &progress_tx,
            "probe_gemini".to_string(),
            "progress".to_string(),
            Some(0),
            Some(total_queries),
        );

        for (i, query) in queries.iter().enumerate() {
            let client = client.clone();
            let query = query.clone();
            let key = key.clone();
            let tx = progress_tx.clone();

            let task = tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(i as u64 * 50)).await;
                let result = probe_gemini(&client, &query, &key).await;

                if let Some(sender) = tx {
                    let _ = sender.try_send(crate::types::ProgressEvent {
                        step: "probe_gemini".to_string(),
                        status: "progress".to_string(),
                        detail: Some(format!("Completed: {}", query.text)),
                        done: Some((i + 1) as u32),
                        total: None,
                        data: None,
                    });
                }

                result
            });
            all_tasks.push(task);
        }
    }

    // Anthropic probes
    if let Some(key) = anthropic_key {
        send_progress(
            &progress_tx,
            "probe_anthropic".to_string(),
            "progress".to_string(),
            Some(0),
            Some(total_queries),
        );

        for (i, query) in queries.iter().enumerate() {
            let client = client.clone();
            let query = query.clone();
            let key = key.clone();
            let tx = progress_tx.clone();

            let task = tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(i as u64 * 50)).await;
                let result = probe_anthropic(&client, &query, &key).await;

                if let Some(sender) = tx {
                    let _ = sender.try_send(crate::types::ProgressEvent {
                        step: "probe_anthropic".to_string(),
                        status: "progress".to_string(),
                        detail: Some(format!("Completed: {}", query.text)),
                        done: Some((i + 1) as u32),
                        total: None,
                        data: None,
                    });
                }

                result
            });
            all_tasks.push(task);
        }
    }

    // Perplexity probes
    if let Some(key) = perplexity_key {
        send_progress(
            &progress_tx,
            "probe_perplexity".to_string(),
            "progress".to_string(),
            Some(0),
            Some(total_queries),
        );

        for (i, query) in queries.iter().enumerate() {
            let client = client.clone();
            let query = query.clone();
            let key = key.clone();
            let tx = progress_tx.clone();

            let task = tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(i as u64 * 50)).await;
                let result = probe_perplexity(&client, &query, &key).await;

                if let Some(sender) = tx {
                    let _ = sender.try_send(crate::types::ProgressEvent {
                        step: "probe_perplexity".to_string(),
                        status: "progress".to_string(),
                        detail: Some(format!("Completed: {}", query.text)),
                        done: Some((i + 1) as u32),
                        total: None,
                        data: None,
                    });
                }

                result
            });
            all_tasks.push(task);
        }
    }

    // GLM probes
    if let Some(key) = glm_key {
        send_progress(
            &progress_tx,
            "probe_glm".to_string(),
            "progress".to_string(),
            Some(0),
            Some(total_queries),
        );

        for (i, query) in queries.iter().enumerate() {
            let client = client.clone();
            let query = query.clone();
            let key = key.clone();
            let tx = progress_tx.clone();

            let task = tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(i as u64 * 50)).await;
                let result = probe_glm(&client, &query, &key).await;

                if let Some(sender) = tx {
                    let _ = sender.try_send(crate::types::ProgressEvent {
                        step: "probe_glm".to_string(),
                        status: "progress".to_string(),
                        detail: Some(format!("Completed: {}", query.text)),
                        done: Some((i + 1) as u32),
                        total: None,
                        data: None,
                    });
                }

                result
            });
            all_tasks.push(task);
        }
    }

    // Wait for all tasks to complete
    let results: Vec<ProbeResult> = join_all(all_tasks)
        .await
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();

    // Send completion events
    if openai_key.is_some() {
        send_progress(
            &progress_tx,
            "probe_openai".to_string(),
            "complete".to_string(),
            None,
            None,
        );
    }
    if gemini_key.is_some() {
        send_progress(
            &progress_tx,
            "probe_gemini".to_string(),
            "complete".to_string(),
            None,
            None,
        );
    }
    if anthropic_key.is_some() {
        send_progress(
            &progress_tx,
            "probe_anthropic".to_string(),
            "complete".to_string(),
            None,
            None,
        );
    }
    if perplexity_key.is_some() {
        send_progress(
            &progress_tx,
            "probe_perplexity".to_string(),
            "complete".to_string(),
            None,
            None,
        );
    }
    if glm_key.is_some() {
        send_progress(
            &progress_tx,
            "probe_glm".to_string(),
            "complete".to_string(),
            None,
            None,
        );
    }

    Ok(results)
}
