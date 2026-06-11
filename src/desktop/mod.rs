use anyhow::Result;
use tracing::info;

pub mod widgets;
pub mod rendering;

#[allow(dead_code)]
pub struct DesktopManager {
    is_running: bool,
    widgets: Vec<String>,
}

impl DesktopManager {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            is_running: false,
            widgets: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting desktop manager");
        self.is_running = true;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping desktop manager");
        self.is_running = false;
        Ok(())
    }
}
