use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize MKA in the current directory
    Init,
    /// List all features/workflows
    Features,
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
}
