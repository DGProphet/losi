use crate::config::Config;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub command: String,
    pub timestamp: String,
    pub exit_code: i32,
    pub duration_ms: u64,
}

pub struct History {
    entries: Vec<HistoryEntry>,
    config: Config,
}

impl History {
    pub async fn new(config: Config) -> Result<Self> {
        let entries = Self::load_entries(&config).await.unwrap_or_default();
        Ok(Self { entries, config })
    }

    pub async fn add(&mut self, entry: HistoryEntry) -> Result<()> {
        self.entries.push(entry);
        
        if self.entries.len() > self.config.history.max_entries {
            self.entries.remove(0);
        }
        
        self.save().await
    }

    pub fn get_all(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn search(&self, pattern: &str) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.command.contains(pattern))
            .collect()
    }

    async fn load_entries(config: &Config) -> Result<Vec<HistoryEntry>> {
        let path = &config.history.location;
        if path.exists() {
            let content = tokio::fs::read_to_string(path).await?;
            let entries = serde_json::from_str(&content)?;
            Ok(entries)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn save(&self) -> Result<()> {
        let path = &self.config.history.location;
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let content = serde_json::to_string_pretty(&self.entries)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }
}
