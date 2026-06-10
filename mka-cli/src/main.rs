mod models;
mod utils;
mod analyzer;
mod commands;

use clap::Parser;
use anyhow::Result;
use crate::models::enums::Commands;

#[derive(Parser)]
#[command(name = "mka")]
#[command(version)]
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
        Commands::WorkflowGet { id, snippets } => commands::workflow_get::handle(id, *snippets).await?,
        Commands::Feature { id, view } => commands::workflow_get::handle(id, *view).await?,

        Commands::Install { language, list } => commands::install::handle(language.as_deref(), *list).await?,
        Commands::GetMethod { path, method } => commands::get_method::handle(path, method).await?,
        Commands::Mcp => commands::mcp::handle().await.map_err(|e| anyhow::anyhow!(e.to_string()))?,
        Commands::ModelInstall => commands::model_install::handle().await?,
        Commands::WorkflowSearch { query, list_all } => commands::workflow_search::handle(query.clone(), *list_all).await?,
    }


    Ok(())
}
