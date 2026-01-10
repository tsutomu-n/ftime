# ftime とは？（概要と使い方）

このドキュメントは、Rust製CLIツール「ftime」の目的と使い方を日本語で簡潔にまとめたものです。仕様の詳細は `docs/SPEC-v1.0.md`、設計は `docs/ARCHITECTURE.md`、試験方針は `docs/TESTPLAN-v1.0.md` を参照してください。

## 目的
- ローカルディレクトリ直下のファイル・ディレクトリを **更新時刻（mtime）** で並べ、最近の作業コンテキストを視覚化する。
- 読み取り専用ツール（ファイルの変更や削除は行わない）。

## 主な特徴
- 深さ1のスキャン（再帰なし）。
- 更新時刻に基づき4つの時間バケットに分類：`<1h` / `今日` / `<7日` / `それ以外`。
- TTY出力時はカラー・アイコン付きでバケット表示、Historyはデフォルト折り畳み（`--all` で展開）。
- パイプ・リダイレクト時はタブ区切りのプレーンテキストで全件出力。
- JSON Lines出力（`--json`、フィールド固定）で機械処理が容易。
- シンボリックリンクはリンク自身のメタデータを用い、`name -> target` として表示。解決できない場合は `<unresolved>`。
- 隠しファイルはデフォルト非表示（`--hidden` で表示）。
- 拡張子ホワイトリスト（`--ext rs,toml` など、大文字小文字無視、ファイルのみ対象）。
- デフォルト除外: `.DS_Store`, `Thumbs.db`（`--hidden` でも除外）。

## インストール
```bash
# GitHub Releases からインストール（推奨）
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/CLI-Tools/main/scripts/install.sh | bash

# Windows (PowerShell)
powershell -ExecutionPolicy Bypass -Command "iwr https://raw.githubusercontent.com/tsutomu-n/CLI-Tools/main/scripts/install.ps1 -UseBasicParsing | iex"

# crates.io からインストール（公開済みの場合）
cargo install ftime
※ 未公開の場合は GitHub Releases またはソースからビルドを利用。

# ソースからビルド
cargo build --release
# 実行ファイルは target/release/ftime
```
要件: Rust/Cargo 1.85+（edition 2024）

## 使い方
```bash
ftime [OPTIONS] [PATH]
```
- `PATH` 省略時はカレントディレクトリを対象。
- パスがファイルだった場合はエラー終了（コード1）。

### オプション
- `--json`            : JSON Lines で出力（path, bucket, mtime, relative_time, is_dir, is_symlink, symlink_target(解決時のみ), label(Fresh時のみ)。symlink_target/labelは該当時のみ出力）。
- `--ext rs,toml`     : 拡張子ホワイトリスト（カンマ区切り・大小無視）。ファイルのみ対象。
- `-a, --all`         : History バケットを展開して表示。
- `-H, --hidden`      : 隠しファイルも含める。
- `-I, --icons`       : Nerd Fontアイコンを表示（`--features icons` ビルド時）。
- `-h, --help` / `-V, --version` : ヘルプ/バージョン表示。

### 環境変数
- `NO_COLOR` : 設定されていればカラー出力を無効化（空文字でも無効扱い）。
- `FTIME_FORCE_TTY` : パイプ出力でもTTYスタイル（バケット・カラー）を強制。

## 出力イメージ
- **TTY**:  
  ```
  🔥 Active Context (< 1h)
    • src/main.rs  12 mins ago
  ...
  💤 History (25 files hidden)
  ```
- **パイプ**: `path/to/file.rs<TAB>3 days ago`

## 時間バケット規則（ローカル時刻基準）
1. `now - mtime < 1h` : Active Context  
2. `mtime >= 今日の00:00` : Today's Session  
3. `now - mtime < 7d` : This Week  
4. それ以外 : History  
未来時刻は Active 扱い。

## 制限・非対応
- 再帰スキャンなし（深さ1固定）。
- Git連携なし（v1.0時点）。
- パフォーマンスのため余計な I/O を抑制（サマリ表示で20件上限）。

## テスト
開発時は以下を実行して整合を確認してください。
```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

## 参考
- 仕様: `docs/SPEC-v1.0.md`
- 設計: `docs/ARCHITECTURE.md`
- CLI契約: `docs/CLI.md`
- 試験計画: `docs/TESTPLAN-v1.0.md`
