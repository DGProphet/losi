use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::info;

pub mod init;
pub mod theme;
pub mod commands;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: ThemeConfig,
    pub shell: ShellConfig,
    pub wsl2: Wsl2Config,
    pub history: HistoryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub colors: ColorScheme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub foreground: String,
    pub background: String,
    pub accent: String,
    pub error: String,
    pub success: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    pub prompt: String,
    pub editor: String,
    pub shell_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wsl2Config {
    pub enabled: bool,
    pub default_distro: Option<String>,
    pub path_mapping: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryConfig {
    pub enabled: bool,
    pub max_entries: usize,
    pub location: PathBuf,
}

impl Config {
    pub async fn load(path: Option<&str>) -> Result<Self> {
        let config_path = if let Some(p) = path {
            PathBuf::from(p)
        } else {
            Self::default_path()?
        };

        if config_path.exists() {
            info!("Loading config from {:?}", config_path);
            let content = tokio::fs::read_to_string(&config_path).await?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            info!("Config not found, using defaults");
            Ok(Self::default())
        }
    }

    pub fn default_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
            .join("hshell");
        Ok(config_dir.join("config.toml"))
    }

    pub async fn save(&self, path: Option<&str>) -> Result<()> {
        let config_path = if let Some(p) = path {
            PathBuf::from(p)
        } else {
            Self::default_path()?
        };

        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let content = toml::to_string_pretty(self)?;
        tokio::fs::write(&config_path, content).await?;
        info!("Config saved to {:?}", config_path);
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
            shell: ShellConfig::default(),
            wsl2: Wsl2Config::default(),
            history: HistoryConfig::default(),
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            colors: ColorScheme::default(),
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            foreground: "#E1E1E1".to_string(),
            background: "#0C0C0C".to_string(),
            accent: "#007ACC".to_string(),
            error: "#F48771".to_string(),
            success: "#4EC9B0".to_string(),
        }
    }
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            prompt: "hshell> ".to_string(),
            editor: "code".to_string(),
            shell_type: "powershell".to_string(),
        }
    }
}

impl Default for Wsl2Config {
    fn default() -> Self {
        Self {
            enabled: false,
            default_distro: None,
            path_mapping: true,
        }
    }
}

impl Default for HistoryConfig {
    fn default() -> Self {
        let history_path = dirs::data_dir()
            .map(|d| d.join("hshell").join("history.json"))
            .unwrap_or_else(|| PathBuf::from("./history.json"));

        Self {
            enabled: true,
            max_entries: 10000,
            location: history_path,
        }
    }
}
