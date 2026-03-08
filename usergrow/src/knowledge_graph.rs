//! Knowledge Graph module — structured entity-relation storage.
//!
//! Tables: kg_entities, kg_relations, kg_snapshots
//! All stored in the same SQLite DB as Engram memories.

use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub struct KnowledgeGraph {
    conn: Mutex<Connection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KgEntity {
    pub id: String,
    pub entity_type: String,
    pub name: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KgRelation {
    pub source_id: String,
    pub target_id: String,
    pub relation_type: String,
    pub weight: f64,
    pub evidence: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KgSnapshot {
    pub entity_id: String,
    pub timestamp: String,
    pub metrics: serde_json::Value,
}

impl KnowledgeGraph {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        // Ensure tables exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS kg_entities (
                id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                name TEXT NOT NULL,
                metadata TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS kg_relations (
                source_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                relation_type TEXT NOT NULL,
                weight REAL NOT NULL DEFAULT 0.5,
                evidence TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (source_id, target_id, relation_type)
            );
            CREATE TABLE IF NOT EXISTS kg_snapshots (
                entity_id TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                metrics TEXT NOT NULL,
                PRIMARY KEY (entity_id, timestamp)
            );",
        )?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Upsert an entity (insert or update metadata).
    pub fn upsert_entity(
        &self,
        id: &str,
        entity_type: &str,
        name: &str,
        metadata: Option<&serde_json::Value>,
    ) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("{}", e))?;
        let now = chrono::Utc::now().to_rfc3339();
        let meta_str = metadata.map(|m| serde_json::to_string(m).unwrap_or_default());
        conn.execute(
            "INSERT INTO kg_entities (id, entity_type, name, metadata, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)
             ON CONFLICT(id) DO UPDATE SET
                metadata = COALESCE(?4, metadata),
                updated_at = ?5",
            rusqlite::params![id, entity_type, name, meta_str, now],
        )?;
        Ok(())
    }

    /// Upsert a relation (insert or strengthen weight).
    pub fn upsert_relation(
        &self,
        source_id: &str,
        target_id: &str,
        relation_type: &str,
        weight: f64,
        evidence: Option<&serde_json::Value>,
    ) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("{}", e))?;
        let now = chrono::Utc::now().to_rfc3339();
        let ev_str = evidence.map(|e| serde_json::to_string(e).unwrap_or_default());
        conn.execute(
            "INSERT INTO kg_relations (source_id, target_id, relation_type, weight, evidence, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)
             ON CONFLICT(source_id, target_id, relation_type) DO UPDATE SET
                weight = ?4,
                evidence = COALESCE(?5, evidence),
                updated_at = ?6",
            rusqlite::params![source_id, target_id, relation_type, weight, ev_str, now],
        )?;
        Ok(())
    }

    /// Store a metrics snapshot for an entity at current time.
    pub fn store_snapshot(&self, entity_id: &str, metrics: &serde_json::Value) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("{}", e))?;
        let now = chrono::Utc::now().to_rfc3339();
        let metrics_str = serde_json::to_string(metrics)?;
        conn.execute(
            "INSERT OR REPLACE INTO kg_snapshots (entity_id, timestamp, metrics) VALUES (?1, ?2, ?3)",
            rusqlite::params![entity_id, now, metrics_str],
        )?;
        Ok(())
    }

    /// Ingest a complete analysis report into the KG.
    /// Creates entities for brand, industry, city, personas, keywords, models
    /// and relations between them.
    pub fn ingest_report(&self, report: &crate::types::FullReport) -> Result<()> {
        let brand_id = slugify(&format!("brand:{}", report.brand));
        let industry_id = slugify(&format!("industry:{}", report.industry));
        let city_id = slugify(&format!("city:{}", report.city));

        // 1. Upsert core entities (brand includes visibility score in metadata)
        let brand_meta = serde_json::json!({
            "visibility_score": report.visibility_score,
            "chatgpt_pct": report.share_of_model.chatgpt,
            "claude_pct": report.share_of_model.claude,
            "gemini_pct": report.share_of_model.gemini,
        });
        self.upsert_entity(&brand_id, "brand", &report.brand, Some(&brand_meta))?;
        self.upsert_entity(&industry_id, "industry", &report.industry, None)?;
        if !report.city.is_empty() {
            self.upsert_entity(&city_id, "city", &report.city, None)?;
            self.upsert_relation(&brand_id, &city_id, "located_in", 1.0, None)?;
        }
        self.upsert_relation(&brand_id, &industry_id, "belongs_to", 1.0, None)?;

        // 2. Model visibility relations
        let som = &report.share_of_model;
        let model_scores = [
            ("model:chatgpt", "ChatGPT", som.chatgpt),
            ("model:claude", "Claude", som.claude),
            ("model:gemini", "Gemini", som.gemini),
            ("model:glm47", "GLM-4.7", som.glm_47),
            ("model:glm45", "GLM-4.5", som.glm_45),
        ];
        for (mid, mname, score) in &model_scores {
            self.upsert_entity(mid, "model", mname, None)?;
            if *score > 0.0 {
                let ev = serde_json::json!({"share_pct": score});
                self.upsert_relation(&brand_id, mid, "visible_in", score / 100.0, Some(&ev))?;
            }
        }

        // 3. Persona visibility relations
        for entry in &report.persona_heatmap {
            let persona_id = slugify(&format!("persona:{}", entry.persona_name));
            let meta = serde_json::json!({"description": entry.persona_description});
            self.upsert_entity(&persona_id, "persona", &entry.persona_name, Some(&meta))?;

            // Aggregate: how many models mentioned brand for this persona
            let total = entry.results.len() as f64;
            let mentioned = entry
                .results
                .values()
                .filter(|c| c.mentioned)
                .count() as f64;
            let vis_weight = if total > 0.0 { mentioned / total } else { 0.0 };

            let ev = serde_json::json!({
                "models_mentioned": mentioned as u32,
                "models_total": total as u32,
            });
            self.upsert_relation(&brand_id, &persona_id, "visible_to", vis_weight, Some(&ev))?;
        }

        // 4. Brand DNA keywords
        for kw in &report.brand_dna.your_brand {
            let kw_id = slugify(&format!("keyword:{}", kw.keyword));
            let meta = serde_json::json!({"sentiment": kw.sentiment});
            self.upsert_entity(&kw_id, "keyword", &kw.keyword, Some(&meta))?;
            self.upsert_relation(
                &brand_id,
                &kw_id,
                "associated_with",
                kw.score,
                None,
            )?;
        }

        // 5. Competitor relations
        for (comp_name, _kws) in &report.brand_dna.competitors {
            let comp_id = slugify(&format!("brand:{}", comp_name));
            self.upsert_entity(&comp_id, "brand", comp_name, None)?;
            self.upsert_relation(&brand_id, &comp_id, "competes_with", 0.5, None)?;
            self.upsert_relation(&comp_id, &industry_id, "belongs_to", 1.0, None)?;
        }

        // 6. Snapshot with current metrics
        let metrics = serde_json::json!({
            "visibility_score": report.visibility_score,
            "chatgpt_pct": som.chatgpt,
            "claude_pct": som.claude,
            "gemini_pct": som.gemini,
            "glm47_pct": som.glm_47,
            "glm45_pct": som.glm_45,
        });
        self.store_snapshot(&brand_id, &metrics)?;

        Ok(())
    }

    // ── Query helpers ──────────────────────────────────────────────────

    /// Get all entities of a given type.
    pub fn entities_by_type(&self, entity_type: &str) -> Result<Vec<KgEntity>> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("{}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, entity_type, name, metadata FROM kg_entities WHERE entity_type = ?1",
        )?;
        let rows = stmt.query_map([entity_type], |row| {
            let meta_str: Option<String> = row.get(3)?;
            Ok(KgEntity {
                id: row.get(0)?,
                entity_type: row.get(1)?,
                name: row.get(2)?,
                metadata: meta_str.and_then(|s| serde_json::from_str(&s).ok()),
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Get all relations from/to an entity.
    pub fn relations_for(&self, entity_id: &str) -> Result<Vec<KgRelation>> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("{}", e))?;
        let mut stmt = conn.prepare(
            "SELECT source_id, target_id, relation_type, weight, evidence
             FROM kg_relations WHERE source_id = ?1 OR target_id = ?1",
        )?;
        let rows = stmt.query_map([entity_id], |row| {
            let ev_str: Option<String> = row.get(4)?;
            Ok(KgRelation {
                source_id: row.get(0)?,
                target_id: row.get(1)?,
                relation_type: row.get(2)?,
                weight: row.get(3)?,
                evidence: ev_str.and_then(|s| serde_json::from_str(&s).ok()),
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Get snapshots for an entity (time series).
    pub fn snapshots_for(&self, entity_id: &str) -> Result<Vec<KgSnapshot>> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("{}", e))?;
        let mut stmt = conn.prepare(
            "SELECT entity_id, timestamp, metrics FROM kg_snapshots
             WHERE entity_id = ?1 ORDER BY timestamp",
        )?;
        let rows = stmt.query_map([entity_id], |row| {
            let m: String = row.get(2)?;
            Ok(KgSnapshot {
                entity_id: row.get(0)?,
                timestamp: row.get(1)?,
                metrics: serde_json::from_str(&m).unwrap_or_default(),
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Get full graph data for visualization (entities + relations).
    pub fn full_graph(&self) -> Result<(Vec<KgEntity>, Vec<KgRelation>)> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("{}", e))?;

        let mut stmt = conn.prepare("SELECT id, entity_type, name, metadata FROM kg_entities")?;
        let entities: Vec<KgEntity> = stmt
            .query_map([], |row| {
                let meta_str: Option<String> = row.get(3)?;
                Ok(KgEntity {
                    id: row.get(0)?,
                    entity_type: row.get(1)?,
                    name: row.get(2)?,
                    metadata: meta_str.and_then(|s| serde_json::from_str(&s).ok()),
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        let mut stmt = conn.prepare(
            "SELECT source_id, target_id, relation_type, weight, evidence FROM kg_relations",
        )?;
        let relations: Vec<KgRelation> = stmt
            .query_map([], |row| {
                let ev_str: Option<String> = row.get(4)?;
                Ok(KgRelation {
                    source_id: row.get(0)?,
                    target_id: row.get(1)?,
                    relation_type: row.get(2)?,
                    weight: row.get(3)?,
                    evidence: ev_str.and_then(|s| serde_json::from_str(&s).ok()),
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok((entities, relations))
    }
}

/// Simple slug generator for entity IDs.
fn slugify(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == ':' || c == '-' { c } else { '-' })
        .collect::<String>()
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("brand:Dr. Mehmet Oz"), "brand:dr--mehmet-oz");
        assert_eq!(slugify("industry:healthcare"), "industry:healthcare");
        assert_eq!(slugify("keyword:trusted provider"), "keyword:trusted-provider");
    }
}
