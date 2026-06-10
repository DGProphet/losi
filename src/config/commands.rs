use anyhow::Result;
use crate::config::Config;

pub async fn show_config(show_path: bool) -> Result<()> {
    let config_path = Config::default_path()?;
    
    if show_path {
        println!("{}", config_path.display());
    } else {
        let config = Config::load(None).await?;
        println!("Current configuration:");
        println!("{:#?}", config);
    }
    
    Ok(())
}
