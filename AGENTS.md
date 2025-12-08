# Repository Guidelines

## プロジェクト構成と配置
- ドキュメント: `docs/ARCHITECTURE.md`, `docs/CLI.md`, `docs/SPEC-v0.1.md`, `docs/TESTPLAN-v0.1.md` が設計・仕様・試験方針の唯一のソース。変更時はここを先に更新。
- 想定コード配置（Rust CLI `ftime`）: `src/main.rs`（CLI/モード判定）、`src/engine.rs`（スキャン・ソート・フィルタ）、`src/model.rs`（構造体）、`src/view/`（tty/text 出力）、`src/util/time.rs`（相対時間計算）。統合テストは `tests/`、単体テストは各モジュール内 `mod tests` に置く。
- 資産類（例: サンプル出力、基準データ）は `docs/` 配下にまとめる。トップ階層には生成物を置かない。

## ビルド・テスト・開発コマンド
- `cargo build` / `cargo build --release` : 通常ビルドと最適化ビルド。
- `cargo run -- -h` : CLI ヘルプ確認。開発時は任意のパスを付けて実行。
- `cargo fmt` : rustfmt による自動整形（必須）。
- `cargo clippy -- -D warnings` : 警告をエラー扱いで静的解析。
- `cargo test` : 単体・統合テスト一括実行。特定モジュールのみは `cargo test engine::` のようにフィルタ。

## コーディングスタイルと命名
- インデントはスペース4。`rustfmt` のデフォルト設定に準拠。
- パニック禁止を基本とし、`Result` で伝播。`unwrap` / `expect` はテスト以外で使用しない。
- 命名: 関数・変数は `snake_case`、型は `PascalCase`、定数は `SCREAMING_SNAKE_CASE`。モジュールは設計図に合わせて分割し、`view::tty`/`view::text` のように出力経路で明確化。
- 外部依存は設計で指定された `clap`（derive）、`colored`、`chrono`、`is-terminal` に限定し、不要追加を避ける。

## テスト指針
- `docs/TESTPLAN-v0.1.md` を必読。時間バケットの境界（1h, 当日0:00, 7d）の前後値をモック時刻で検証する。
- 統合テストは `tests/` で `tempfile` などを使い実ファイルを生成し、`--hidden`、`--all`、パイプ出力（`cargo run -- -H | cat`）を確認。
- バケット表示の上限20件と履歴折り畳みのカウント文言を必ずカバーする。

## コミット・PR ガイドライン
- 現状 Git 履歴は未整備。Git 運用を開始する場合は短い命令形サブジェクトを推奨（例: `feat: add tty view`）。関連ドキュメントを更新したら同一コミットに含める。
- PR では scope / 目的 / 主要な動作確認（実行コマンドと要約結果）を記載し、`cargo fmt && cargo clippy && cargo test` を通したことを明記。CLI 出力が変わる場合は before/after のサンプルを添付。

## 運用上の注意
- ツールは読み取り専用設計（ファイル削除・上書きは実装しない）。シンボリックリンクは `lstat` ベースで扱い、壊れたリンクでもパニックさせない。
- `NO_COLOR` でカラー無効、`--hidden` や `--all` の挙動は仕様通りに維持する。パフォーマンス要件上、再帰や不要な I/O を入れない。
