use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

pub mod init;
pub mod commands;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub desktop: DesktopConfig,
    pub theme: ThemeConfig,
    pub system: SystemConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopConfig {
    pub enabled: bool,
    pub show_widgets: bool,
    pub widget_opacity: f32,
    pub animation_speed: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub current_theme: String,
    pub theme_directory: PathBuf,
    pub auto_reload: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub integration_mode: String, // "explorer" or "shell"
    pub auto_start: bool,
    pub log_level: String,
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
            .join("losi");
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
            desktop: DesktopConfig::default(),
            theme: ThemeConfig::default(),
            system: SystemConfig::default(),
        }
    }
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            show_widgets: true,
            widget_opacity: 0.9,
            animation_speed: 200,
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        let theme_dir = dirs::config_dir()
            .map(|d| d.join("losi").join("themes"))
            .unwrap_or_else(|| PathBuf::from("./themes"));

        Self {
            current_theme: "default".to_string(),
            theme_directory: theme_dir,
            auto_reload: true,
        }
    }
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            integration_mode: "explorer".to_string(),
            auto_start: false,
            log_level: "info".to_string(),
        }
    }
}
