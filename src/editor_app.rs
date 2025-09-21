//! Editor application for screenshot editing
//! 
//! This module contains the main editor window that allows users to view
//! and edit captured screenshots with annotation tools.

use eframe::egui;
use egui::{Context, TextureHandle, Vec2, Pos2, Rect, Response, Sense};
use image::DynamicImage;
use crate::{AnnotationItem, Tool, AppResult};

/// Main editor application for screenshot editing
pub struct EditorApp {
    /// The source image being edited
    source_image: Option<DynamicImage>,
    /// Texture handle for displaying the image in egui
    texture: Option<TextureHandle>,
    /// List of annotations on the image
    annotations: Vec<AnnotationItem>,
    /// Currently selected editing tool
    current_tool: Tool,
    /// Current zoom level for the image
    zoom_level: f64,
    /// Pan offset for the image
    pan_offset: Vec2,
    /// Whether the application should close
    should_close: bool,
    /// Whether we're currently panning
    is_panning: bool,
    /// Last mouse position for panning
    last_mouse_pos: Option<Pos2>,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            source_image: None,
            texture: None,
            annotations: Vec::new(),
            current_tool: Tool::default(),
            zoom_level: 1.0,
            pan_offset: Vec2::ZERO,
            should_close: false,
            is_panning: false,
            last_mouse_pos: None,
        }
    }
}

impl EditorApp {
    /// Create a new editor application
    pub fn new() -> Self {
        Self::default()
    }

    /// Load an image into the editor
    pub fn load_image(&mut self, image: DynamicImage) -> AppResult<()> {
        self.source_image = Some(image);
        // Reset view state when loading new image
        self.zoom_level = 1.0;
        self.pan_offset = Vec2::ZERO;
        self.texture = None; // Force texture recreation
        Ok(())
    }

    /// Load a test image for demonstration purposes
    pub fn load_test_image(&mut self) -> AppResult<()> {
        // Create a test image with a gradient pattern
        let width = 400;
        let height = 300;
        let mut img_buffer = image::ImageBuffer::new(width, height);
        
        for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
            let r = (x as f32 / width as f32 * 255.0) as u8;
            let g = (y as f32 / height as f32 * 255.0) as u8;
            let b = ((x + y) as f32 / (width + height) as f32 * 255.0) as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        
        let test_image = DynamicImage::ImageRgb8(img_buffer);
        self.load_image(test_image)
    }

    /// Get the current tool
    pub fn current_tool(&self) -> &Tool {
        &self.current_tool
    }

    /// Set the current tool
    pub fn set_tool(&mut self, tool: Tool) {
        self.current_tool = tool;
    }

    /// Check if the application should close
    pub fn should_close(&self) -> bool {
        self.should_close
    }

    /// Request the application to close
    pub fn request_close(&mut self) {
        self.should_close = true;
    }

    /// Create texture from image if needed
    fn ensure_texture(&mut self, ctx: &Context) {
        if self.texture.is_none() && self.source_image.is_some() {
            if let Some(ref image) = self.source_image {
                let rgba_image = image.to_rgba8();
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.as_flat_samples();
                
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                self.texture = Some(ctx.load_texture("screenshot", color_image, Default::default()));
            }
        }
    }

    /// Draw the main menu bar
    fn draw_menu_bar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Screenshot").clicked() {
                        // TODO: Implement new screenshot
                        ui.close_menu();
                    }
                    if ui.button("Open").clicked() {
                        // TODO: Implement open file
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Save").clicked() {
                        // TODO: Implement save
                        ui.close_menu();
                    }
                    if ui.button("Save As").clicked() {
                        // TODO: Implement save as
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        self.request_close();
                        ui.close_menu();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        // TODO: Implement undo
                        ui.close_menu();
                    }
                    if ui.button("Redo").clicked() {
                        // TODO: Implement redo
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Copy to Clipboard").clicked() {
                        // TODO: Implement copy to clipboard
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        // TODO: Implement about dialog
                        ui.close_menu();
                    }
                });
            });
        });
    }

    /// Draw the tool panel
    fn draw_tool_panel(&mut self, ctx: &Context) {
        egui::SidePanel::left("tool_panel").show(ctx, |ui| {
            ui.heading("Tools");
            ui.separator();

            // Tool selection buttons
            if ui.selectable_label(matches!(self.current_tool, Tool::Select), "Select").clicked() {
                self.current_tool = Tool::Select;
            }
            if ui.selectable_label(matches!(self.current_tool, Tool::Rectangle), "Rectangle").clicked() {
                self.current_tool = Tool::Rectangle;
            }
            if ui.selectable_label(matches!(self.current_tool, Tool::Text), "Text").clicked() {
                self.current_tool = Tool::Text;
            }

            ui.separator();

            // Zoom controls
            ui.heading("View");
            ui.horizontal(|ui| {
                if ui.button("Zoom In").clicked() {
                    self.zoom_level = (self.zoom_level * 1.2).min(10.0);
                }
                if ui.button("Zoom Out").clicked() {
                    self.zoom_level = (self.zoom_level / 1.2).max(0.1);
                }
            });
            
            // Zoom slider
            ui.add(egui::Slider::new(&mut self.zoom_level, 0.1..=10.0)
                .text("Zoom")
                .suffix("%")
                .custom_formatter(|n, _| format!("{:.0}", n * 100.0))
                .custom_parser(|s| s.parse::<f64>().map(|n| n / 100.0).ok()));
            
            if ui.button("Actual Size").clicked() {
                self.zoom_level = 1.0;
            }
            if ui.button("Fit to Screen").clicked() {
                if let Some(ref texture) = self.texture {
                    // Calculate zoom to fit the image in the available space
                    let image_size = texture.size_vec2();
                    let available_size = Vec2::new(800.0, 600.0); // Approximate canvas size
                    let zoom_x = available_size.x as f64 / image_size.x as f64;
                    let zoom_y = available_size.y as f64 / image_size.y as f64;
                    self.zoom_level = zoom_x.min(zoom_y).min(1.0); // Don't zoom in beyond 100%
                    self.pan_offset = Vec2::ZERO; // Center the image
                }
            }
            if ui.button("Reset View").clicked() {
                self.zoom_level = 1.0;
                self.pan_offset = Vec2::ZERO;
            }
            
            ui.separator();
            
            // Test image button
            if ui.button("Load Test Image").clicked() {
                if let Err(e) = self.load_test_image() {
                    log::error!("Failed to load test image: {}", e);
                }
            }
            
            ui.separator();
            ui.label(format!("Zoom: {:.0}%", self.zoom_level * 100.0));
            if self.pan_offset != Vec2::ZERO {
                ui.label(format!("Pan: ({:.0}, {:.0})", self.pan_offset.x, self.pan_offset.y));
            }
        });
    }

    /// Draw the main canvas area
    fn draw_canvas(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Ensure texture is created
            self.ensure_texture(ctx);

            // Clone the texture handle to avoid borrowing issues
            if let Some(texture) = self.texture.clone() {
                self.draw_image_with_controls(ui, &texture);
            } else {
                // Show placeholder when no image is loaded
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("Take a screenshot or open an image file");
                        ui.separator();
                        ui.label("Or click 'Load Test Image' button in the left panel");
                    });
                });
            }
        });
    }

    /// Draw the image with zoom and pan controls
    fn draw_image_with_controls(&mut self, ui: &mut egui::Ui, texture: &TextureHandle) {
        let available_rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(available_rect, Sense::click_and_drag());

        // Handle mouse interactions
        self.handle_mouse_interactions(&response, available_rect);

        // Calculate image display parameters
        let original_size = texture.size_vec2();
        let display_size = original_size * self.zoom_level as f32;
        
        // Calculate image position with pan offset
        let center_offset = (available_rect.size() - display_size) * 0.5;
        let image_pos = available_rect.min + center_offset + self.pan_offset;
        let image_rect = Rect::from_min_size(image_pos, display_size);

        // Clip the drawing to the available area
        ui.allocate_ui_at_rect(available_rect, |ui| {
            // Set clipping rectangle to prevent drawing outside the canvas area
            ui.set_clip_rect(available_rect);
            
            // Draw background
            ui.painter().rect_filled(
                available_rect,
                0.0,
                ui.style().visuals.extreme_bg_color,
            );

            // Calculate the visible portion of the image that intersects with available area
            let visible_image_rect = image_rect.intersect(available_rect);
            
            // Draw the image only if it's visible
            if visible_image_rect.width() > 0.0 && visible_image_rect.height() > 0.0 {
                // Calculate UV coordinates for the visible portion
                let uv_rect = if image_rect.width() > 0.0 && image_rect.height() > 0.0 {
                    let left = ((visible_image_rect.min.x - image_rect.min.x) / image_rect.width()).max(0.0);
                    let top = ((visible_image_rect.min.y - image_rect.min.y) / image_rect.height()).max(0.0);
                    let right = ((visible_image_rect.max.x - image_rect.min.x) / image_rect.width()).min(1.0);
                    let bottom = ((visible_image_rect.max.y - image_rect.min.y) / image_rect.height()).min(1.0);
                    
                    Rect::from_min_max(
                        Pos2::new(left, top),
                        Pos2::new(right, bottom)
                    )
                } else {
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0))
                };

                ui.painter().image(
                    texture.id(),
                    visible_image_rect,
                    uv_rect,
                    egui::Color32::WHITE,
                );
            }

            // Draw image border (only the visible part)
            if visible_image_rect.width() > 0.0 && visible_image_rect.height() > 0.0 {
                ui.painter().rect_stroke(
                    visible_image_rect,
                    0.0,
                    egui::Stroke::new(1.0, ui.style().visuals.widgets.inactive.bg_stroke.color),
                );
            }

            // Draw annotations (they will be clipped automatically)
            self.draw_annotations(ui, image_rect);

            // Show zoom and pan info overlay
            self.draw_info_overlay(ui, available_rect);
        });
    }

    /// Handle mouse interactions for panning and zooming
    fn handle_mouse_interactions(&mut self, response: &Response, available_rect: Rect) {
        // Handle scroll wheel for zooming
        if response.hovered() {
            let scroll_delta = response.ctx.input(|i| i.scroll_delta.y);
            if scroll_delta != 0.0 {
                let zoom_factor = 1.0 + scroll_delta * 0.001;
                let old_zoom = self.zoom_level;
                self.zoom_level = (self.zoom_level * zoom_factor as f64).clamp(0.1, 10.0);
                
                // Adjust pan offset to zoom towards mouse cursor
                if let Some(mouse_pos) = response.hover_pos() {
                    let relative_pos = mouse_pos - available_rect.center();
                    let zoom_change = (self.zoom_level / old_zoom - 1.0) as f32;
                    self.pan_offset -= relative_pos * zoom_change;
                }
            }
        }

        // Handle middle mouse button or right mouse button for panning
        if response.dragged_by(egui::PointerButton::Middle) || 
           (response.dragged_by(egui::PointerButton::Primary) && 
            response.ctx.input(|i| i.modifiers.shift)) {
            
            let delta = response.drag_delta();
            let new_pan_offset = self.pan_offset + delta;
            
            // Apply pan limits to prevent the image from going completely off-screen
            self.pan_offset = self.constrain_pan_offset(new_pan_offset, available_rect);
        }

        // Handle double-click to reset zoom and pan
        if response.double_clicked() {
            self.zoom_level = 1.0;
            self.pan_offset = Vec2::ZERO;
        }
    }

    /// Draw annotations over the image
    fn draw_annotations(&self, ui: &mut egui::Ui, image_rect: Rect) {
        for annotation in &self.annotations {
            let annotation_pos = image_rect.min + annotation.position.to_vec2() * self.zoom_level as f32;
            
            match &annotation.annotation_type {
                crate::AnnotationType::Rectangle { size, stroke_color, stroke_width } => {
                    let rect_size = *size * self.zoom_level as f32;
                    let rect = Rect::from_min_size(annotation_pos, rect_size);
                    
                    ui.painter().rect_stroke(
                        rect,
                        0.0,
                        egui::Stroke::new(*stroke_width, *stroke_color),
                    );
                    
                    // Draw selection handles if selected
                    if annotation.is_selected {
                        self.draw_selection_handles(ui, rect);
                    }
                }
                crate::AnnotationType::Text { content, font_size, color } => {
                    let scaled_font_size = font_size * self.zoom_level as f32;
                    ui.painter().text(
                        annotation_pos,
                        egui::Align2::LEFT_TOP,
                        content,
                        egui::FontId::proportional(scaled_font_size),
                        *color,
                    );
                }
            }
        }
    }

    /// Draw selection handles around a rectangle
    fn draw_selection_handles(&self, ui: &mut egui::Ui, rect: Rect) {
        let handle_size = 6.0;
        let handle_color = egui::Color32::BLUE;
        
        let corners = [
            rect.min,
            Pos2::new(rect.max.x, rect.min.y),
            rect.max,
            Pos2::new(rect.min.x, rect.max.y),
        ];
        
        for corner in corners {
            let handle_rect = Rect::from_center_size(corner, Vec2::splat(handle_size));
            ui.painter().rect_filled(handle_rect, 2.0, handle_color);
            ui.painter().rect_stroke(handle_rect, 2.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
        }
    }

    /// Constrain pan offset to keep at least part of the image visible
    fn constrain_pan_offset(&self, pan_offset: Vec2, available_rect: Rect) -> Vec2 {
        if let Some(ref texture) = self.texture {
            let original_size = texture.size_vec2();
            let display_size = original_size * self.zoom_level as f32;
            
            // Calculate the bounds for the pan offset
            let min_visible_size = 50.0; // Keep at least 50 pixels of the image visible
            
            let max_pan_x = (available_rect.width() - min_visible_size).max(0.0);
            let min_pan_x = -(display_size.x - min_visible_size).max(0.0);
            
            let max_pan_y = (available_rect.height() - min_visible_size).max(0.0);
            let min_pan_y = -(display_size.y - min_visible_size).max(0.0);
            
            Vec2::new(
                pan_offset.x.clamp(min_pan_x, max_pan_x),
                pan_offset.y.clamp(min_pan_y, max_pan_y)
            )
        } else {
            pan_offset
        }
    }

    /// Draw info overlay showing zoom and pan information
    fn draw_info_overlay(&self, ui: &mut egui::Ui, available_rect: Rect) {
        if self.zoom_level != 1.0 || self.pan_offset != Vec2::ZERO {
            let overlay_pos = available_rect.min + Vec2::new(10.0, 10.0);
            let info_text = format!(
                "Zoom: {:.0}%{}",
                self.zoom_level * 100.0,
                if self.pan_offset != Vec2::ZERO {
                    format!(" | Pan: ({:.0}, {:.0})", self.pan_offset.x, self.pan_offset.y)
                } else {
                    String::new()
                }
            );
            
            // Draw background
            let text_size = ui.painter().layout_no_wrap(
                info_text.clone(),
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            ).size();
            
            let bg_rect = Rect::from_min_size(
                overlay_pos,
                text_size + Vec2::splat(8.0),
            );
            
            ui.painter().rect_filled(
                bg_rect,
                4.0,
                egui::Color32::from_black_alpha(180),
            );
            
            // Draw text
            ui.painter().text(
                overlay_pos + Vec2::splat(4.0),
                egui::Align2::LEFT_TOP,
                info_text,
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            );
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Handle close request
        if self.should_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // Draw UI components
        self.draw_menu_bar(ctx);
        self.draw_tool_panel(ctx);
        self.draw_canvas(ctx);

        // Request repaint for smooth interaction
        ctx.request_repaint();
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_app_creation() {
        let app = EditorApp::new();
        assert!(app.source_image.is_none());
        assert!(app.texture.is_none());
        assert!(app.annotations.is_empty());
        assert_eq!(app.current_tool, Tool::Select);
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, Vec2::ZERO);
        assert!(!app.should_close);
        assert!(!app.is_panning);
        assert!(app.last_mouse_pos.is_none());
    }

    #[test]
    fn test_editor_app_default() {
        let app = EditorApp::default();
        assert!(app.source_image.is_none());
        assert_eq!(app.current_tool, Tool::Select);
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, Vec2::ZERO);
    }

    #[test]
    fn test_tool_management() {
        let mut app = EditorApp::new();
        
        // Test initial tool
        assert_eq!(app.current_tool(), &Tool::Select);
        
        // Test setting tools
        app.set_tool(Tool::Rectangle);
        assert_eq!(app.current_tool(), &Tool::Rectangle);
        
        app.set_tool(Tool::Text);
        assert_eq!(app.current_tool(), &Tool::Text);
    }

    #[test]
    fn test_close_functionality() {
        let mut app = EditorApp::new();
        
        // Initially should not close
        assert!(!app.should_close());
        
        // Request close
        app.request_close();
        assert!(app.should_close());
    }

    #[test]
    fn test_load_image() {
        let mut app = EditorApp::new();
        
        // Create a test image
        let test_image = DynamicImage::new_rgb8(100, 100);
        
        // Load the image
        let result = app.load_image(test_image);
        assert!(result.is_ok());
        assert!(app.source_image.is_some());
        
        // Check that view state is reset
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, Vec2::ZERO);
    }

    #[test]
    fn test_load_test_image() {
        let mut app = EditorApp::new();
        
        // Load test image
        let result = app.load_test_image();
        assert!(result.is_ok());
        assert!(app.source_image.is_some());
        
        // Verify the test image has expected dimensions
        if let Some(ref image) = app.source_image {
            assert_eq!(image.width(), 400);
            assert_eq!(image.height(), 300);
        }
    }

    #[test]
    fn test_zoom_and_pan_state() {
        let mut app = EditorApp::new();
        
        // Test initial state
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, Vec2::ZERO);
        
        // Modify zoom and pan (simulating user interaction)
        app.zoom_level = 2.0;
        app.pan_offset = Vec2::new(10.0, 20.0);
        
        // Load new image should reset view state
        let test_image = DynamicImage::new_rgb8(100, 100);
        let result = app.load_image(test_image);
        assert!(result.is_ok());
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, Vec2::ZERO);
    }
}