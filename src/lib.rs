//! Lightweight Screenshot Application
//! 
//! A fast and lightweight screenshot application for Windows PC
//! that allows users to capture screen areas and perform basic editing.

pub mod types;
pub mod capture;
pub mod editor_app;

// Re-export commonly used types
pub use types::*;
pub use capture::CaptureService;
pub use editor_app::EditorApp;