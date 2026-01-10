# 02 調査で分かったこと（Web + Context7）

本セッションで、Phase2/Phase3 の整合性を確認するために短時間の調査を実施した。

## Web 調査（公式/一次情報ベース）
- **JSON Lines**  
  - 1行に1つの有効なJSON値（オブジェクトに限らず `null` も許容）。  
  - 行終端は `\n`。  
  - 参照: jsonlines.org

- **NO_COLOR 仕様**  
  - 環境変数が存在し、かつ空文字でない場合に色を抑止するのが標準。  
  - 参照: no-color.org  
  - 本プロジェクトは「空文字でも無効化」とするため、標準と差分がある点を明記する方針。

- **Rust edition 2024**  
  - Rust 1.85.0 で 2024 edition が安定化。  
  - v1.0 では Rust/Cargo 1.85+ を必須とする。  
  - 参照: Rust 1.85.0 release notes (blog.rust-lang.org)

- **Cargo publish の要件**  
  - `license` と `description` は必須。  
  - `readme` / `repository` / `homepage` は推奨。  
  - 公開前に `cargo publish --dry-run`（または `cargo package`）の実行が推奨。  
  - 参照: Cargo Book (Publishing)

## Context7 での確認
- **Serde**  
  - `#[serde(skip_serializing_if = "Option::is_none")]` は `None` の場合にフィールドを省略する挙動。
- **Cargo**  
  - `rust-version` は Cargo.toml の `[package]` に記載する最小ツールチェーン指定項目。

## 結論（設計への反映）
- JSON の `label` / `symlink_target` は省略で統一（既存実装と整合）。
- `NO_COLOR` は空文字でも無効化（差分を v1.0 仕様に明記）。
- Rust edition 2024 に合わせ `rust-version = 1.85` を設定し、README/docs に要件を記載。
