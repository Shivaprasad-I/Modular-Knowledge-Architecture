use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize MKA in the current directory
    Init,
    /// List all features/workflows
    FeaturesList,
    /// Alias for features
    Workflows,

    /// Get details of a specific feature
    Feature {
        /// The ID of the feature/workflow
        id: String,
        /// Show detailed minified logic
        #[arg(short, long)]
        view: bool,
    },
    /// Sync MKA documentation with the codebase
    Sync,
    /// Install a tree-sitter parser for a specific language
    Install {
        /// The language to install (e.g., rust, python, typescript)
        language: String,
    },
    /// Get the minified logic of a specific method from a file
    GetMethod {
        /// The relative path to the file
        path: String,
        /// The name of the method to extract
        method: String,
    },
}
