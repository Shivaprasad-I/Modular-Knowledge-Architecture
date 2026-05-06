mod models;
mod utils;
mod analyzer;
mod commands;

use clap::Parser;
use anyhow::Result;
use crate::models::enums::Commands;

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
        Commands::Actions | Commands::Workflows => commands::actions::handle()?,
        Commands::TriggerMap { id, no_snippets } => commands::trigger_map::handle(id, !*no_snippets)?,
        Commands::Feature { id, no_view } => commands::trigger_map::handle(id, !*no_view)?,

        Commands::Sync => commands::sync::handle()?,
        Commands::Install { language } => commands::install::handle(language)?,
        Commands::GetMethod { path, method } => commands::get_method::handle(path, method)?,
    }


    Ok(())
}
