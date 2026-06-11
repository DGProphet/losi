use anyhow::Result;
use std::collections::HashMap;

/// Parse LiteStep .rc format
/// Simple INI-like format:
/// [Section]
/// Key=Value
/// Lines starting with ; are comments
pub fn parse_rc(content: &str) -> Result<HashMap<String, String>> {
    let mut properties = HashMap::new();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        
        // Skip comments and empty lines
        if line.starts_with(';') || line.is_empty() {
            continue;
        }

        // Section header [Section]
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        // Key=Value pair
        if let Some((key, value)) = line.split_once('=') {
            let full_key = if current_section.is_empty() {
                key.trim().to_string()
            } else {
                format!("{}.{}", current_section, key.trim())
            };
            properties.insert(full_key, value.trim().to_string());
        }
    }

    Ok(properties)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rc_basic() {
        let content = r#"
        [Desktop]
        Background=C:\Images\bg.png
        Color=0xFF00FF
        
        [Taskbar]
        Height=48
        "#;
        
        let result = parse_rc(content).unwrap();
        assert_eq!(result.get("Desktop.Background"), Some(&"C:\\Images\\bg.png".to_string()));
        assert_eq!(result.get("Desktop.Color"), Some(&"0xFF00FF".to_string()));
        assert_eq!(result.get("Taskbar.Height"), Some(&"48".to_string()));
    }

    #[test]
    fn test_parse_rc_comments() {
        let content = r#"
        ; This is a comment
        [Desktop]
        ; Another comment
        Background=image.png
        "#;
        
        let result = parse_rc(content).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("Desktop.Background"), Some(&"image.png".to_string()));
    }
}
