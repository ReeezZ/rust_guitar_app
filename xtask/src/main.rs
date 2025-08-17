use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::process::Command;

// Configuration constants
const FRONTEND_PORT: u16 = 3010;
const BACKEND_PORT: u16 = 8080;
const BACKEND_STARTUP_DELAY_MS: u64 = 2000;
const RUSTFLAGS_WASM: &str = "--cfg getrandom_backend=\"wasm_js\"";

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

/// Helper functions for common operations
mod tasks {
    use super::{Result, Context, Command, FRONTEND_PORT, BACKEND_PORT, RUSTFLAGS_WASM};
    
    /// Generate URL for a given port
    pub fn url(port: u16) -> String {
        format!("http://127.0.0.1:{port}")
    }
    
    /// Run a cargo workspace command with proper error handling
    pub fn run_cargo_workspace(args: &[&str], description: &str) -> Result<()> {
        println!("{description}");
        Command::new("cargo")
            .args(args)
            .status()
            .with_context(|| format!("Failed to run: cargo {}", args.join(" ")))?;
        Ok(())
    }
    
    /// Run a trunk command with WASM flags
    pub fn run_trunk(args: &[&str], description: &str) -> Result<()> {
        println!("{description}");
        if args.contains(&"serve") {
            println!("📍 Frontend will be available at: {}", url(FRONTEND_PORT));
        }
        
        Command::new("trunk")
            .args(args)
            .current_dir("frontend")
            .env("RUSTFLAGS", RUSTFLAGS_WASM)
            .status()
            .with_context(|| format!("Failed to run: trunk {}", args.join(" ")))?;
        Ok(())
    }
    
    /// Print service URLs for development mode
    pub fn print_dev_info() {
        println!();
        println!("✅ Development servers started!");
        println!();
        println!("📍 Services:");
        println!("   🎨 Frontend: {}", url(FRONTEND_PORT));
        println!("   🔧 Backend:  {}", url(BACKEND_PORT));
        println!("   📝 API:      {}/api/exercises", url(BACKEND_PORT));
        println!();
        println!("📋 Press Ctrl+C to stop both servers");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dev => run_dev_command().await?,
        Commands::Frontend => tasks::run_trunk(&["serve", "--open"], "🎨 Starting Frontend Development Server")?,
        Commands::Backend => run_backend_command()?,
        Commands::Test => tasks::run_cargo_workspace(&["test", "--workspace"], "🧪 Running tests for all workspaces...")?,
        Commands::Check => tasks::run_cargo_workspace(&["check", "--workspace"], "🔍 Checking all workspaces...")?,
        Commands::Lint => tasks::run_cargo_workspace(&["clippy", "--workspace", "--", "-D", "warnings"], "🔍 Running clippy for all workspaces...")?,
        Commands::Build => tasks::run_trunk(&["build", "--release"], "🏗️ Building frontend for production...")?,
    }

    Ok(())
}

/// Handle the complex dev command that starts both services
async fn run_dev_command() -> Result<()> {
    use tokio::process::Command as AsyncCommand;
    
    println!("🎸 Starting Guitar Practice App Development Environment");
    println!();
    
    // Start backend in background
    println!("🔧 Starting backend server ({})...", tasks::url(BACKEND_PORT));
    let mut backend = AsyncCommand::new("cargo")
        .args(["run", "--package", "backend"])
        .spawn()
        .context("Failed to start backend server")?;

    // Give backend time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(BACKEND_STARTUP_DELAY_MS)).await;

    // Start frontend
    println!("🎨 Starting frontend server ({})...", tasks::url(FRONTEND_PORT));
    let mut frontend = AsyncCommand::new("trunk")
        .args(["serve", "--open"])
        .current_dir("frontend")
        .env("RUSTFLAGS", RUSTFLAGS_WASM)
        .spawn()
        .context("Failed to start frontend server")?;

    tasks::print_dev_info();

    // Wait for frontend to complete or be interrupted
    let result = handle_dev_process_lifecycle(&mut frontend).await;

    // Cleanup: kill backend
    if let Err(e) = backend.kill().await {
        eprintln!("Warning: Failed to kill backend process: {e}");
    }
    
    println!("✅ All servers stopped");
    result
}

/// Handle backend command
fn run_backend_command() -> Result<()> {
    println!("🔧 Starting Backend API Server");
    println!("📍 API will be available at: {}", tasks::url(BACKEND_PORT));
    
    Command::new("cargo")
        .args(["run", "--package", "backend"])
        .status()
        .context("Failed to run backend")?;
    Ok(())
}

/// Handle the lifecycle of development processes (frontend + ctrl+c handling)
async fn handle_dev_process_lifecycle(frontend: &mut tokio::process::Child) -> Result<()> {
    tokio::select! {
        result = frontend.wait() => {
            match result {
                Ok(status) => {
                    println!("Frontend process ended with status: {status}");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Frontend process error: {e}");
                    Err(e.into())
                }
            }
        }
        _ = tokio::signal::ctrl_c() => {
            println!("\nReceived Ctrl+C, shutting down gracefully...");
            Ok(())
        }
    }
}
