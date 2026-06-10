use crate::config::Config;
use anyhow::Result;
use std::io::Write;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing::info;

pub struct Shell {
    config: Config,
    command_history: Vec<String>,
}

impl Shell {
    pub async fn new(config: Config) -> Result<Self> {
        info!("Initializing shell with config: {}", config.theme.name);
        Ok(Self {
            config,
            command_history: Vec::new(),
        })
    }

    pub async fn run_interactive(&mut self) -> Result<()> {
        println!("╔════════════════════════════════════════════════╗");
        println!("║  Hybrid Shell for Windows 11 v0.1.0           ║");
        println!("║  Type 'help' for available commands            ║");
        println!("║  Type 'exit' to quit                           ║");
        println!("╚════════════════════════════════════════════════╝");
        println!();

        let stdin = tokio::io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
            print!("{}", self.config.shell.prompt);
            std::io::stdout().flush()?;

            match lines.next_line().await? {
                Some(line) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }

                    self.command_history.push(trimmed.to_string());
                    self.execute_command(trimmed).await?;
                }
                None => break,
            }
        }

        println!("Goodbye!");
        Ok(())
    }

    async fn execute_command(&self, command: &str) -> Result<()> {
        match command {
            "help" => self.show_help(),
            "exit" | "quit" => std::process::exit(0),
            "clear" | "cls" => print!("\x1B[2J\x1B[1;1H"),
            "history" => self.show_history(),
            cmd if cmd.starts_with("wsl ") => self.execute_wsl_command(cmd).await?,
            cmd => self.execute_system_command(cmd).await?,
        }
        Ok(())
    }

    fn show_help(&self) {
        println!("Available commands:");
        println!("  help                - Show this help message");
        println!("  exit/quit           - Exit the shell");
        println!("  clear/cls           - Clear the screen");
        println!("  history             - Show command history");
        println!("  wsl <command>       - Execute in WSL2");
        println!("  <command>           - Execute system command");
    }

    fn show_history(&self) {
        if self.command_history.is_empty() {
            println!("No commands in history");
            return;
        }
        for (idx, cmd) in self.command_history.iter().enumerate() {
            println!("  {} {}", idx + 1, cmd);
        }
    }

    async fn execute_wsl_command(&self, command: &str) -> Result<()> {
        if !self.config.wsl2.enabled {
            println!("WSL2 is not enabled. Run 'hshell wsl status' to check.");
            return Ok(());
        }
        let cmd_part = &command[4..]; // Remove "wsl "
        println!("[WSL2] Executing: {}", cmd_part);
        // TODO: Implement actual WSL2 execution
        Ok(())
    }

    async fn execute_system_command(&self, command: &str) -> Result<()> {
        match std::process::Command::new("cmd")
            .args(&["/C", command])
            .output()
        {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    eprint!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => println!("Error executing command: {}", e),
        }
        Ok(())
    }
}
