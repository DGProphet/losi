use clap::{Parser, Subcommand};
use std::error::Error;
use tracing_subscriber;

mod config;
mod shell;
mod windows;
mod wsl2;
mod cli;
mod history;

use config::Config;
use shell::Shell;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "hshell")]
#[command(about = "Hybrid Shell for Windows 11 - Modern CLI with Unix/Windows interoperability", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Start interactive shell mode (default if no command)
    #[arg(short, long, global = true)]
    interactive: bool,

    /// Configuration file path
    #[arg(short, long, global = true)]
    config: Option<String>,

    /// Verbosity level (v=info, vv=debug, vvv=trace)
    #[arg(short, action = clap::ArgAction::Count, global = true)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize configuration
    Init {
        /// Force overwrite existing config
        #[arg(short, long)]
        force: bool,
    },
    /// Show current configuration
    Config {
        /// Show config path
        #[arg(long)]
        path: bool,
    },
    /// Manage themes
    Theme {
        #[command(subcommand)]
        action: Option<ThemeCommand>,
    },
    /// WSL2 integration commands
    Wsl {
        #[command(subcommand)]
        action: Option<WslCommand>,
    },
    /// Shell environment commands
    Env {
        #[command(subcommand)]
        action: Option<EnvCommand>,
    },
}

#[derive(Subcommand)]
enum ThemeCommand {
    /// List available themes
    List,
    /// Apply a theme
    Apply { name: String },
    /// Show current theme
    Current,
}

#[derive(Subcommand)]
enum WslCommand {
    /// Check WSL2 status
    Status,
    /// List available distributions
    List,
    /// Set default distribution
    SetDefault { distro: String },
}

#[derive(Subcommand)]
enum EnvCommand {
    /// Show all environment variables
    List,
    /// Get a specific variable
    Get { name: String },
    /// Set a variable
    Set { name: String, value: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Initialize tracing
    let level = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    tracing_subscriber::fmt()
        .with_env_filter(level)
        .init();

    // Load configuration
    let config = Config::load(cli.config.as_deref()).await?;

    // Handle commands
    match cli.command {
        Some(Commands::Init { force }) => {
            config::init::initialize_config(force).await?
        }
        Some(Commands::Config { path }) => {
            config::commands::show_config(path).await?
        }
        Some(Commands::Theme { action }) => {
            match action {
                Some(ThemeCommand::List) => config::theme::list_themes().await?,
                Some(ThemeCommand::Apply { name }) => {
                    config::theme::apply_theme(&config, &name).await?
                }
                Some(ThemeCommand::Current) => config::theme::show_current(&config).await?,
                None => println!("Use 'theme list', 'theme apply <name>', or 'theme current'"),
            }
        }
        Some(Commands::Wsl { action }) => {
            match action {
                Some(WslCommand::Status) => wsl2::check_status().await?,
                Some(WslCommand::List) => wsl2::list_distros().await?,
                Some(WslCommand::SetDefault { distro }) => {
                    wsl2::set_default(&distro).await?
                }
                None => println!("Use 'wsl status', 'wsl list', or 'wsl set-default <distro>"),
            }
        }
        Some(Commands::Env { action }) => {
            match action {
                Some(EnvCommand::List) => cli::env::list_vars(),
                Some(EnvCommand::Get { name }) => cli::env::get_var(&name)?,
                Some(EnvCommand::Set { name, value }) => cli::env::set_var(&name, &value)?,
                None => println!("Use 'env list', 'env get <name>', or 'env set <name> <value>"),
            }
        }
        None => {
            // Start interactive shell
            let mut shell = Shell::new(config).await?;
            shell.run_interactive().await?
        }
    }

    Ok(())
}
