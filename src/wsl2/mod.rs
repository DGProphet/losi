use anyhow::Result;
use std::process::Command;
use tracing::info;

/// Check if WSL2 is installed and enabled
pub async fn check_status() -> Result<()> {
    info!("Checking WSL2 status");
    
    match Command::new("wsl")
        .args(&["--list", "--verbose"])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                println!("✓ WSL2 is installed");
                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            } else {
                println!("✗ WSL2 is not properly installed");
            }
        }
        Err(e) => println!("✗ WSL2 check failed: {}", e),
    }
    
    Ok(())
}

/// List installed WSL2 distributions
pub async fn list_distros() -> Result<()> {
    info!("Listing WSL2 distributions");
    
    match Command::new("wsl")
        .args(&["--list"])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                println!("Installed distributions:");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                println!("Failed to list distributions");
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}

/// Set default WSL2 distribution
pub async fn set_default(distro: &str) -> Result<()> {
    info!("Setting default distro to {}", distro);
    
    match Command::new("wsl")
        .args(&["--set-default", distro])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                println!("✓ Default distribution set to {}", distro);
            } else {
                println!("✗ Failed to set default distribution");
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}

/// Execute a command in WSL2
#[allow(dead_code)]
pub async fn execute(distro: Option<&str>, command: &str) -> Result<()> {
    let mut cmd = Command::new("wsl");
    
    if let Some(d) = distro {
        cmd.args(&["-d", d]);
    }
    
    cmd.arg(command);
    
    let output = cmd.output()?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}
