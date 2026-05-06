use clap::{Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize MKA in the current directory
    Init,
    /// List all available actions
    Actions,
    /// Alias for actions
    Workflows,
    /// Get details of a specific trigger map
    TriggerMap {
        /// The ID of the action/trigger-map
        id: String,
        /// Suppress minified snippets and return TOON JSON
        #[arg(long)]
        no_snippets: bool,
    },
    /// Alias for trigger-map
    Feature {
        id: String,
        #[arg(long)]
        no_view: bool,
    },
    /// Sync the MKA index and heal broken paths
    Sync,
    /// Install a tree-sitter parser
    Install {
        /// Language to install (e.g., rust, python)
        language: String,
    },
    /// Surgically extract a method signature and logic
    GetMethod {
        /// Path to the source file
        path: String,
        /// Name of the method
        method: String,
    },
}
