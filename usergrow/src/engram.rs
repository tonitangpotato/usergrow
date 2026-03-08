//! Engram memory integration using ironclaw-engram (native Rust).

use std::sync::Mutex;

use anyhow::Result;
use ironclaw_engram::{Memory, MemoryType};
use serde::{Deserialize, Serialize};

use crate::types::{DriftInfo, EngramStatus};

pub struct EngramClient {
    memory: Mutex<Memory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanMemory {
    pub id: String,
    pub content: String,
    pub confidence: f64,
    pub confidence_label: String,
    pub activation: f64,
    pub age_days: f64,
}

impl EngramClient {
    pub fn new(db_path: &str) -> Result<Self> {
        let memory = Memory::new(db_path, None)
            .map_err(|e| anyhow::anyhow!("Failed to init engram: {}", e))?;
        Ok(Self {
            memory: Mutex::new(memory),
        })
    }

    /// Store a completed scan result as an engram memory.
    pub fn store_scan(
        &self,
        brand: &str,
        industry: &str,
        city: &str,
        visibility_score: u32,
        report_summary: &str,
    ) -> Result<()> {
        let content = format!(
            "Brand scan for {} ({}, {}): visibility={}/100. {}",
            brand, industry, city, visibility_score, report_summary
        );
        let metadata = serde_json::json!({
            "brand": brand,
            "industry": industry,
            "city": city,
            "visibility_score": visibility_score,
        });
        let mut mem = self
            .memory
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        mem.add(
            &content,
            MemoryType::Factual,
            Some(0.7),
            Some("geo-agent"),
            Some(metadata),
        )
        .map_err(|e| anyhow::anyhow!("Engram add error: {}", e))?;
        Ok(())
    }

    /// Recall previous scans for a brand.
    pub fn recall_history(&self, brand: &str) -> Result<Vec<ScanMemory>> {
        let mut mem = self
            .memory
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let results = mem
            .recall(
                &format!("{} brand scan visibility", brand),
                10,
                None,
                None,
            )
            .map_err(|e| anyhow::anyhow!("Recall error: {}", e))?;
        Ok(results
            .into_iter()
            .map(|r| {
                let age = r.record.age_days();
                ScanMemory {
                    id: r.record.id,
                    content: r.record.content,
                    confidence: r.confidence,
                    confidence_label: r.confidence_label,
                    activation: r.activation,
                    age_days: age,
                }
            })
            .collect())
    }

    /// Detect drift by comparing current score with historical.
    pub fn detect_drift(&self, brand: &str, current_score: u32) -> Result<Option<DriftInfo>> {
        let history = self.recall_history(brand)?;
        for mem in &history {
            if let Some(past_score) = extract_score(&mem.content) {
                let delta = current_score as f64 - past_score as f64;
                let direction = if delta.abs() < 5.0 {
                    "stable"
                } else if delta > 0.0 {
                    "up"
                } else {
                    "down"
                };
                return Ok(Some(DriftInfo {
                    direction: direction.to_string(),
                    delta,
                }));
            }
        }
        Ok(None)
    }

    /// Get total number of scans stored.
    pub fn total_scans(&self) -> Result<u32> {
        let mem = self
            .memory
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let stats = mem
            .stats()
            .map_err(|e| anyhow::anyhow!("Stats error: {}", e))?;
        Ok(stats.total_memories as u32)
    }

    /// Run consolidation (periodic maintenance).
    #[allow(dead_code)]
    pub fn consolidate(&self) -> Result<()> {
        let mut mem = self
            .memory
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        mem.consolidate(1.0)
            .map_err(|e| anyhow::anyhow!("Consolidate error: {}", e))?;
        Ok(())
    }

    /// Build EngramStatus for the report.
    pub fn build_status(&self, brand: &str, current_score: u32) -> EngramStatus {
        let total = self.total_scans().unwrap_or(0);
        let drift = self.detect_drift(brand, current_score).unwrap_or(None);
        let confidence = if total > 5 {
            "high"
        } else if total > 1 {
            "medium"
        } else {
            "initial"
        };
        EngramStatus {
            total_scans: total,
            drift,
            confidence: confidence.to_string(),
        }
    }
}

/// Extract visibility score from a memory content string.
/// Pattern: "visibility=XX/100"
fn extract_score(content: &str) -> Option<u32> {
    let marker = "visibility=";
    let idx = content.find(marker)?;
    let after = &content[idx + marker.len()..];
    let slash = after.find('/')?;
    after[..slash].parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_score() {
        assert_eq!(
            extract_score("Brand scan for X (finance, NYC): visibility=45/100. stuff"),
            Some(45)
        );
        assert_eq!(extract_score("no score here"), None);
        assert_eq!(extract_score("visibility=0/100"), Some(0));
        assert_eq!(extract_score("visibility=100/100"), Some(100));
    }
}
