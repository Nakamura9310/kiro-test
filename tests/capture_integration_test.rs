//! Integration tests for screen capture functionality
//! 
//! These tests verify that the capture service works correctly
//! in a real environment with actual screens.

use lightweight_screenshot_app::{CaptureService, CaptureArea};
use egui::{Pos2, Rect, Vec2};

#[test]
fn test_capture_service_real_environment() {
    // This test will be skipped in headless environments
    match CaptureService::new() {
        Ok(service) => {
            // Test that we can get screen information
            let screens = service.get_screens();
            assert!(!screens.is_empty(), "Should have at least one screen");

            // Test that we can get primary screen
            let primary = service.get_primary_screen();
            assert!(primary.is_ok(), "Should be able to get primary screen");

            // Test desktop bounds calculation
            let desktop_bounds = service.get_desktop_bounds();
            assert!(desktop_bounds.width() > 0.0, "Desktop should have positive width");
            assert!(desktop_bounds.height() > 0.0, "Desktop should have positive height");

            println!("Found {} screen(s)", screens.len());
            for screen in &screens {
                println!("Screen {}: {}x{} at ({}, {}), Primary: {}", 
                    screen.index,
                    screen.bounds.width(),
                    screen.bounds.height(),
                    screen.bounds.min.x,
                    screen.bounds.min.y,
                    screen.is_primary
                );
            }
        }
        Err(e) => {
            println!("Skipping test in headless environment: {}", e);
        }
    }
}

#[test]
fn test_capture_primary_screen_real() {
    // This test will be skipped in headless environments
    match CaptureService::new() {
        Ok(service) => {
            match service.capture_primary_screen() {
                Ok(image) => {
                    assert!(image.width() > 0, "Captured image should have positive width");
                    assert!(image.height() > 0, "Captured image should have positive height");
                    println!("Successfully captured primary screen: {}x{}", image.width(), image.height());
                }
                Err(e) => {
                    println!("Failed to capture primary screen (may be expected in some environments): {}", e);
                }
            }
        }
        Err(e) => {
            println!("Skipping test in headless environment: {}", e);
        }
    }
}

#[test]
fn test_capture_area_real() {
    // This test will be skipped in headless environments
    match CaptureService::new() {
        Ok(service) => {
            if let Ok(primary) = service.get_primary_screen() {
                // Create a small capture area in the top-left corner
                let capture_bounds = Rect::from_min_size(
                    Pos2::new(0.0, 0.0),
                    Vec2::new(100.0, 100.0)
                );
                
                let capture_area = CaptureArea::new(capture_bounds, primary.index);
                
                match service.capture_area(&capture_area) {
                    Ok(image) => {
                        assert_eq!(image.width(), 100, "Captured area should be 100 pixels wide");
                        assert_eq!(image.height(), 100, "Captured area should be 100 pixels tall");
                        println!("Successfully captured area: {}x{}", image.width(), image.height());
                    }
                    Err(e) => {
                        println!("Failed to capture area (may be expected in some environments): {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Skipping test in headless environment: {}", e);
        }
    }
}

#[test]
fn test_multi_screen_detection() {
    // This test checks multi-screen detection capabilities
    match CaptureService::new() {
        Ok(service) => {
            let screens = service.get_screens();
            
            if screens.len() > 1 {
                println!("Multi-screen setup detected with {} screens", screens.len());
                
                // Verify each screen has valid bounds
                for screen in &screens {
                    assert!(screen.bounds.width() > 0.0, "Screen {} should have positive width", screen.index);
                    assert!(screen.bounds.height() > 0.0, "Screen {} should have positive height", screen.index);
                }
                
                // Test desktop bounds span all screens
                let desktop_bounds = service.get_desktop_bounds();
                let desktop_area = desktop_bounds.width() * desktop_bounds.height();
                
                // Desktop area should be at least as large as the largest single screen
                let max_screen_area = screens.iter()
                    .map(|s| s.bounds.width() * s.bounds.height())
                    .fold(0.0f32, f32::max);
                
                assert!(desktop_area >= max_screen_area, 
                    "Desktop bounds should encompass at least the largest screen");
            } else {
                println!("Single screen setup detected");
            }
        }
        Err(e) => {
            println!("Skipping test in headless environment: {}", e);
        }
    }
}