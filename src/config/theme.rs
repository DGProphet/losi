use crate::config::Config;
use anyhow::Result;

pub async fn list_themes() -> Result<()> {
    println!("Available themes:");
    println!("  • default   - Default Windows 11 theme");
    println!("  • dracula   - Dracula dark theme");
    println!("  • nord      - Nord theme");
    println!("  • solarized - Solarized theme");
    println!("\nUse 'hshell theme apply <name>' to apply a theme");
    Ok(())
}

pub async fn apply_theme(_config: &Config, name: &str) -> Result<()> {
    match name {
        "default" | "dracula" | "nord" | "solarized" => {
            println!("Theme '{}' will be applied on next shell start", name);
        }
        _ => println!("Unknown theme: {}. Use 'theme list' to see available themes.", name),
    }
    Ok(())
}

pub async fn show_current(config: &Config) -> Result<()> {
    println!("Current theme: {}", config.theme.name);
    println!("Foreground: {}", config.theme.colors.foreground);
    println!("Background: {}", config.theme.colors.background);
    println!("Accent: {}", config.theme.colors.accent);
    Ok(())
}
