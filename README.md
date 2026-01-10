# ftime

Recent-file viewer with time buckets. Depth=1, read-only, zero-panic設計。

## Features
- mtime降順で4バケット分類: Active (<1h) / Today / This Week (<7d) / History。
- TTY: カラー＆バケット表示、Historyはデフォルト折りたたみ（各バケット20件上限）。
- Pipe/リダイレクト: タブ区切りで全件出力（ヘッダ・色・アイコンなし）。
- 隠しファイルはデフォルト非表示、`-H/--hidden` で表示。
- オプトインのNerd Fontアイコン: `--icons`（要 `cargo build --features icons`）。
- JSON Lines出力: `--json` で1行1オブジェクト（フィールドは後方互換のため固定: path, bucket, mtime, relative_time, is_dir, is_symlink, symlink_target, label。symlink_target/labelは該当時のみ出力）。
- 拡張子ホワイトリスト: `--ext rs,toml`（カンマ区切り・大小無視・ファイルのみ）
- グローバル ignore: `~/.ftimeignore`（`FTIME_IGNORE` で指定、`--no-ignore` で無効化）

## Requirements
- Rust/Cargo 1.85+（edition 2024）

## Install / Build
```bash
# Install from crates.io (v1.0+)
cargo install ftime

# Build from source
cargo build
# Nerd Fontアイコンを使う場合
cargo build --features icons
# JSONはデフォルト有効（json feature）。無効ビルドは `--no-default-features`。
```

## Usage
```bash
ftime [OPTIONS] [PATH]
```

主なオプション:
- `-a, --all`   : Historyも展開して表示
- `-H, --hidden`: ドットファイルを含める
- `-I, --icons` : バケット見出しをNerd Fontグリフに（feature iconsビルド時のみ）
- `--json`      : JSON Linesで出力（色・アイコン・バケット上限なし）
- `--ext`       : 拡張子ホワイトリスト（カンマ区切り、case-insensitive、ファイルのみ対象）
- `--no-ignore` : デフォルト・ユーザーignoreを無効化

環境変数:
- `NO_COLOR`        : 色を無効化（最優先、空文字でも無効扱い）
- `FTIME_FORCE_TTY` : パイプ先でもTTYレイアウトを強制（色の有無は NO_COLOR に従う）
- `FTIME_IGNORE`    : グローバル ignore ファイルのパスを上書き（デフォルトは `~/.ftimeignore`）

## Output Examples
TTY:
```
🔥 Active Context (< 1h)
  • src/main.rs  12 mins ago
```

Pipe:
```
src/main.rs\t12 mins ago
subdir\t2 hours ago
link_to_file\t3 days ago
```

JSON Lines:
```
{"path":"src/main.rs","bucket":"active","mtime":"2025-12-10T12:00:00Z","relative_time":"just now","is_dir":false,"is_symlink":false}
```

## Performance (参考値)
- 約2,000ファイルでの実測 (devビルド, /dev/null出力): TSV/TTY ~0.06s, JSON ~0.25s。線形に近い挙動を確認。

## Notes
- ソート安定性: `mtime` DESC、同値は `name` ASC。
- symlink: TTYでは `name -> target`、Pipeではパスのみ。
- ディレクトリ: TTYでは末尾`/`付き、Pipeではパスのみ。
- デフォルトで除外: `.DS_Store`, `Thumbs.db`（`--hidden` でも除外）
