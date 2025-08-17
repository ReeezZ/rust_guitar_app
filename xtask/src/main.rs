use anyhow::Result;
use clap::{Parser, Subcommand};

mod config;
mod tasks;
mod commands;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Guitar Practice App - Development Tasks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start both frontend and backend development servers
    Dev,
    /// Start only the frontend development server
    Frontend,
    /// Start only the backend API server
    Backend,
    /// Run tests for all workspaces
    Test,
    /// Check all workspaces
    Check,
    /// Run linting (clippy) for all workspaces
    Lint,
    /// Build the frontend for production
    Build,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dev => commands::run_dev_command().await?,
        Commands::Frontend => tasks::run_trunk(&["serve", "--open"], "🎨 Starting Frontend Development Server")?,
        Commands::Backend => commands::run_backend_command()?,
        Commands::Test => tasks::run_cargo_workspace(&["test", "--workspace"], "🧪 Running tests for all workspaces...")?,
        Commands::Check => tasks::run_cargo_workspace(&["check", "--workspace"], "🔍 Checking all workspaces...")?,
        Commands::Lint => tasks::run_cargo_workspace(&["clippy", "--workspace", "--", "-D", "warnings"], "🔍 Running clippy for all workspaces...")?,
        Commands::Build => tasks::run_trunk(&["build", "--release"], "🏗️ Building frontend for production...")?,
    }

    Ok(())
}
