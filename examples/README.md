# Examples

このディレクトリには、実装した機能の動作確認用サンプルプログラムが含まれています。

## 📁 ファイル構成

### Task 3: スクリーンキャプチャ機能

- **`capture_demo.rs`** - キャプチャ機能のメインデモ
  - プライマリスクリーン全画面キャプチャ
  - 指定範囲キャプチャ（左上角、中央部分）
  - マルチモニター対応
  - 実行: `cargo run --example capture_demo`

- **`debug_capture.rs`** - screenshots crateのAPI確認用
  - スクリーン情報の詳細表示
  - PNGデータのデコード確認
  - デバッグ情報出力
  - 実行: `cargo run --example debug_capture`

## 🚀 実行方法

```bash
# キャプチャ機能のデモ実行
cargo run --example capture_demo

# デバッグ情報確認
cargo run --example debug_capture

# すべてのexampleを一覧表示
cargo run --example
```

## 📸 出力ファイル

- `screenshots/` - capture_demoの出力画像
- `debug/` - debug_captureの出力画像

## 📋 今後の追加予定

- Task 4: GUI基本構造のデモ
- Task 5: 画像表示機能のデモ  
- Task 6: ホットキー機能のデモ
- Task 7-8: オーバーレイ・範囲選択のデモ
- Task 9以降: 統合機能のデモ

各タスク完了時に対応するexampleを追加していきます。