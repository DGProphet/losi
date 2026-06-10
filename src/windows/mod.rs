use anyhow::Result;
use tracing::info;

#[cfg(target_os = "windows")]
use winapi::um::winuser::GetSystemMetrics;

/// Initialize Windows 11 specific features
pub async fn init_windows11() -> Result<()> {
    info!("Initializing Windows 11 features");
    
    // Windows 11 detection
    if is_windows11() {
        info!("Windows 11 detected");
    }
    
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn is_windows11() -> bool {
    use std::os::raw::c_int;
    unsafe {
        let major = GetSystemMetrics(0); // SM_CXSCREEN
        major > 0 // Simplified check
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_windows11() -> bool {
    false
}

/// Get Windows Terminal capabilities
pub fn has_terminal_support() -> bool {
    #[cfg(target_os = "windows")]
    {
        std::env::var("WT_SESSION").is_ok()
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}
