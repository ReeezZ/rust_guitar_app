/// Utilities for generating unique IDs across different platforms

/// Simple unique ID generator using timestamps
pub struct IdGenerator;

impl IdGenerator {
    /// Generate a simple unique ID using timestamp
    pub fn generate() -> String {
        #[cfg(feature = "wasm")]
        {
            // Use JavaScript's Date.now() for WASM compatibility
            let timestamp = js_sys::Date::now() as u64;
            format!("ex_{}", timestamp)
        }
        
        #[cfg(not(feature = "wasm"))]
        {
            // Use system timestamp for backend
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as u64;
            format!("ex_{}", timestamp)
        }
    }
}

/// Generate a simple unique ID - convenience function
pub fn generate_id() -> String {
    IdGenerator::generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generation() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert_ne!(id1, id2);
        assert!(!id1.is_empty());
        assert!(id1.starts_with("ex_"));
    }

    #[test]
    fn test_id_generator_struct() {
        let id1 = IdGenerator::generate();
        let id2 = IdGenerator::generate();
        assert_ne!(id1, id2);
        assert!(!id1.is_empty());
    }
}
