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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => commands::init::handle().await?,
        Commands::WorkflowList => commands::workflow_list::handle().await?,
        Commands::WorkflowGet { id, no_snippets } => commands::workflow_get::handle(id, !*no_snippets).await?,
        Commands::Feature { id, no_view } => commands::workflow_get::handle(id, !*no_view).await?,

        Commands::Sync => commands::sync::handle().await?,
        Commands::Install { language } => commands::install::handle(language).await?,
        Commands::GetMethod { path, method } => commands::get_method::handle(path, method).await?,
        Commands::Mcp => commands::mcp::handle().await.map_err(|e| anyhow::anyhow!(e.to_string()))?,
    }


    Ok(())
}
