# 05 Phase3 計画とスケジュール

## スケジュール（2026-01-10 時点）
- 2026-01-10: Phase3 計画確定・ドキュメント整備開始（完了）
- 2026-01-10〜2026-01-11: v1.0 仕様・互換性・ツールチェーン要件の明文化
- 2026-01-11: crates.io 配布要件の整備（メタデータ + dry-run 検証）
- 2026-01-11〜2026-01-12: v1.0 ドキュメント群の作成/更新（SPEC/ARCHITECTURE/TESTPLAN）
- 2026-01-12: Phase3 完了条件の確認 + リリースノート準備

## Phase3 チェックリスト（最新版）
### 5.2.1 CLI と互換性ポリシー
- [ ] v1.0 でサポートするオプションを明示し `docs/CLI.md` に凍結仕様として記載
- [ ] 重要オプション（`--all`, `--hidden`, `--json`, `--no-color`）の意味と互換性リスクを明文化
- [ ] `NO_COLOR` 空文字でも無効化する方針を v1.0 仕様に明記
- [ ] JSON 出力フィールド（path, bucket, mtime, relative_time, is_dir, is_symlink, symlink_target, label）の v1.0 凍結方針を明記
- [ ] 残タスクのチェックリストを docs に付与
- [ ] 仕様変更の可能性があるものを “Experimental” としてマーク

### 5.2.2 プラットフォーム検証
- [ ] 対象 OS を明示（Linux major distros, macOS 等）
- [ ] 時間バケット境界（DST/ローカル時刻）の挙動を確認
- [ ] TTY 判定（IsTerminal）の挙動差を確認
- [ ] symlink / パーミッションエラーの挙動差を確認

### 5.2.3 ノイズ除外の整理
- [ ] デフォルト ignore の最小セット定義（`.DS_Store`, `Thumbs.db`, 他）
- [ ] `~/.ftimeignore` の扱いと拡張余地の記述整理

### 5.2.4 品質基準の明確化
- [ ] テスト観点の明文化（1h/1d/7d境界、hidden、symlink、TTY/非TTY）
- [ ] CI で `fmt/clippy/test`（`-D warnings`）維持
- [ ] 1000件程度の体感性能確認と受忍ライン記載
- [ ] Rust/Cargo 最低対応バージョンを README/docs に明記（edition 2024 前提）

### 5.2.5 配布とドキュメント
- [ ] `cargo install ftime` で利用可能にする
- [ ] GitHub Releases で Linux/macOS バイナリ提供
- [ ] crates.io メタデータ（homepage/repository/readme 等）を `Cargo.toml` に揃える
- [ ] `cargo publish --dry-run` / `cargo package --list` で公開内容を検証
- [ ] README を v1.0 仕様に更新
- [ ] `docs/SPEC-v1.0.md` 作成
- [ ] `docs/ARCHITECTURE.md` を v1.0 に追従
- [ ] `docs/TESTPLAN-v1.0.md` 作成

### 5.3 Phase3 完了条件
- [ ] `SPEC-v1.0` が確定し互換性重視の体制が整う
- [ ] CLI/出力変更は Major が必要という認識が共有される
- [ ] v1.0.0 リリースノート公開可能な状態
