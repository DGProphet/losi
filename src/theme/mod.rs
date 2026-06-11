use crate::config::Config;
use anyhow::Result;
use std::path::Path;
use tracing::info;

pub mod parser;

pub async fn list_themes() -> Result<()> {
    println!("Theme Management:");
    println!("\nTo use a theme, place a .rc file in your themes directory");
    println!("and run: losi theme apply /path/to/theme.rc");
    println!("\nExample .rc format:");
    println!("  [Desktop]");
    println!("  Background=C:\\Images\\bg.png");
    println!("  Color=0xFF00FF");
    println!("\nSupported properties:");
    println!("  Background      - Desktop background image path");
    println!("  Color           - Desktop background color (hex)");
    println!("  TransparencyKey - Transparent color (hex)");
    Ok(())
}

pub async fn apply_theme(config: &Config, theme_path: &str) -> Result<()> {
    let path = Path::new(theme_path);
    
    if !path.exists() {
        anyhow::bail!("Theme file not found: {}", theme_path);
    }

    info!("Loading theme from: {:?}", path);
    
    let content = tokio::fs::read_to_string(path).await?;
    let theme = parser::parse_rc(&content)?;
    
    println!("✓ Theme loaded successfully");
    println!("\nTheme properties:");
    for (key, value) in &theme {
        println!("  {}: {}", key, value);
    }
    
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
    // TODO: Implement hot reload
    Ok(())
}
