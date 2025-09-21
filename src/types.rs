//! Core data types for the screenshot application
//! 
//! This module defines all the fundamental data structures used throughout
//! the screenshot application, including capture areas, annotations, settings,
//! and error types with comprehensive error handling.

use egui::{Pos2, Rect, Vec2, Color32};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Represents a screen capture area with DPI information
#[derive(Debug, Clone, PartialEq)]
pub struct CaptureArea {
    pub bounds: Rect,
    pub screen_index: usize,
    pub dpi_scale_x: f32,
    pub dpi_scale_y: f32,
}

impl Default for CaptureArea {
    fn default() -> Self {
        Self {
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0)),
            screen_index: 0,
            dpi_scale_x: 1.0,
            dpi_scale_y: 1.0,
        }
    }
}

/// Information about a screen/monitor
#[derive(Debug, Clone, PartialEq)]
pub struct ScreenInfo {
    pub index: usize,
    pub bounds: Rect,
    pub dpi_scale_x: f32,
    pub dpi_scale_y: f32,
    pub is_primary: bool,
}

/// Annotation item that can be placed on an image
#[derive(Debug, Clone, PartialEq)]
pub struct AnnotationItem {
    pub id: Uuid,
    pub position: Pos2,
    pub is_selected: bool,
    pub annotation_type: AnnotationType,
}

impl AnnotationItem {
    /// Create a new rectangle annotation
    pub fn new_rectangle(position: Pos2, size: Vec2) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            is_selected: false,
            annotation_type: AnnotationType::Rectangle {
                size,
                stroke_color: Color32::RED,
                stroke_width: 2.0,
            },
        }
    }

    /// Create a new text annotation
    pub fn new_text(position: Pos2, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            is_selected: false,
            annotation_type: AnnotationType::Text {
                content,
                font_size: 14.0,
                color: Color32::BLACK,
            },
        }
    }

    /// Get the bounding rectangle of this annotation
    pub fn bounds(&self) -> Rect {
        match &self.annotation_type {
            AnnotationType::Rectangle { size, .. } => {
                Rect::from_min_size(self.position, *size)
            }
            AnnotationType::Text { font_size, content, .. } => {
                // Approximate text bounds based on font size and content length
                let width = content.len() as f32 * font_size * 0.6;
                let height = *font_size * 1.2;
                Rect::from_min_size(self.position, Vec2::new(width, height))
            }
        }
    }

    /// Check if a point is inside this annotation
    pub fn contains_point(&self, point: Pos2) -> bool {
        self.bounds().contains(point)
    }
}

/// Types of annotations that can be added to images
#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationType {
    Rectangle {
        size: Vec2,
        stroke_color: Color32,
        stroke_width: f32,
    },
    Text {
        content: String,
        font_size: f32,
        color: Color32,
    },
}

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppSettings {
    pub hotkey_modifiers: u32,
    pub hotkey_vk_code: u32,
    pub default_save_directory: Option<String>,
    pub default_image_format: ImageFormat,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            // Ctrl + Shift modifiers
            hotkey_modifiers: 0x0002 | 0x0004, // MOD_CONTROL | MOD_SHIFT
            hotkey_vk_code: 0x53, // 'S' key
            default_save_directory: None,
            default_image_format: ImageFormat::Png,
        }
    }
}

/// Supported image formats for saving
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImageFormat {
    Png,
    Jpg,
    Bmp,
}

/// Application error types
#[derive(Error, Debug)]
pub enum AppError {
    #[error("ホットキー登録に失敗しました: {0}")]
    HotkeyRegistration(String),
    
    #[error("スクリーンキャプチャに失敗しました: {0}")]
    ScreenCapture(String),
    
    #[error("ファイルアクセスエラー: {0}")]
    FileAccess(#[from] std::io::Error),
    
    #[error("クリップボードエラー: {0}")]
    Clipboard(String),
    
    #[error("画像処理エラー: {0}")]
    ImageProcessing(String),
    
    #[error("設定エラー: {0}")]
    Settings(String),
}

/// Result type alias for application operations
pub type AppResult<T> = Result<T, AppError>;

/// Hotkey event information
#[derive(Debug, Clone, PartialEq)]
pub struct HotkeyEvent {
    pub id: i32,
    pub modifiers: u32,
    pub vk_code: u32,
}

/// Available editing tools
#[derive(Debug, Clone, PartialEq)]
pub enum Tool {
    Select,
    Rectangle,
    Text,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Select
    }
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageFormat::Png => write!(f, "PNG"),
            ImageFormat::Jpg => write!(f, "JPEG"),
            ImageFormat::Bmp => write!(f, "BMP"),
        }
    }
}

impl ImageFormat {
    /// Get the file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpg => "jpg",
            ImageFormat::Bmp => "bmp",
        }
    }

    /// Get all supported formats
    pub fn all() -> Vec<ImageFormat> {
        vec![ImageFormat::Png, ImageFormat::Jpg, ImageFormat::Bmp]
    }
}

impl CaptureArea {
    /// Create a new capture area
    pub fn new(bounds: Rect, screen_index: usize) -> Self {
        Self {
            bounds,
            screen_index,
            dpi_scale_x: 1.0,
            dpi_scale_y: 1.0,
        }
    }

    /// Create a capture area with DPI scaling
    pub fn with_dpi_scaling(bounds: Rect, screen_index: usize, dpi_scale_x: f32, dpi_scale_y: f32) -> Self {
        Self {
            bounds,
            screen_index,
            dpi_scale_x,
            dpi_scale_y,
        }
    }

    /// Get the physical pixel bounds accounting for DPI scaling
    pub fn physical_bounds(&self) -> Rect {
        let min = Pos2::new(
            self.bounds.min.x * self.dpi_scale_x,
            self.bounds.min.y * self.dpi_scale_y,
        );
        let size = Vec2::new(
            self.bounds.width() * self.dpi_scale_x,
            self.bounds.height() * self.dpi_scale_y,
        );
        Rect::from_min_size(min, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_area_default() {
        let area = CaptureArea::default();
        assert_eq!(area.screen_index, 0);
        assert_eq!(area.dpi_scale_x, 1.0);
        assert_eq!(area.dpi_scale_y, 1.0);
        assert_eq!(area.bounds.min, Pos2::ZERO);
        assert_eq!(area.bounds.size(), Vec2::new(100.0, 100.0));
    }

    #[test]
    fn test_capture_area_custom() {
        let bounds = Rect::from_min_size(Pos2::new(10.0, 20.0), Vec2::new(200.0, 150.0));
        let area = CaptureArea {
            bounds,
            screen_index: 1,
            dpi_scale_x: 1.5,
            dpi_scale_y: 2.0,
        };
        
        assert_eq!(area.bounds, bounds);
        assert_eq!(area.screen_index, 1);
        assert_eq!(area.dpi_scale_x, 1.5);
        assert_eq!(area.dpi_scale_y, 2.0);
    }

    #[test]
    fn test_screen_info_creation() {
        let bounds = Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0));
        let screen = ScreenInfo {
            index: 0,
            bounds,
            dpi_scale_x: 1.0,
            dpi_scale_y: 1.0,
            is_primary: true,
        };
        
        assert_eq!(screen.index, 0);
        assert!(screen.is_primary);
        assert_eq!(screen.bounds.size(), Vec2::new(1920.0, 1080.0));
    }

    #[test]
    fn test_annotation_rectangle_creation() {
        let pos = Pos2::new(10.0, 20.0);
        let size = Vec2::new(50.0, 30.0);
        
        let rect_annotation = AnnotationItem::new_rectangle(pos, size);
        assert_eq!(rect_annotation.position, pos);
        assert!(!rect_annotation.is_selected);
        
        match rect_annotation.annotation_type {
            AnnotationType::Rectangle { size: rect_size, stroke_color, stroke_width } => {
                assert_eq!(rect_size, size);
                assert_eq!(stroke_color, Color32::RED);
                assert_eq!(stroke_width, 2.0);
            }
            _ => panic!("Expected Rectangle annotation type"),
        }
    }

    #[test]
    fn test_annotation_text_creation() {
        let pos = Pos2::new(15.0, 25.0);
        let content = "Test Text".to_string();
        
        let text_annotation = AnnotationItem::new_text(pos, content.clone());
        assert_eq!(text_annotation.position, pos);
        assert!(!text_annotation.is_selected);
        
        match text_annotation.annotation_type {
            AnnotationType::Text { content: text_content, font_size, color } => {
                assert_eq!(text_content, content);
                assert_eq!(font_size, 14.0);
                assert_eq!(color, Color32::BLACK);
            }
            _ => panic!("Expected Text annotation type"),
        }
    }

    #[test]
    fn test_annotation_unique_ids() {
        let pos = Pos2::new(0.0, 0.0);
        let ann1 = AnnotationItem::new_rectangle(pos, Vec2::new(10.0, 10.0));
        let ann2 = AnnotationItem::new_rectangle(pos, Vec2::new(10.0, 10.0));
        
        assert_ne!(ann1.id, ann2.id);
    }

    #[test]
    fn test_app_settings_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.hotkey_vk_code, 0x53); // 'S' key
        assert_eq!(settings.hotkey_modifiers, 0x0002 | 0x0004); // Ctrl + Shift
        assert!(settings.default_save_directory.is_none());
        
        match settings.default_image_format {
            ImageFormat::Png => {},
            _ => panic!("Expected PNG as default format"),
        }
    }

    #[test]
    fn test_app_settings_serialization() {
        let settings = AppSettings::default();
        
        // Test that the settings can be serialized (this would fail at compile time if serde derives are missing)
        let _json = serde_json::to_string(&settings);
    }

    #[test]
    fn test_image_format_variants() {
        let png = ImageFormat::Png;
        let jpg = ImageFormat::Jpg;
        let bmp = ImageFormat::Bmp;
        
        // Test that all variants can be created and are different
        assert!(matches!(png, ImageFormat::Png));
        assert!(matches!(jpg, ImageFormat::Jpg));
        assert!(matches!(bmp, ImageFormat::Bmp));
    }

    #[test]
    fn test_app_error_display() {
        let error = AppError::HotkeyRegistration("Test error".to_string());
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("ホットキー登録に失敗しました"));
        assert!(error_msg.contains("Test error"));
    }

    #[test]
    fn test_app_error_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let app_error = AppError::from(io_error);
        
        match app_error {
            AppError::FileAccess(_) => {},
            _ => panic!("Expected FileAccess error variant"),
        }
    }

    #[test]
    fn test_hotkey_event_creation() {
        let event = HotkeyEvent {
            id: 1,
            modifiers: 0x0002,
            vk_code: 0x53,
        };
        
        assert_eq!(event.id, 1);
        assert_eq!(event.modifiers, 0x0002);
        assert_eq!(event.vk_code, 0x53);
    }

    #[test]
    fn test_tool_variants() {
        let select = Tool::Select;
        let rectangle = Tool::Rectangle;
        let text = Tool::Text;
        
        assert_eq!(select, Tool::Select);
        assert_eq!(rectangle, Tool::Rectangle);
        assert_eq!(text, Tool::Text);
        
        // Test that they are different
        assert_ne!(select, rectangle);
        assert_ne!(rectangle, text);
        assert_ne!(select, text);
    }

    #[test]
    fn test_tool_default() {
        let tool = Tool::default();
        assert_eq!(tool, Tool::Select);
    }

    #[test]
    fn test_app_result_type_alias() {
        // Test that AppResult works as expected
        let success: AppResult<i32> = Ok(42);
        let failure: AppResult<i32> = Err(AppError::Settings("Test".to_string()));
        
        assert!(success.is_ok());
        assert!(failure.is_err());
        
        match success {
            Ok(value) => assert_eq!(value, 42),
            Err(_) => panic!("Expected Ok value"),
        }
    }

    #[test]
    fn test_annotation_bounds() {
        let pos = Pos2::new(10.0, 20.0);
        let size = Vec2::new(50.0, 30.0);
        
        let rect_annotation = AnnotationItem::new_rectangle(pos, size);
        let bounds = rect_annotation.bounds();
        
        assert_eq!(bounds.min, pos);
        assert_eq!(bounds.size(), size);
    }

    #[test]
    fn test_annotation_contains_point() {
        let pos = Pos2::new(10.0, 20.0);
        let size = Vec2::new(50.0, 30.0);
        
        let annotation = AnnotationItem::new_rectangle(pos, size);
        
        // Point inside
        assert!(annotation.contains_point(Pos2::new(30.0, 35.0)));
        
        // Point outside
        assert!(!annotation.contains_point(Pos2::new(5.0, 15.0)));
        assert!(!annotation.contains_point(Pos2::new(70.0, 60.0)));
    }

    #[test]
    fn test_image_format_display() {
        assert_eq!(format!("{}", ImageFormat::Png), "PNG");
        assert_eq!(format!("{}", ImageFormat::Jpg), "JPEG");
        assert_eq!(format!("{}", ImageFormat::Bmp), "BMP");
    }

    #[test]
    fn test_image_format_extension() {
        assert_eq!(ImageFormat::Png.extension(), "png");
        assert_eq!(ImageFormat::Jpg.extension(), "jpg");
        assert_eq!(ImageFormat::Bmp.extension(), "bmp");
    }

    #[test]
    fn test_image_format_all() {
        let formats = ImageFormat::all();
        assert_eq!(formats.len(), 3);
        assert!(formats.contains(&ImageFormat::Png));
        assert!(formats.contains(&ImageFormat::Jpg));
        assert!(formats.contains(&ImageFormat::Bmp));
    }

    #[test]
    fn test_capture_area_constructors() {
        let bounds = Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(100.0, 100.0));
        
        let area1 = CaptureArea::new(bounds, 1);
        assert_eq!(area1.bounds, bounds);
        assert_eq!(area1.screen_index, 1);
        assert_eq!(area1.dpi_scale_x, 1.0);
        assert_eq!(area1.dpi_scale_y, 1.0);
        
        let area2 = CaptureArea::with_dpi_scaling(bounds, 2, 1.5, 2.0);
        assert_eq!(area2.bounds, bounds);
        assert_eq!(area2.screen_index, 2);
        assert_eq!(area2.dpi_scale_x, 1.5);
        assert_eq!(area2.dpi_scale_y, 2.0);
    }

    #[test]
    fn test_capture_area_physical_bounds() {
        let bounds = Rect::from_min_size(Pos2::new(10.0, 20.0), Vec2::new(100.0, 50.0));
        let area = CaptureArea::with_dpi_scaling(bounds, 0, 2.0, 1.5);
        
        let physical = area.physical_bounds();
        assert_eq!(physical.min.x, 20.0); // 10.0 * 2.0
        assert_eq!(physical.min.y, 30.0); // 20.0 * 1.5
        assert_eq!(physical.width(), 200.0); // 100.0 * 2.0
        assert_eq!(physical.height(), 75.0); // 50.0 * 1.5
    }
}