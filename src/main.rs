use clap::{Parser, Subcommand};
use std::error::Error;
use tracing_subscriber;

mod config;
mod theme;
mod desktop;
mod explorer;

use config::Config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "losi")]
#[command(about = "Modern LiteStep - Desktop Customization Engine for Windows 11", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Configuration file or theme directory path
    #[arg(short, long, global = true)]
    config: Option<String>,

    /// Verbosity level (v=info, vv=debug, vvv=trace)
    #[arg(short, action = clap::ArgAction::Count, global = true)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize default configuration and themes
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
    /// Start desktop customization (explorer integration or shell replacement)
    Start {
        /// Use shell replacement instead of explorer integration
        #[arg(long)]
        shell: bool,
    },
    /// Stop desktop customization
    Stop,
}

#[derive(Subcommand)]
enum ThemeCommand {
    /// List available themes
    List,
    /// Apply a theme (.rc file)
    Apply { theme_path: String },
    /// Show current theme
    Current,
    /// Reload theme from file
    Reload,
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
                Some(ThemeCommand::List) => theme::list_themes().await?,
                Some(ThemeCommand::Apply { theme_path }) => {
                    theme::apply_theme(&config, &theme_path).await?
                }
                Some(ThemeCommand::Current) => theme::show_current(&config).await?,
                Some(ThemeCommand::Reload) => theme::reload_theme(&config).await?,
                None => println!("Use 'theme list', 'theme apply <path>', 'theme current', or 'theme reload'"),
            }
        }
        Some(Commands::Start { shell }) => {
            if shell {
                println!("Starting LiteStep as shell replacement...");
                // TODO: Shell replacement mode
            } else {
                println!("Starting LiteStep with explorer integration...");
                explorer::start_integration(&config).await?
            }
        }
        Some(Commands::Stop) => {
            println!("Stopping LiteStep...");
            // TODO: Cleanup and restore
        }
        None => {
            println!("\n╔═══════════════════════════════════════════════════════╗");
            println!("║     Modern LiteStep - Desktop Customization Engine    ║");
            println!("║               Windows 11 Customization                ║");
            println!("╚═══════════════════════════════════════════════════════╝\n");
            println!("Usage: losi [COMMAND] [OPTIONS]");
            println!("\nCommands:");
            println!("  init           Initialize configuration");
            println!("  config         Show configuration");
            println!("  theme          Manage themes");
            println!("  start          Start desktop customization");
            println!("  stop           Stop customization");
            println!("\nOptions:");
            println!("  -c, --config <PATH>   Configuration path");
            println!("  -v, --verbose         Increase verbosity");
            println!("\nExamples:");
            println!("  losi init                              # Initialize config");
            println!("  losi theme list                        # List available themes");
            println!("  losi theme apply ./themes/mytheme.rc   # Apply a theme");
            println!("  losi start                             # Start customization");
        }
    }

    Ok(())
}
