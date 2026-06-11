use crate::config::Config;
use anyhow::Result;

pub async fn initialize_config(force: bool) -> Result<()> {
    let config_path = Config::default_path()?;

    if config_path.exists() && !force {
        println!("Configuration already exists at {:?}", config_path);
        println!("Use --force to overwrite");
        return Ok(());
    }

    if let Some(parent) = config_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let config = Config::default();
    config.save(None).await?;

    println!("✓ Configuration initialized at {:?}", config_path);
    println!();
    println!("Next steps:");
    println!("  1. Edit the config: hshell config --path");
    println!("  2. List available themes: hshell theme list");
    println!("  3. Start the shell: hshell");

    Ok(())
}
