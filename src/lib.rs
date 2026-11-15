//! Version-locked composition and integration runtime for Filament V1.
#![recursion_limit = "256"] // The correspondence record is one large `json!` object.

pub mod temporal;

/// Placeholder entry point.
pub fn hello() -> &'static str {
    "hello from filament_integration"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_returns_greeting() {
        assert!(hello().contains("filament_integration"));
    }
}
