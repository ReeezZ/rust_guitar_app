use anyhow::{Context, Result};
use std::process::Command;
use tokio::process::Command as AsyncCommand;
use crate::config::{FRONTEND_PORT, BACKEND_PORT, BACKEND_STARTUP_DELAY_MS, RUSTFLAGS_WASM, url};
use crate::tasks;

/// Handle the complex dev command that starts both services
pub async fn run_dev_command() -> Result<()> {
    println!("🎸 Starting Guitar Practice App Development Environment");
    println!();
    
    // Start backend in background
    println!("🔧 Starting backend server ({})...", url(BACKEND_PORT));
    let mut backend = AsyncCommand::new("cargo")
        .args(["run", "--package", "backend"])
        .spawn()
        .context("Failed to start backend server")?;

    // Give backend time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(BACKEND_STARTUP_DELAY_MS)).await;

    // Start frontend
    println!("🎨 Starting frontend server ({})...", url(FRONTEND_PORT));
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
pub fn run_backend_command() -> Result<()> {
    println!("🔧 Starting Backend API Server");
    println!("📍 API will be available at: {}", url(BACKEND_PORT));
    
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
