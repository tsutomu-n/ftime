# 04 本セッションで行った変更とテスト

## 変更概要（主な内容）
- mtime同値時の `name` 昇順 tie-break を実装（`src/engine.rs`）
- JSON 出力の `label` / `symlink_target` は **該当時のみ出力（省略）** に統一
- `NO_COLOR` は空文字でも無効化と明文化（標準との差分を仕様として固定）
- Rust edition 2024 / MSRV 1.85 を明記し、`rust-version` を追加
- v1.0 仕様/テスト計画ドキュメントを新規作成
- Phase2 完了宣言と Phase3 計画補強をロードマップに反映

## 変更ファイル（作成/更新）
**作成**
- `docs/SPEC-v1.0.md`
- `docs/TESTPLAN-v1.0.md`

**更新**
- `src/engine.rs`
- `Cargo.toml`
- `README.md`
- `docs/README-ja.md`
- `docs/CLI.md`
- `docs/CLI-ja.md`
- `docs/SPEC-ja.md`
- `docs/TESTPLAN-v0.1.md`
- `docs/TESTPLAN-ja.md`
- `docs/ARCHITECTURE.md`
- `docs/ARCHITECTURE-ja.md`
- `docs/ftime-overview-ja.md`
- `docs/12-10_ROADMAP.md`

## 実行したテスト
- `cargo test`
  - unit tests: 5 passed
  - integration tests (cli): 11 passed

## 仕様・ドキュメント整合に関する決定
- JSON: `label` / `symlink_target` は **None の場合は省略**（null固定ではない）
- `NO_COLOR`: 空文字でも無効化（no-color.org 標準と異なるが仕様として固定）
