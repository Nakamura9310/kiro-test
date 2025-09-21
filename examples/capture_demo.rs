//! キャプチャ機能のデモプログラム
//! 
//! このプログラムは実装したキャプチャ機能を実際に試すためのサンプルです。
//! 
//! 実行方法:
//! cargo run --example capture_demo

use lightweight_screenshot_app::{CaptureService, CaptureArea};
use egui::{Pos2, Rect, Vec2};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🖥️  スクリーンキャプチャ機能デモ");
    println!("================================");

    // CaptureServiceを初期化
    let service = match CaptureService::new() {
        Ok(service) => {
            println!("✅ CaptureService初期化成功");
            service
        }
        Err(e) => {
            println!("❌ CaptureService初期化失敗: {}", e);
            return Err(e.into());
        }
    };

    // 利用可能なスクリーン情報を表示
    println!("\n📺 利用可能なスクリーン:");
    let screens = service.get_screens();
    for screen in &screens {
        println!("  スクリーン {}: {}x{} at ({}, {}) - プライマリ: {}", 
            screen.index,
            screen.bounds.width() as i32,
            screen.bounds.height() as i32,
            screen.bounds.min.x as i32,
            screen.bounds.min.y as i32,
            if screen.is_primary { "はい" } else { "いいえ" }
        );
    }

    // デスクトップ全体の範囲を表示
    let desktop_bounds = service.get_desktop_bounds();
    println!("\n🖥️  デスクトップ全体: {}x{}", 
        desktop_bounds.width() as i32, 
        desktop_bounds.height() as i32
    );

    // 出力ディレクトリを作成
    fs::create_dir_all("screenshots")?;

    // 1. プライマリスクリーンの全画面キャプチャ
    println!("\n📸 1. プライマリスクリーン全画面キャプチャ中...");
    match service.capture_primary_screen() {
        Ok(image) => {
            let filename = "screenshots/primary_screen.png";
            image.save(filename)?;
            println!("✅ 保存完了: {} ({}x{})", filename, image.width(), image.height());
        }
        Err(e) => {
            println!("❌ プライマリスクリーンキャプチャ失敗: {}", e);
        }
    }

    // 2. 指定範囲のキャプチャ（左上角の小さな領域）
    if let Ok(primary) = service.get_primary_screen() {
        println!("\n📸 2. 指定範囲キャプチャ中（左上角 200x150）...");
        
        let capture_bounds = Rect::from_min_size(
            Pos2::new(0.0, 0.0),
            Vec2::new(200.0, 150.0)
        );
        
        let capture_area = CaptureArea::new(capture_bounds, primary.index);
        
        match service.capture_area(&capture_area) {
            Ok(image) => {
                let filename = "screenshots/area_capture.png";
                image.save(filename)?;
                println!("✅ 保存完了: {} ({}x{})", filename, image.width(), image.height());
            }
            Err(e) => {
                println!("❌ 範囲キャプチャ失敗: {}", e);
            }
        }
    }

    // 3. 複数スクリーンがある場合、各スクリーンをキャプチャ
    if screens.len() > 1 {
        println!("\n📸 3. 複数スクリーンキャプチャ中...");
        for (i, _screen) in screens.iter().enumerate() {
            match service.capture_screen_by_index(i) {
                Ok(image) => {
                    let filename = format!("screenshots/screen_{}.png", i);
                    image.save(&filename)?;
                    println!("✅ 保存完了: {} ({}x{})", filename, image.width(), image.height());
                }
                Err(e) => {
                    println!("❌ スクリーン{}キャプチャ失敗: {}", i, e);
                }
            }
        }
    }

    // 4. 中央部分の小さなキャプチャ（デモ用）
    if let Ok(primary) = service.get_primary_screen() {
        println!("\n📸 4. 中央部分キャプチャ中（300x200）...");
        
        let center_x = primary.bounds.width() / 2.0 - 150.0;
        let center_y = primary.bounds.height() / 2.0 - 100.0;
        
        let capture_bounds = Rect::from_min_size(
            Pos2::new(center_x, center_y),
            Vec2::new(300.0, 200.0)
        );
        
        let capture_area = CaptureArea::new(capture_bounds, primary.index);
        
        match service.capture_area(&capture_area) {
            Ok(image) => {
                let filename = "screenshots/center_capture.png";
                image.save(filename)?;
                println!("✅ 保存完了: {} ({}x{})", filename, image.width(), image.height());
            }
            Err(e) => {
                println!("❌ 中央部分キャプチャ失敗: {}", e);
            }
        }
    }

    println!("\n🎉 キャプチャデモ完了！");
    println!("📁 保存先: screenshots/ フォルダ");
    println!("💡 ヒント: 画像ビューアーで結果を確認してください");

    Ok(())
}