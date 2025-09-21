//! Screen capture functionality
//! 
//! This module provides screen capture services including full screen capture,
//! area-specific capture, and multi-monitor support using the screenshots crate.

use crate::types::{AppError, AppResult, CaptureArea, ScreenInfo};
use egui::{Pos2, Rect, Vec2};
use image::DynamicImage;
use screenshots::Screen;
use std::collections::HashMap;

/// Service for capturing screenshots
pub struct CaptureService {
    screens: Vec<Screen>,
    screen_cache: HashMap<usize, ScreenInfo>,
}

impl CaptureService {
    /// Create a new capture service instance
    pub fn new() -> AppResult<Self> {
        let screens = Screen::all();

        if screens.is_empty() {
            return Err(AppError::ScreenCapture(
                "No screens found on the system".to_string(),
            ));
        }

        let mut service = Self {
            screens,
            screen_cache: HashMap::new(),
        };

        // Initialize screen cache
        service.refresh_screen_info()?;
        
        Ok(service)
    }

    /// Capture the entire primary screen
    pub fn capture_primary_screen(&self) -> AppResult<DynamicImage> {
        let primary_screen = self.get_primary_screen()?;
        self.capture_screen_by_index(primary_screen.index)
    }

    /// Capture a specific screen by index
    pub fn capture_screen_by_index(&self, screen_index: usize) -> AppResult<DynamicImage> {
        let screen = self.screens.get(screen_index).ok_or_else(|| {
            AppError::ScreenCapture(format!("Screen index {} not found", screen_index))
        })?;

        let image = screen.capture().ok_or_else(|| {
            AppError::ScreenCapture(format!("Failed to capture screen {}", screen_index))
        })?;

        // Convert screenshots::Image to image::DynamicImage
        // The screenshots crate returns PNG-encoded data, so we need to decode it
        let buffer = image.buffer();
        
        // Decode the PNG data using the image crate
        let dynamic_image = image::load_from_memory(buffer)
            .map_err(|e| {
                AppError::ScreenCapture(format!("Failed to decode PNG data: {}", e))
            })?;

        Ok(dynamic_image)
    }

    /// Capture a specific area of the screen
    pub fn capture_area(&self, area: &CaptureArea) -> AppResult<DynamicImage> {
        // First capture the entire screen
        let full_image = self.capture_screen_by_index(area.screen_index)?;
        
        // Get physical bounds accounting for DPI scaling
        let physical_bounds = area.physical_bounds();
        
        // Validate bounds
        let screen_info = self.get_screen_info(area.screen_index)?;
        if physical_bounds.min.x < 0.0 
            || physical_bounds.min.y < 0.0 
            || physical_bounds.max.x > screen_info.bounds.max.x * screen_info.dpi_scale_x
            || physical_bounds.max.y > screen_info.bounds.max.y * screen_info.dpi_scale_y {
            return Err(AppError::ScreenCapture(
                "Capture area extends beyond screen boundaries".to_string(),
            ));
        }

        // Crop the image to the specified area
        let cropped = full_image.crop_imm(
            physical_bounds.min.x as u32,
            physical_bounds.min.y as u32,
            physical_bounds.width() as u32,
            physical_bounds.height() as u32,
        );

        Ok(cropped)
    }

    /// Get information about all available screens
    pub fn get_screens(&self) -> Vec<ScreenInfo> {
        self.screen_cache.values().cloned().collect()
    }

    /// Get information about a specific screen
    pub fn get_screen_info(&self, screen_index: usize) -> AppResult<&ScreenInfo> {
        self.screen_cache.get(&screen_index).ok_or_else(|| {
            AppError::ScreenCapture(format!("Screen info for index {} not found", screen_index))
        })
    }

    /// Get the primary screen information
    pub fn get_primary_screen(&self) -> AppResult<&ScreenInfo> {
        self.screen_cache
            .values()
            .find(|screen| screen.is_primary)
            .ok_or_else(|| {
                AppError::ScreenCapture("No primary screen found".to_string())
            })
    }

    /// Refresh screen information (useful when display configuration changes)
    pub fn refresh_screen_info(&mut self) -> AppResult<()> {
        self.screen_cache.clear();
        
        // Refresh the screens list
        self.screens = Screen::all();

        // Rebuild screen cache
        for (index, screen) in self.screens.iter().enumerate() {
            // Convert screen coordinates to egui Rect
            let bounds = Rect::from_min_size(
                Pos2::new(screen.x as f32, screen.y as f32),
                Vec2::new(screen.width as f32, screen.height as f32),
            );

            // For now, assume 1.0 DPI scaling - this can be enhanced later with proper DPI detection
            let dpi_scale_x = 1.0;
            let dpi_scale_y = 1.0;

            // Assume the first screen is primary - this can be enhanced later
            let is_primary = index == 0;

            let screen_info = ScreenInfo {
                index,
                bounds,
                dpi_scale_x,
                dpi_scale_y,
                is_primary,
            };

            self.screen_cache.insert(index, screen_info);
        }

        Ok(())
    }

    /// Get the total desktop bounds (useful for multi-monitor setups)
    pub fn get_desktop_bounds(&self) -> Rect {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for screen_info in self.screen_cache.values() {
            min_x = min_x.min(screen_info.bounds.min.x);
            min_y = min_y.min(screen_info.bounds.min.y);
            max_x = max_x.max(screen_info.bounds.max.x);
            max_y = max_y.max(screen_info.bounds.max.y);
        }

        if min_x == f32::MAX {
            // No screens found, return default
            return Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0));
        }

        Rect::from_min_max(
            Pos2::new(min_x, min_y),
            Pos2::new(max_x, max_y),
        )
    }

    /// Find which screen contains a given point
    pub fn find_screen_at_point(&self, point: Pos2) -> Option<&ScreenInfo> {
        self.screen_cache
            .values()
            .find(|screen| screen.bounds.contains(point))
    }

    /// Create a capture area from screen coordinates
    pub fn create_capture_area(&self, start: Pos2, end: Pos2) -> AppResult<CaptureArea> {
        // Normalize coordinates (ensure start is top-left, end is bottom-right)
        let min_x = start.x.min(end.x);
        let min_y = start.y.min(end.y);
        let max_x = start.x.max(end.x);
        let max_y = start.y.max(end.y);

        let bounds = Rect::from_min_max(
            Pos2::new(min_x, min_y),
            Pos2::new(max_x, max_y),
        );

        // Find which screen contains the center of the selection
        let center = bounds.center();
        let screen_info = self.find_screen_at_point(center)
            .ok_or_else(|| {
                AppError::ScreenCapture("Selection area is not within any screen".to_string())
            })?;

        // Convert to screen-relative coordinates
        let relative_bounds = Rect::from_min_max(
            Pos2::new(
                bounds.min.x - screen_info.bounds.min.x,
                bounds.min.y - screen_info.bounds.min.y,
            ),
            Pos2::new(
                bounds.max.x - screen_info.bounds.min.x,
                bounds.max.y - screen_info.bounds.min.y,
            ),
        );

        Ok(CaptureArea::with_dpi_scaling(
            relative_bounds,
            screen_info.index,
            screen_info.dpi_scale_x,
            screen_info.dpi_scale_y,
        ))
    }
}

impl Default for CaptureService {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback for when screen enumeration fails
            Self {
                screens: Vec::new(),
                screen_cache: HashMap::new(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_service_creation() {
        // This test might fail in headless environments, so we handle that gracefully
        match CaptureService::new() {
            Ok(service) => {
                assert!(!service.screens.is_empty());
                assert!(!service.screen_cache.is_empty());
            }
            Err(AppError::ScreenCapture(_)) => {
                // Expected in headless environments
                println!("Skipping test in headless environment");
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[test]
    fn test_capture_service_default() {
        let service = CaptureService::default();
        // Should not panic even if screen enumeration fails
        // This test ensures the default constructor doesn't panic
        let _screen_count = service.screens.len();
    }

    #[test]
    fn test_desktop_bounds_empty_screens() {
        let service = CaptureService {
            screens: Vec::new(),
            screen_cache: HashMap::new(),
        };
        
        let bounds = service.get_desktop_bounds();
        assert_eq!(bounds.min, Pos2::ZERO);
        assert_eq!(bounds.size(), Vec2::new(1920.0, 1080.0));
    }

    #[test]
    fn test_desktop_bounds_single_screen() {
        let mut service = CaptureService {
            screens: Vec::new(),
            screen_cache: HashMap::new(),
        };

        // Add a mock screen
        let screen_info = ScreenInfo {
            index: 0,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0)),
            dpi_scale_x: 1.0,
            dpi_scale_y: 1.0,
            is_primary: true,
        };
        service.screen_cache.insert(0, screen_info);

        let bounds = service.get_desktop_bounds();
        assert_eq!(bounds.min, Pos2::ZERO);
        assert_eq!(bounds.size(), Vec2::new(1920.0, 1080.0));
    }

    #[test]
    fn test_desktop_bounds_multiple_screens() {
        let mut service = CaptureService {
            screens: Vec::new(),
            screen_cache: HashMap::new(),
        };

        // Add mock screens
        let screen1 = ScreenInfo {
            index: 0,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0)),
            dpi_scale_x: 1.0,
            dpi_scale_y: 1.0,
            is_primary: true,
        };
        let screen2 = ScreenInfo {
            index: 1,
            bounds: Rect::from_min_size(Pos2::new(1920.0, 0.0), Vec2::new(1920.0, 1080.0)),
            dpi_scale_x: 1.0,
            dpi_scale_y: 1.0,
            is_primary: false,
        };

        service.screen_cache.insert(0, screen1);
        service.screen_cache.insert(1, screen2);

        let bounds = service.get_desktop_bounds();
        assert_eq!(bounds.min, Pos2::ZERO);
        assert_eq!(bounds.size(), Vec2::new(3840.0, 1080.0)); // Two 1920x1080 screens side by side
    }

    #[test]
    fn test_find_screen_at_point() {
        let mut service = CaptureService {
            screens: Vec::new(),
            screen_cache: HashMap::new(),
        };

        let screen_info = ScreenInfo {
            index: 0,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0)),
            dpi_scale_x: 1.0,
            dpi_scale_y: 1.0,
            is_primary: true,
        };
        service.screen_cache.insert(0, screen_info);

        // Point inside screen
        let found = service.find_screen_at_point(Pos2::new(960.0, 540.0));
        assert!(found.is_some());
        assert_eq!(found.unwrap().index, 0);

        // Point outside screen
        let not_found = service.find_screen_at_point(Pos2::new(2000.0, 540.0));
        assert!(not_found.is_none());
    }

    #[test]
    fn test_create_capture_area() {
        let mut service = CaptureService {
            screens: Vec::new(),
            screen_cache: HashMap::new(),
        };

        let screen_info = ScreenInfo {
            index: 0,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0)),
            dpi_scale_x: 1.0,
            dpi_scale_y: 1.0,
            is_primary: true,
        };
        service.screen_cache.insert(0, screen_info);

        // Create capture area within screen bounds
        let start = Pos2::new(100.0, 100.0);
        let end = Pos2::new(300.0, 200.0);
        
        let result = service.create_capture_area(start, end);
        assert!(result.is_ok());
        
        let area = result.unwrap();
        assert_eq!(area.screen_index, 0);
        assert_eq!(area.bounds.min, Pos2::new(100.0, 100.0));
        assert_eq!(area.bounds.size(), Vec2::new(200.0, 100.0));
    }

    #[test]
    fn test_create_capture_area_normalized_coordinates() {
        let mut service = CaptureService {
            screens: Vec::new(),
            screen_cache: HashMap::new(),
        };

        let screen_info = ScreenInfo {
            index: 0,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0)),
            dpi_scale_x: 1.0,
            dpi_scale_y: 1.0,
            is_primary: true,
        };
        service.screen_cache.insert(0, screen_info);

        // Test with end point before start point (should be normalized)
        let start = Pos2::new(300.0, 200.0);
        let end = Pos2::new(100.0, 100.0);
        
        let result = service.create_capture_area(start, end);
        assert!(result.is_ok());
        
        let area = result.unwrap();
        assert_eq!(area.bounds.min, Pos2::new(100.0, 100.0));
        assert_eq!(area.bounds.max, Pos2::new(300.0, 200.0));
    }

    #[test]
    fn test_create_capture_area_outside_screen() {
        let mut service = CaptureService {
            screens: Vec::new(),
            screen_cache: HashMap::new(),
        };

        let screen_info = ScreenInfo {
            index: 0,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0)),
            dpi_scale_x: 1.0,
            dpi_scale_y: 1.0,
            is_primary: true,
        };
        service.screen_cache.insert(0, screen_info);

        // Create capture area outside screen bounds
        let start = Pos2::new(2000.0, 100.0);
        let end = Pos2::new(2200.0, 200.0);
        
        let result = service.create_capture_area(start, end);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            AppError::ScreenCapture(msg) => {
                assert!(msg.contains("not within any screen"));
            }
            _ => panic!("Expected ScreenCapture error"),
        }
    }

    #[test]
    fn test_get_primary_screen_not_found() {
        let service = CaptureService {
            screens: Vec::new(),
            screen_cache: HashMap::new(),
        };

        let result = service.get_primary_screen();
        assert!(result.is_err());
        
        match result.unwrap_err() {
            AppError::ScreenCapture(msg) => {
                assert!(msg.contains("No primary screen found"));
            }
            _ => panic!("Expected ScreenCapture error"),
        }
    }

    #[test]
    fn test_get_screen_info_not_found() {
        let service = CaptureService {
            screens: Vec::new(),
            screen_cache: HashMap::new(),
        };

        let result = service.get_screen_info(0);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            AppError::ScreenCapture(msg) => {
                assert!(msg.contains("Screen info for index 0 not found"));
            }
            _ => panic!("Expected ScreenCapture error"),
        }
    }

    #[test]
    fn test_capture_area_bounds_validation() {
        // Test that CaptureArea properly handles DPI scaling
        let bounds = Rect::from_min_size(Pos2::new(10.0, 20.0), Vec2::new(100.0, 50.0));
        let area = CaptureArea::with_dpi_scaling(bounds, 0, 2.0, 1.5);
        
        let physical = area.physical_bounds();
        assert_eq!(physical.min.x, 20.0); // 10.0 * 2.0
        assert_eq!(physical.min.y, 30.0); // 20.0 * 1.5
        assert_eq!(physical.width(), 200.0); // 100.0 * 2.0
        assert_eq!(physical.height(), 75.0); // 50.0 * 1.5
    }
}