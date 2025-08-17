/// Configuration constants for the development environment
pub const FRONTEND_PORT: u16 = 3010;
pub const BACKEND_PORT: u16 = 8080;
pub const BACKEND_STARTUP_DELAY_MS: u64 = 2000;
pub const RUSTFLAGS_WASM: &str = "--cfg getrandom_backend=\"wasm_js\"";

/// Generate URL for a given port
pub fn url(port: u16) -> String {
    format!("http://127.0.0.1:{port}")
}
