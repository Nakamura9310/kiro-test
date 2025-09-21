//! Lightweight Screenshot Application
//! 
//! A fast and lightweight screenshot application for Windows PC
//! that allows users to capture screen areas and perform basic editing.

pub mod types;
pub mod capture;

// Re-export commonly used types
pub use types::*;
pub use capture::CaptureService;