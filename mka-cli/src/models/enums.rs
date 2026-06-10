use clap::{Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize MKA in the current directory
    Init,
    /// Get details of a specific workflow
    #[command(name = "workflow-get")]
    WorkflowGet {
        /// The ID of the workflow
        id: String,
        /// Include minified snippets and return markdown format
        #[arg(long)]
        snippets: bool,
    },
    /// Alias for workflow-get
    Feature {
        id: String,
        #[arg(long)]
        view: bool,
    },
    /// Install a tree-sitter parser
    Install {
        /// Language to install (e.g., rust, python)
        #[arg(required_unless_present = "list")]
        language: Option<String>,
        /// List all supported languages/treesitters
        #[arg(long, short)]
        list: bool,
    },
    /// Surgically extract a method signature and logic
    GetMethod {
        /// Path to the source file
        path: String,
        /// Name of the method
        method: String,
    },
    /// Start an MCP server on stdio
    Mcp,
    /// Install the embedding model for semantic search
    #[command(name = "model-install")]
    ModelInstall,
    /// Search for workflows using natural language
    #[command(name = "workflow-search")]
    WorkflowSearch {
        /// The search query
        #[arg(required_unless_present = "list_all")]
        query: Option<String>,
        /// List all available workflows
        #[arg(long = "listAll", alias = "list-all")]
        list_all: bool,
    },
}
