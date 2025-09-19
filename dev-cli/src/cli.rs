use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "dev",
    about = "🚀 Blazingly fast development workflow CLI with AI assistance",
    version = "1.0.0",
    author = "Dev Tools Team"
)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Start development server
    #[command(alias = "s")]
    Start,

    /// Run build and type check
    #[command(alias = "b")]
    Build,

    /// Commit and push changes
    #[command(alias = "p")]
    Push,

    /// Run linting only
    #[command(alias = "l")]
    Lint,

    /// Run type check only
    #[command(alias = "t")]
    TypeCheck,

    /// AI error analysis
    #[command(alias = "ai")]
    Analyze {
        /// Error message to analyze
        #[arg(short, long)]
        error: Option<String>,
    },

    /// Setup AI configuration
    Setup,
}