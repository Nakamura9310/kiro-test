//! キャプチャ機能のデバッグプログラム
//! screenshots crateの実際のAPIを確認するためのプログラム

use screenshots::Screen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Screenshots Crate API デバッグ");
    println!("================================");

    let screens = Screen::all();
    println!("スクリーン数: {}", screens.len());

    if let Some(screen) = screens.first() {
        println!("スクリーン情報:");
        println!("  ID: {}", screen.id);
        println!("  X: {}", screen.x);
        println!("  Y: {}", screen.y);
        println!("  Width: {}", screen.width);
        println!("  Height: {}", screen.height);

        println!("\nキャプチャ試行中...");
        match screen.capture() {
            Some(image) => {
                println!("✅ キャプチャ成功!");
                println!("  画像サイズ: {}x{}", image.width(), image.height());
                println!("  バッファサイズ: {}", image.buffer().len());
                println!("  期待されるサイズ: {}", image.width() * image.height() * 4);
                
                // 最初の数バイトを確認
                let buffer = image.buffer();
                if buffer.len() >= 16 {
                    println!("  最初の16バイト: {:?}", &buffer[0..16]);
                }

                // 実際に画像として保存してみる
                std::fs::create_dir_all("debug")?;
                
                // screenshots::Imageには直接保存機能がないため、スキップ

                // PNGデータとしてデコードして保存
                match image::load_from_memory(buffer) {
                    Ok(dynamic_image) => {
                        println!("✅ PNG デコード成功!");
                        println!("  デコード後サイズ: {}x{}", dynamic_image.width(), dynamic_image.height());
                        
                        if let Err(e) = dynamic_image.save("debug/decoded_save.png") {
                            println!("❌ デコード保存失敗: {}", e);
                        } else {
                            println!("✅ デコード保存成功: debug/decoded_save.png");
                        }
                    }
                    Err(e) => {
                        println!("❌ PNG デコード失敗: {}", e);
                    }
                }
            }
            None => {
                println!("❌ キャプチャ失敗");
            }
        }
    } else {
        println!("❌ スクリーンが見つかりません");
    }

    Ok(())
}