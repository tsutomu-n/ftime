# ftime（日本語 README）

`ftime` は、指定ディレクトリ直下（深さ1）のエントリを更新時刻（mtime）で並べ、時間バケットで俯瞰する **読み取り専用** CLIです。

## できること
- ファイル/ディレクトリ/シンボリックリンクを mtime 降順で一覧
- 4つの時間バケット: Active (<1h) / 今日 / 7日以内 / それ以外（History）
- TTY: バケット表示・色・（任意で）アイコン、Historyはデフォルト折りたたみ（`--all` で展開）
- パイプ/リダイレクト: タブ区切りのプレーンテキストで全件出力（ヘッダ・色・アイコンなし）
- JSON Lines: `--json`（機械処理向け）
- フィルタ: `--ext`（拡張子）/ ignore（後述）

## クイックスタート
```bash
ftime              # カレントディレクトリを対象
ftime /path/to/dir # 指定ディレクトリを対象
```

## インストール
要件: Rust/Cargo 1.85+（edition 2024）

### GitHub Releases（推奨）
```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/install.sh | bash

# Windows (PowerShell)
powershell -ExecutionPolicy Bypass -Command "iwr https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/install.ps1 -UseBasicParsing | iex"
```

### crates.io（公開済みの場合）
```bash
cargo install ftime
```

### ソースからインストール（ビルド＋グローバル化）
```bash
cargo install --path .
ftime --version
```

- インストール先は既定で `~/.cargo/bin`（Windowsは `%USERPROFILE%\\.cargo\\bin`）。
- `ftime` をそのまま実行できるのは、上記ディレクトリが `PATH` に通っている場合です。

### ソースからビルド（成果物だけ欲しい）
ビルドは次のどちらかです。

```bash
# 速い方（タイミング + sccache/リンク高速化を自動で使う）
./scripts/build-release-fast.sh

# 標準
cargo build --release
```

ビルドしただけではグローバル（`ftime` だけで実行）にはなりません。生成物が `target/release/ftime` に置かれるだけで `PATH` に入らないからです。

```bash
./target/release/ftime
```

グローバル化したい場合は次のどちらかが必要です。

```bash
# 公式のインストール方式（推奨）
cargo install --path .

# もしくはシンボリックリンク（Linux/macOS）
ln -s /path/to/ftime/target/release/ftime ~/bin/ftime
```

## 使い方
```bash
ftime [OPTIONS] [PATH]
```

- `PATH` 省略時はカレントディレクトリを対象。
- パスがファイルだった場合はエラー終了（コード1）。

### オプション（よく使うもの）
- `-a, --all`      : History バケットも展開（TTYモード）
- `-H, --hidden`   : ドットファイルを含める
- `--ext rs,toml`  : 拡張子ホワイトリスト（カンマ区切り・大小無視、ファイルのみ）
- `--no-ignore`    : ignore（組み込み＋ユーザー設定）を無効化
- `--no-labels`    : ラベル（例: Fresh）を無効化
- `--json`         : JSON Lines で出力（デフォルトビルド。`--no-default-features` だと使えません）
- `-I, --icons`    : Nerd Font アイコン（`--features icons` ビルド時のみ有効。未対応ビルドでは無害な no-op）

### 環境変数
- `NO_COLOR`        : 色を無効化
- `FTIME_FORCE_TTY` : パイプ先でもTTYレイアウトを強制
- `FTIME_IGNORE`    : グローバル ignore ファイルのパスを上書き（既定: `~/.ftimeignore`）

## ignore ルール
- 組み込みで除外: `.DS_Store`, `Thumbs.db`（`--hidden` でも除外）
- ユーザー ignore:
  - グローバル: `~/.ftimeignore`（または `FTIME_IGNORE`）
  - ローカル: `<PATH>/.ftimeignore`（スキャン対象ディレクトリ直下）
- `--no-ignore` で上記をまとめて無効化

## 出力モード
### TTY（通常）
- バケットごとに表示、History はデフォルト折りたたみ（`--all` で展開）

### パイプ / リダイレクト
- `path<TAB>relative_time` を全件出力（ヘッダ/色/アイコンなし）

### JSON Lines
- 1行1JSON。主なフィールド: `path`, `bucket`, `mtime`, `relative_time`, `is_dir`, `is_symlink`（状況により `symlink_target`, `label`）

## 制限
- 深さ1固定（再帰しない）
- 読み取り専用（ファイルの変更/削除はしない）

## 関連ドキュメント
- ユーザーガイド: `docs/USER-GUIDE-ja.md`
- CLI 詳細: `docs/CLI-ja.md`
- 仕様: `docs/SPEC-ja.md`
- 設計: `docs/ARCHITECTURE-ja.md`
- テスト計画: `docs/TESTPLAN-ja.md`
