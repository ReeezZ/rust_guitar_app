use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::process::Command;
use tokio::process::Command as AsyncCommand;

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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dev => {
            println!("🎸 Starting Guitar Practice App Development Environment");
            println!();
            
            // Start backend in background
            println!("🔧 Starting backend server (http://127.0.0.1:{BACKEND_PORT})...");
            let mut backend = AsyncCommand::new("cargo")
                .args(["run", "--package", "backend"])
                .spawn()
                .context("Failed to start backend server")?;

            // Give backend time to start
            tokio::time::sleep(tokio::time::Duration::from_millis(BACKEND_STARTUP_DELAY_MS)).await;

            // Start frontend
            println!("🎨 Starting frontend server (http://127.0.0.1:{FRONTEND_PORT})...");
            let mut frontend = AsyncCommand::new("trunk")
                .args(["serve", "--open"])
                .current_dir("frontend")
                .env("RUSTFLAGS", RUSTFLAGS_WASM)
                .spawn()
                .context("Failed to start frontend server")?;

            println!();
            println!("✅ Development servers started!");
            println!();
            println!("📍 Services:");
            println!("   🎨 Frontend: http://127.0.0.1:{FRONTEND_PORT}");
            println!("   🔧 Backend:  http://127.0.0.1:{BACKEND_PORT}");
            println!("   📝 API:      http://127.0.0.1:{BACKEND_PORT}/api/exercises");
            println!();
            println!("📋 Press Ctrl+C to stop both servers");

            // Wait for frontend to complete or be interrupted
            let result: Result<()> = tokio::select! {
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
            };

            // Cleanup: kill backend
            if let Err(e) = backend.kill().await {
                eprintln!("Warning: Failed to kill backend process: {e}");
            }
            
            println!("✅ All servers stopped");
            result?;
        }
        Commands::Frontend => {
            println!("🎨 Starting Frontend Development Server");
            println!("📍 Frontend will be available at: http://127.0.0.1:{FRONTEND_PORT}");
            
            Command::new("trunk")
                .args(["serve", "--open"])
                .current_dir("frontend")
                .env("RUSTFLAGS", RUSTFLAGS_WASM)
                .status()
                .context("Failed to run trunk serve")?;
        }
        Commands::Backend => {
            println!("🔧 Starting Backend API Server");
            println!("📍 API will be available at: http://127.0.0.1:{BACKEND_PORT}");
            
            Command::new("cargo")
                .args(["run", "--package", "backend"])
                .status()
                .context("Failed to run backend")?;
        }
        Commands::Test => {
            println!("🧪 Running tests for all workspaces...");
            Command::new("cargo")
                .args(["test", "--workspace"])
                .status()
                .context("Failed to run tests")?;
        }
        Commands::Check => {
            println!("🔍 Checking all workspaces...");
            Command::new("cargo")
                .args(["check", "--workspace"])
                .status()
                .context("Failed to run cargo check")?;
        }
        Commands::Lint => {
            println!("🔍 Running clippy for all workspaces...");
            Command::new("cargo")
                .args(["clippy", "--workspace", "--", "-D", "warnings"])
                .status()
                .context("Failed to run clippy")?;
        }
        Commands::Build => {
            println!("🏗️ Building frontend for production...");
            Command::new("trunk")
                .args(["build", "--release"])
                .current_dir("frontend")
                .env("RUSTFLAGS", RUSTFLAGS_WASM)
                .status()
                .context("Failed to build frontend")?;
        }
    }

    Ok(())
}
