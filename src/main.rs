use log::info;
use lightweight_screenshot_app::{AppSettings, AppResult, Tool};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    info!("Lightweight Screenshot App starting...");
    
    // Initialize app settings to verify types work
    let settings = AppSettings::default();
    info!("Loaded settings with hotkey: Ctrl+Shift+S");
    info!("Default image format: {}", settings.default_image_format);
    
    // Initialize default tool
    let current_tool = Tool::default();
    info!("Current tool: {:?}", current_tool);
    
    // TODO: Initialize hotkey manager and start event loop
    println!("Screenshot app initialized successfully");
    
    Ok(())
}