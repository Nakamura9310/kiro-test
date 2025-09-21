# Design Document

## Overview

軽量スクリーンショットアプリケーションは、Rust + eguiを使用してWindows PC向けに開発される。アプリケーションは3つの主要コンポーネントから構成される：ホットキー監視サービス、スクリーンキャプチャオーバーレイ、画像編集ウィンドウ。Rustの所有権システムとeguiの即座描画を活用し、最高の軽量性とパフォーマンスを実現する。

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
├─────────────────────────────────────────────────────────────┤
│  main.rs            │  overlay_app.rs  │  editor_app.rs     │
│  - App Entry Point  │  - Screen Capture│  - Image Editing   │
│  - Event Loop       │  - Area Selection│  - Annotation Tools│
├─────────────────────────────────────────────────────────────┤
│                    Service Layer                            │
├─────────────────────────────────────────────────────────────┤
│  hotkey.rs          │  capture.rs      │  editing.rs        │
│  - Global Hotkeys   │  - Screen Capture│  - Drawing Tools   │
│  - Win32 API        │  - Image Processing│ - Export Functions │
├─────────────────────────────────────────────────────────────┤
│                    Model Layer                              │
├─────────────────────────────────────────────────────────────┤
│  types.rs           │  annotations.rs  │  settings.rs       │
│  - CaptureArea      │  - AnnotationItem│  - AppSettings     │
│  - Screen Info      │  - Position/Style│  - Hotkey Config   │
└─────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### 1. Application Entry Point (main.rs)
アプリケーションの起動とメインフローを管理。

```rust
pub struct App {
    settings_manager: SettingsManager,
}

impl App {
    pub fn new() -> Result<Self, AppError>;
    pub fn run() -> Result<(), AppError>;
    pub fn launch_overlay() -> Result<(), AppError>;
}
```

**実装詳細:**
- 直接オーバーレイアプリケーションを起動
- 設定管理機能の統合
- エラーハンドリングとログ出力
- 将来的なホットキー機能への拡張性を保持

### 2. Capture Module (capture.rs)
スクリーンキャプチャ機能を提供。

```rust
pub struct CaptureService;

impl CaptureService {
    pub fn capture_screen() -> Result<DynamicImage, CaptureError>;
    pub fn capture_area(area: CaptureArea) -> Result<DynamicImage, CaptureError>;
    pub fn get_screens() -> Vec<ScreenInfo>;
}
```

**実装詳細:**
- `screenshots` クレートでスクリーンキャプチャ
- `image` クレートで画像処理
- マルチモニター対応
- 高DPI対応（DPI Awareness設定）

### 3. Overlay App (overlay_app.rs)
透明オーバーレイでの範囲選択UI。

```rust
pub struct OverlayApp {
    selection_start: Option<Pos2>,
    selection_end: Option<Pos2>,
    is_selecting: bool,
    screen_image: Option<TextureHandle>,
}

impl eframe::App for OverlayApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}
```

**実装詳細:**
- egui の透明ウィンドウで全画面オーバーレイ
- マウスドラッグでの矩形選択
- ESCキーでキャンセル機能
- 選択完了時にエディターアプリに遷移

### 4. Editor App (editor_app.rs)
画像編集とアノテーション機能。

```rust
pub struct EditorApp {
    source_image: Option<DynamicImage>,
    texture: Option<TextureHandle>,
    annotations: Vec<AnnotationItem>,
    current_tool: Tool,
    zoom_level: f32,
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}
```

**実装詳細:**
- egui の `Image` ウィジェットで画像表示
- `Painter` でのアノテーション描画
- ズーム・パン機能
- ツールバーでの編集ツール選択

### 5. Editing Module (editing.rs)
画像編集とエクスポート機能。

```rust
pub struct EditingService;

impl EditingService {
    pub fn add_annotations(image: &DynamicImage, annotations: &[AnnotationItem]) -> DynamicImage;
    pub fn copy_to_clipboard(image: &DynamicImage) -> Result<(), ClipboardError>;
    pub fn save_to_file(image: &DynamicImage, path: &Path) -> Result<(), std::io::Error>;
}
```

## Data Models

### CaptureArea
```rust
#[derive(Debug, Clone)]
pub struct CaptureArea {
    pub bounds: Rect,
    pub screen_index: usize,
    pub dpi_scale_x: f32,
    pub dpi_scale_y: f32,
}
```

### AnnotationItem
```rust
use uuid::Uuid;
use egui::{Pos2, Vec2, Color32};

#[derive(Debug, Clone)]
pub struct AnnotationItem {
    pub id: Uuid,
    pub position: Pos2,
    pub is_selected: bool,
    pub annotation_type: AnnotationType,
}

#[derive(Debug, Clone)]
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
```

### AppSettings
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub hotkey_modifiers: u32,
    pub hotkey_vk_code: u32,
    pub default_save_directory: Option<String>,
    pub default_image_format: ImageFormat,
}

impl AppSettings {
    pub fn load() -> Result<Self, AppError>;
    pub fn save(&self) -> Result<(), AppError>;
    pub fn get_hotkey_display_string(&self) -> String;
    pub fn set_hotkey(&mut self, modifiers: u32, vk_code: u32);
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            hotkey_modifiers: 0x0002 | 0x0004, // MOD_CONTROL | MOD_SHIFT
            hotkey_vk_code: 0x53, // 'S' key
            default_save_directory: None,
            default_image_format: ImageFormat::Png,
        }
    }
}
```

### Settings Module (settings.rs)
アプリケーション設定の管理を担当。

```rust
pub struct SettingsManager {
    settings: AppSettings,
    settings_path: PathBuf,
}

impl SettingsManager {
    pub fn new() -> Result<Self, AppError>;
    pub fn get_settings(&self) -> &AppSettings;
    pub fn update_save_directory(&mut self, directory: Option<String>) -> Result<(), AppError>;
    pub fn update_image_format(&mut self, format: ImageFormat) -> Result<(), AppError>;
    pub fn save(&self) -> Result<(), AppError>;
    pub fn reset_to_defaults(&mut self) -> Result<(), AppError>;
}
```

### Hotkey Module (hotkey.rs) - Future Enhancement
グローバルホットキーの監視と管理（将来実装予定）。

**Note:** ホットキー機能は複雑なシステム統合が必要なため、コア機能完成後に実装予定。

## Error Handling

### Error Types
```rust
use thiserror::Error;

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
}
```

### Error Handling Strategy
1. **ユーザー通知**: 重要なエラーはegui のモーダルダイアログで通知
2. **ログ出力**: `log` クレートで全てのエラーをファイルログに記録
3. **グレースフル・デグラデーション**: `Result` 型で部分的な機能失敗でもアプリ継続
4. **リトライ機能**: 一時的なエラーに対する自動リトライ

### 具体的な処理
```rust
match capture_service.capture_area(selected_area) {
    Ok(image) => {
        self.load_image(image);
    }
    Err(AppError::ScreenCapture(msg)) => {
        log::error!("Screen capture failed: {}", msg);
        self.show_error_dialog("スクリーンキャプチャに失敗しました。もう一度お試しください。");
    }
    Err(e) => {
        log::error!("Unexpected error: {}", e);
        self.show_error_dialog(&format!("予期しないエラーが発生しました: {}", e));
    }
}
```

## Testing Strategy

### Unit Testing
- **フレームワーク**: Rust標準の `#[cfg(test)]` と `cargo test`
- **モッキング**: `mockall` クレート
- **対象**: Service層とModel層の全モジュール
- **カバレッジ目標**: 80%以上（`cargo tarpaulin` で測定）

### Integration Testing
- **画像比較テスト**: `image-compare` クレートでテスト用画像との比較
- **ファイルI/Oテスト**: `tempfile` クレートで一時ディレクトリでのテスト
- **Win32 APIテスト**: テスト環境でのホットキー登録テスト

### Manual Testing
- **ホットキーテスト**: 様々なアプリケーション上でのホットキー動作
- **マルチモニターテスト**: 異なる解像度・DPI環境でのテスト
- **パフォーマンステスト**: `criterion` クレートでベンチマーク測定

### Test Data
```rust
pub mod test_utils {
    use image::{DynamicImage, RgbImage};
    
    pub fn create_test_image(width: u32, height: u32) -> DynamicImage {
        DynamicImage::ImageRgb8(RgbImage::new(width, height))
    }
    
    pub fn create_test_capture_area() -> CaptureArea {
        CaptureArea {
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0)),
            screen_index: 0,
            dpi_scale_x: 1.0,
            dpi_scale_y: 1.0,
        }
    }
    
    pub fn create_test_annotations() -> Vec<AnnotationItem> {
        vec![
            AnnotationItem {
                id: Uuid::new_v4(),
                position: Pos2::new(10.0, 10.0),
                is_selected: false,
                annotation_type: AnnotationType::Rectangle {
                    size: Vec2::new(50.0, 30.0),
                    stroke_color: Color32::RED,
                    stroke_width: 2.0,
                },
            }
        ]
    }
}
```

### Continuous Integration
- **ビルド自動化**: GitHub Actions
- **自動テスト実行**: プルリクエスト時の `cargo test` 実行
- **コードカバレッジ**: `cargo tarpaulin` + Codecov での監視
- **クロスコンパイル**: Windows x64/x86 バイナリの自動生成

## Performance Considerations

### メモリ管理
- Rustの所有権システムによる自動メモリ管理
- `Arc<Mutex<T>>` での安全な共有状態管理
- 大きな画像の場合の `Box<[u8]>` での効率的なメモリ使用
- `Drop` トレイトでのリソース自動解放

### 起動時間最適化
- 遅延初期化（`std::sync::LazyLock`）
- 必要最小限のクレート依存
- LTO（Link Time Optimization）でのバイナリ最適化
- `--release` ビルドでの最適化

### レスポンシブ性
- `tokio` での非同期処理（async/await）
- `std::thread` での重い処理の分離
- `crossbeam-channel` での効率的なメッセージパッシング
- egui の即座描画による滑らかなUI

### バイナリサイズ最適化
```toml
[profile.release]
opt-level = "z"     # 最小サイズ最適化
lto = true          # Link Time Optimization
codegen-units = 1   # 単一コード生成ユニット
panic = "abort"     # パニック時の即座終了
strip = true        # デバッグシンボル削除
```