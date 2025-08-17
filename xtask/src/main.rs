use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::Command;
use tokio::process::Command as AsyncCommand;

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
            println!("🔧 Starting backend server (http://127.0.0.1:8080)...");
            let mut backend = AsyncCommand::new("cargo")
                .args(["run", "--package", "backend"])
                .spawn()?;

            // Give backend time to start
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            // Start frontend
            println!("🎨 Starting frontend server (http://127.0.0.1:3010)...");
            let mut frontend = AsyncCommand::new("trunk")
                .args(["serve", "--open"])
                .current_dir("frontend")
                .env("RUSTFLAGS", "--cfg getrandom_backend=\"wasm_js\"")
                .spawn()?;

            println!();
            println!("✅ Development servers started!");
            println!();
            println!("📍 Services:");
            println!("   🎨 Frontend: http://127.0.0.1:3010");
            println!("   🔧 Backend:  http://127.0.0.1:8080");
            println!("   📝 API:      http://127.0.0.1:8080/api/exercises");
            println!();
            println!("📋 Press Ctrl+C to stop both servers");

            // Wait for frontend to complete (user will Ctrl+C)
            let _ = frontend.wait().await;
            
            // Kill backend
            backend.kill().await?;
        }
        Commands::Frontend => {
            println!("🎨 Starting Frontend Development Server");
            println!("📍 Frontend will be available at: http://127.0.0.1:3010");
            
            Command::new("trunk")
                .args(["serve", "--open"])
                .current_dir("frontend")
                .env("RUSTFLAGS", "--cfg getrandom_backend=\"wasm_js\"")
                .status()?;
        }
        Commands::Backend => {
            println!("🔧 Starting Backend API Server");
            println!("📍 API will be available at: http://127.0.0.1:8080");
            
            Command::new("cargo")
                .args(["run", "--package", "backend"])
                .status()?;
        }
        Commands::Test => {
            println!("🧪 Running tests for all workspaces...");
            Command::new("cargo")
                .args(["test", "--workspace"])
                .status()?;
        }
        Commands::Check => {
            println!("🔍 Checking all workspaces...");
            Command::new("cargo")
                .args(["check", "--workspace"])
                .status()?;
        }
        Commands::Build => {
            println!("🏗️ Building frontend for production...");
            Command::new("trunk")
                .args(["build", "--release"])
                .current_dir("frontend")
                .env("RUSTFLAGS", "--cfg getrandom_backend=\"wasm_js\"")
                .status()?;
        }
    }

    Ok(())
}
