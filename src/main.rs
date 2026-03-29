mod manager;
mod managers;

use clap::{Parser, Subcommand};
use anyhow::Result;
use colored::*;
use crate::managers::get_active_managers;

#[derive(Parser)]
#[command(name = "upl")]
#[command(about = "Universal Package Lister - Manage global developer binaries", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all globally installed developer packages
    List,
    /// Check for updates across all package managers
    Update,
    /// Take a snapshot of current global packages
    Snapshot,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let active_managers = get_active_managers();

    match &cli.command {
        Commands::List => {
            println!("{}", "Scanning for global developer tools...".cyan());
            for manager in active_managers {
                match manager.list_globals() {
                    Ok(bins) => {
                        println!("{}: Found {} packages", manager.name().bold().green(), bins.len());
                        for bin in bins {
                            println!("  - {} ({})", bin.name, bin.version.unwrap_or_else(|| "unknown".to_string()));
                        }
                    }
                    Err(e) => println!("{}: Error: {}", manager.name().red(), e),
                }
            }
        }
        Commands::Update => {
            println!("{}", "Checking for updates...".yellow());
            for manager in active_managers {
                println!("Updating {}...", manager.name().bold().green());
                let _ = manager.update_all();
            }
        }
        Commands::Snapshot => {
            println!("{}", "Taking a snapshot of current packages...".green());
            // TODO: Implement snapshot saving to JSON
        }
    }

    Ok(())
}
