//! Editor application for screenshot editing
//! 
//! This module contains the main editor window that allows users to view
//! and edit captured screenshots with annotation tools.

use eframe::egui;
use egui::{Context, TextureHandle, Vec2};
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
    zoom_level: f32,
    /// Whether the application should close
    should_close: bool,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            source_image: None,
            texture: None,
            annotations: Vec::new(),
            current_tool: Tool::default(),
            zoom_level: 1.0,
            should_close: false,
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
        // Texture will be created in the update loop when we have access to the context
        Ok(())
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
                ui.menu_button("ファイル", |ui| {
                    if ui.button("新規").clicked() {
                        // TODO: Implement new screenshot
                        ui.close_menu();
                    }
                    if ui.button("開く").clicked() {
                        // TODO: Implement open file
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("保存").clicked() {
                        // TODO: Implement save
                        ui.close_menu();
                    }
                    if ui.button("名前を付けて保存").clicked() {
                        // TODO: Implement save as
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("終了").clicked() {
                        self.request_close();
                        ui.close_menu();
                    }
                });

                ui.menu_button("編集", |ui| {
                    if ui.button("元に戻す").clicked() {
                        // TODO: Implement undo
                        ui.close_menu();
                    }
                    if ui.button("やり直し").clicked() {
                        // TODO: Implement redo
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("クリップボードにコピー").clicked() {
                        // TODO: Implement copy to clipboard
                        ui.close_menu();
                    }
                });

                ui.menu_button("ヘルプ", |ui| {
                    if ui.button("バージョン情報").clicked() {
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
            ui.heading("ツール");
            ui.separator();

            // Tool selection buttons
            if ui.selectable_label(matches!(self.current_tool, Tool::Select), "選択").clicked() {
                self.current_tool = Tool::Select;
            }
            if ui.selectable_label(matches!(self.current_tool, Tool::Rectangle), "矩形").clicked() {
                self.current_tool = Tool::Rectangle;
            }
            if ui.selectable_label(matches!(self.current_tool, Tool::Text), "テキスト").clicked() {
                self.current_tool = Tool::Text;
            }

            ui.separator();

            // Zoom controls
            ui.heading("表示");
            ui.horizontal(|ui| {
                if ui.button("拡大").clicked() {
                    self.zoom_level = (self.zoom_level * 1.2).min(5.0);
                }
                if ui.button("縮小").clicked() {
                    self.zoom_level = (self.zoom_level / 1.2).max(0.1);
                }
            });
            if ui.button("実際のサイズ").clicked() {
                self.zoom_level = 1.0;
            }
            ui.label(format!("ズーム: {:.0}%", self.zoom_level * 100.0));
        });
    }

    /// Draw the main canvas area
    fn draw_canvas(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Ensure texture is created
            self.ensure_texture(ctx);

            if let Some(ref texture) = self.texture {
                // Calculate display size with zoom
                let original_size = texture.size_vec2();
                let display_size = original_size * self.zoom_level;

                // Center the image in the available space
                let available_size = ui.available_size();
                let offset = ((available_size - display_size) * 0.5).max(Vec2::ZERO);

                // Draw the image
                let image_rect = egui::Rect::from_min_size(
                    ui.min_rect().min + offset,
                    display_size
                );

                ui.allocate_ui_at_rect(image_rect, |ui| {
                    ui.add(egui::Image::from_texture(texture).fit_to_exact_size(display_size));
                });

                // TODO: Draw annotations over the image
                // TODO: Handle mouse interactions for editing
            } else {
                // Show placeholder when no image is loaded
                ui.centered_and_justified(|ui| {
                    ui.label("スクリーンショットを撮影するか、画像ファイルを開いてください");
                });
            }
        });
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
        assert!(!app.should_close);
    }

    #[test]
    fn test_editor_app_default() {
        let app = EditorApp::default();
        assert!(app.source_image.is_none());
        assert_eq!(app.current_tool, Tool::Select);
        assert_eq!(app.zoom_level, 1.0);
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
    }
}