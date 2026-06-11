use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;
use tracing::info;

use crate::theme::Theme;

/// Parse LSZ (LiteStep ZIP) theme archive
/// LSZ files are ZIP archives containing:
/// - theme.rc (main theme file)
/// - resources/ (images, icons, etc.)
/// - readme.txt (optional)
pub async fn parse_lsz(path: &Path) -> Result<Theme> {
    info!("Parsing LSZ archive: {:?}", path);
    
    // Open the ZIP file
    let file = tokio::fs::File::open(path).await?;
    let reader = std::io::BufReader::new(std::fs::File::open(path)?);
    
    let mut archive = zip::ZipArchive::new(reader)?;
    let mut properties = HashMap::new();
    
    // Look for theme.rc or any .rc file in the archive
    let mut theme_content: Option<String> = None;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_lowercase();
        
        if file_name.ends_with("theme.rc") || file_name.ends_with(".rc") {
            let mut content = String::new();
            std::io::Read::read_to_string(&mut file, &mut content)?;
            theme_content = Some(content);
            break;
        }
    }
    
    if let Some(content) = theme_content {
        properties = super::parser::parse_rc(&content)?;
    } else {
        anyhow::bail!("No theme.rc found in LSZ archive");
    }
    
    let theme_name = path.file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    Ok(Theme {
        name: theme_name,
        properties,
        path: path.display().to_string(),
    })
}
