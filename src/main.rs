use log::info;
use lightweight_screenshot_app::{AppSettings, EditorApp, Tool};

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
    
    // Configure native options for the egui application
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("軽量スクリーンショットアプリ")
            .with_icon(load_icon()),
        ..Default::default()
    };
    
    info!("Starting egui application...");
    
    // Run the native egui application
    eframe::run_native(
        "軽量スクリーンショットアプリ",
        native_options,
        Box::new(|_cc| {
            // Create and return the editor application
            Box::new(EditorApp::new())
        }),
    )?;
    
    info!("Application closed successfully");
    Ok(())
}

/// Load application icon (placeholder implementation)
fn load_icon() -> egui::IconData {
    // For now, return a default icon
    // TODO: Load actual application icon from resources
    egui::IconData {
        rgba: vec![255; 32 * 32 * 4], // 32x32 white icon
        width: 32,
        height: 32,
    }
}