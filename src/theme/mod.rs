use crate::config::Config;
use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;
use tracing::info;

pub mod parser;
pub mod lsz;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub properties: HashMap<String, String>,
    pub path: String,
}

impl Theme {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|v| v.as_str())
    }

    pub fn get_desktop_background(&self) -> Option<&str> {
        self.get("Desktop.Background")
    }

    pub fn get_desktop_color(&self) -> Option<&str> {
        self.get("Desktop.Color")
    }
}

pub async fn list_themes() -> Result<()> {
    println!("Theme Management:");
    println!("\nSupported formats:");
    println!("  .rc  - LiteStep theme file (INI format)");
    println!("  .lsz - Compressed LiteStep theme archive (ZIP)");
    println!("\nTo use a theme:");
    println!("  1. Place theme file in: losi/Personal/themes/");
    println!("  2. Apply: losi theme apply mytheme.rc");
    println!("  3. Or: losi theme apply mytheme.lsz");
    println!("\nExample .rc format:");
    println!("  [Desktop]");
    println!("  Background=C:\\Images\\bg.png");
    println!("  Color=0xFF00FF");
    println!("  TransparencyKey=0x000000");
    println!("\n  [Taskbar]");
    println!("  Height=48");
    println!("  Opacity=0.95");
    Ok(())
}

pub async fn apply_theme(config: &Config, theme_path: &str) -> Result<()> {
    let path = Path::new(theme_path);
    
    // Check if path is absolute or relative to themes directory
    let full_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        // Try themes directory first
        let mut candidate = config.theme.theme_directory.join(theme_path);
        if !candidate.exists() {
            // Try current directory
            candidate = Path::new(theme_path).to_path_buf();
        }
        candidate
    };

    if !full_path.exists() {
        anyhow::bail!("Theme file not found: {:?}\n\nSearched in:\n  - {:?}\n  - {}", 
            theme_path,
            config.theme.theme_directory,
            std::env::current_dir().unwrap_or_default().display()
        );
    }

    info!("Loading theme from: {:?}", full_path);
    
    // Determine file type by extension
    let extension = full_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    let theme = match extension.as_str() {
        "rc" => {
            let content = tokio::fs::read_to_string(&full_path).await?;
            let properties = parser::parse_rc(&content)?;
            Theme {
                name: full_path.file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
                properties,
                path: full_path.display().to_string(),
            }
        }
        "lsz" => {
            // Parse LSZ (compressed ZIP archive)
            let theme = lsz::parse_lsz(&full_path).await?;
            theme
        }
        _ => anyhow::bail!("Unsupported theme format: .{}", extension),
    };

    println!("\n✓ Theme loaded successfully: {}", theme.name);
    println!("  Path: {}", theme.path);
    println!("\nTheme properties:");
    for (key, value) in &theme.properties {
        println!("  {}: {}", key, value);
    }
    
    println!("\nTo apply this theme, run: losi start");
    
    Ok(())
}

pub async fn show_current(config: &Config) -> Result<()> {
    println!("Current theme: {}", config.theme.current_theme);
    println!("Theme directory: {:?}", config.theme.theme_directory);
    println!("Auto-reload: {}", config.theme.auto_reload);
    Ok(())
}

pub async fn reload_theme(config: &Config) -> Result<()> {
    println!("Reloading theme: {}", config.theme.current_theme);
    // TODO: Implement hot reload by watching file
    Ok(())
}
