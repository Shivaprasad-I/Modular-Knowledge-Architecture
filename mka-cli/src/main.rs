mod models;
mod utils;
mod analyzer;
mod commands;

use clap::Parser;
use anyhow::Result;
use crate::models::enums::cli_commands::Commands;

#[derive(Parser)]
#[command(name = "mka")]
#[command(about = "Modular Knowledge Architecture Utility for Token Efficiency", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => commands::init::handle()?,
        Commands::Features | Commands::Workflows => commands::features::handle()?,
        Commands::Feature { id, view } => commands::feature::handle(id, *view)?,
        Commands::Sync => commands::sync::handle()?,
    }

    Ok(())
}
