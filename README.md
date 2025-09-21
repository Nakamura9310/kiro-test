# kiroの依頼スクリプト

```
Windows PC用の軽量なスクリーンショットアプリを作りたい

# 機能イメージ

- ショートカットでスクショアプリ起動→マウスで範囲選択するようの画面になる
- マウスでクリック+ドラッグした範囲の画像を取得
- 選択した範囲の画像が、スクショアプリのウィンドウに表示される
- 表示した画像に対して、GUI上で編集できる
    - 編集：いったん 赤四角で囲む+テキスト追加くらい
- 編集した画像をクリップボードに保存きる
- ローカルに画像保存も可能

# 進め方の依頼

- まず、どの言語を使うのがいいのか C++ ? あたりから選定 メリデメ等考慮して。
    そこまで複雑なことはせず、シンプルに軽量に動くものならOKでｓ
- 機能実装は、一つ一つテストしていきたい
    - テストコードは原則実装。可能かつ有効な範囲で。
    - 機能ごとに、ユーザー(私)に対して実装完了したら動作確認させてください。まず、「ショートカット起動→範囲選択した画像がGUIに表示される」みたいなstepから
- 進め方として、タスクリストを作成して、それを実装するごとに潰しながら進めてください
    - 常にタスクリストは変更+更新されていくイメージ
    - kiroならこのあたり上手にやってくれると期待しています
```

# 軽量スクリーンショットアプリ

Windows PC用の軽量なスクリーンショットアプリケーション

## 概要

このアプリケーションは、ショートカットキーでスクリーンショットを撮影し、GUI上で簡単な編集（矩形描画、テキスト追加）を行い、クリップボードやファイルに保存できる軽量なツールです。

## 機能

- ショートカットキー（Ctrl+Shift+S）でのスクリーンショット撮影
- マウスドラッグによる範囲選択
- 撮影した画像のGUI表示・編集
- 編集機能：矩形描画、テキスト追加
- クリップボードへのコピー
- ローカルファイルへの保存（PNG/JPEG/BMP対応）

## 技術スタック

- **言語**: Rust
- **GUIフレームワーク**: egui + eframe
- **画像処理**: image crate
- **スクリーンキャプチャ**: screenshots crate
- **エラーハンドリング**: thiserror
- **非同期処理**: tokio

## プロジェクト構造

```
src/
├── main.rs           # アプリケーションエントリーポイント
├── lib.rs            # ライブラリルート
├── types.rs          # 基本データ型とエラー定義
├── capture.rs        # スクリーンキャプチャ機能
└── editor_app.rs     # メインGUIアプリケーション
```

## 実行方法

```bash
# 開発版の実行
cargo run

# リリース版のビルド
cargo build --release

# テストの実行
cargo test
```

## `cargo run` の実行フロー

### 1. アプリケーション初期化 (`src/main.rs`)

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. ログシステムの初期化
    env_logger::init();
    
    // 2. アプリケーション設定の読み込み
    let settings = AppSettings::default(); // Ctrl+Shift+S, PNG形式
    
    // 3. eframe（egui）の設定
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])      // 初期ウィンドウサイズ
            .with_min_inner_size([800.0, 600.0])   // 最小サイズ
            .with_title("軽量スクリーンショットアプリ") // ウィンドウタイトル
            .with_icon(load_icon()),                // アプリケーションアイコン
        ..Default::default()
    };
    
    // 4. ネイティブGUIアプリケーションの起動
    eframe::run_native(
        "軽量スクリーンショットアプリ",
        native_options,
        Box::new(|_cc| Box::new(EditorApp::new())), // EditorAppインスタンス作成
    )?;
    
    Ok(())
}
```

### 2. EditorApp の初期化 (`src/editor_app.rs`)

```rust
impl Default for EditorApp {
    fn default() -> Self {
        Self {
            source_image: None,                    // 編集対象画像（未設定）
            texture: None,                         // GPU用テクスチャ（未作成）
            annotations: Vec::new(),               // 注釈リスト（空）
            current_tool: Tool::Select,            // 選択ツールを初期選択
            zoom_level: 1.0,                       // 100%ズーム
            should_close: false,                   // アプリ終了フラグ
        }
    }
}
```

### 3. メインイベントループ (`EditorApp::update`)

eguiフレームワークが60FPSで`update`メソッドを呼び出し：

```rust
fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
    // 1. 終了リクエストの処理
    if self.should_close {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        return;
    }

    // 2. UI描画（毎フレーム実行）
    self.draw_menu_bar(ctx);    // メニューバー（ファイル、編集、ヘルプ）
    self.draw_tool_panel(ctx);  // ツールパネル（選択、矩形、テキスト、ズーム）
    self.draw_canvas(ctx);      // メインキャンバス（画像表示エリア）

    // 3. 再描画リクエスト（スムーズなインタラクション用）
    ctx.request_repaint();
}
```

### 4. UI構成要素

#### メニューバー
- **ファイル**: 新規、開く、保存、名前を付けて保存、終了
- **編集**: 元に戻す、やり直し、クリップボードにコピー
- **ヘルプ**: バージョン情報

#### ツールパネル
- **ツール選択**: 選択、矩形、テキスト
- **表示制御**: 拡大、縮小、実際のサイズ、ズーム表示

#### メインキャンバス
- 画像未読み込み時: 「スクリーンショットを撮影するか、画像ファイルを開いてください」
- 画像読み込み後: 画像表示 + 注釈描画 + マウスインタラクション

### 5. 現在の実装状況

- ✅ **基本GUI構造**: 完了（タスク4）
- ⏳ **画像表示機能**: 未実装
- ⏳ **スクリーンキャプチャ**: 未実装
- ⏳ **編集機能**: 未実装
- ⏳ **保存機能**: 未実装

## 開発進捗

詳細な開発進捗とタスクリストは `.kiro/specs/lightweight-screenshot-app/` を参照してください。