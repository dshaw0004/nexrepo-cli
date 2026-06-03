mod core; // Make sure to create src/core/mod.rs and expose config.rs

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "nexrepo")]
#[command(about = "Your repositories. Your storage. Zero vendor lock-in.", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize nexrepo for the current local repository
    Init,

    /// Add a new remote provider to sync with
    RemoteAdd {
        /// Name of the remote (e.g., gitlab)
        name: String,
        /// The Git URL (e.g., git@gitlab.com:user/repo.git)
        url: String,
    },

    /// Push the current branch to ALL configured remotes simultaneously
    Push,

    /// Check the sync status of all configured remotes
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("{}", "🚀 Initializing nexrepo...".green());
            // TODO: Call core::commands::init::execute()
        }
        Commands::RemoteAdd { name, url } => {
            println!("{} Adding remote '{}' -> {}", "➕".green(), name, url);
            // TODO: Call core::commands::remote_add::execute(name, url)
        }
        Commands::Push => {
            println!("{}", "📤 Pushing to all configured remotes...".cyan());
            // TODO: Call core::commands::push::execute()
        }
        Commands::Status => {
            println!("{}", "📊 Checking remote sync status...".cyan());
            // TODO: Call core::commands::status::execute()
        }
    }

    Ok(())
}
