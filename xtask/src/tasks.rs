use anyhow::{Context, Result};
use std::process::Command;
use crate::config::{FRONTEND_PORT, BACKEND_PORT, RUSTFLAGS_WASM, url};

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
