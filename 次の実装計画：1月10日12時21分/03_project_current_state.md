# 03 プロジェクト現状（仕様・アーキテクチャ・コード）

## プロジェクト概要
- 名称: `ftime`
- 目的: 直近のファイル更新履歴を時間バケットで可視化する読み取り専用CLI。
- スコープ: ローカルFSの深さ1のみ（再帰なし）。

## Cargo（設定）
- `edition = "2024"`
- `rust-version = "1.85"`
- 主要依存: `clap`(derive), `colored`, `chrono`, `is-terminal`, `anyhow`
- JSON出力: `serde` / `serde_json`（feature `json`、デフォルト有効）
- icons: feature `icons`（オプトイン）

## CLI 概要（`src/main.rs`）
- `ftime [OPTIONS] [PATH]`
- PATHがファイルの場合はエラー終了。
- 環境変数:
  - `NO_COLOR`: 設定されていれば色無効（空文字も無効扱い）
  - `FTIME_FORCE_TTY`: 非TTYでもTTYレイアウトを強制
  - `FTIME_IGNORE`: グローバル ignore ファイルパス上書き
- 出力モード:
  - `--json` 指定 → JSON Lines
  - `stdout` が TTY or `FTIME_FORCE_TTY` → TTY 出力
  - その他 → TSV（パイプ）出力

## コアロジック（`src/engine.rs`）
- `scan_dir`:
  - `read_dir` で depth=1 のみ列挙
  - `symlink_metadata` でメタデータ取得
  - `mtime` 取得不可はスキップ
  - symlinkは `read_link` 成功時のみ `symlink_target` を保持
  - hidden フィルタ（`--hidden` なしで除外）
  - ignore ルール（デフォルト / グローバル / ローカル）
  - ext フィルタ（ファイルのみ対象）
- ソート:
  - `mtime` DESC
  - 同一 `mtime` は `name` ASC（安定化）
- `bucketize`:
  - `classify_bucket` による4バケット割当（Active/Today/ThisWeek/History）
  - `label` は `Fresh` のみ、`--no-labels` で無効化

## 時間ロジック（`src/util/time.rs`）
- `classify_bucket`: Active (<1h) / Today / ThisWeek (<7d) / History
- 未来時刻は Active 扱い
- `relative_time`: 60秒未満 `just now`、1分 `1 min ago`、…、7日以上は `YYYY-MM-DD`
- `classify_label`: 5分以内 → `Fresh`

## 出力（`src/view/`）
- `tty.rs`:
  - バケットヘッダ + 20件上限
  - ディレクトリは `/` + 青太字
  - symlink は `name -> target`（未解決は `<unresolved>`）
  - `label` は `Fresh` バッジ
- `text.rs`:
  - TSV 2カラム: `<path>\t<relative_time>`（全件）
  - 色・ヘッダ・バケット無し
- `json.rs`:
  - 1行1オブジェクト
  - フィールド: `path`, `bucket`, `mtime`(RFC3339/UTC), `relative_time`, `is_dir`, `is_symlink`, `symlink_target`, `label`
  - `symlink_target` / `label` は `None` の場合 **省略**

## 仕様上の固定点（v1.0）
- JSONフィールドは凍結（変更はメジャーのみ）
- NO_COLOR は空文字でも無効化（標準との差分として固定）
- CLI/出力フォーマットは v1.0 以降後方互換を維持
