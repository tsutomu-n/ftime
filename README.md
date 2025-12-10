# ftime

Recent-file viewer with time buckets. Depth=1, read-only, zero-panic設計。

## Features
- mtime降順で4バケット分類: Active (<1h) / Today / This Week (<7d) / History。
- TTY: カラー＆バケット表示、Historyはデフォルト折りたたみ（各バケット20件上限）。
- Pipe/リダイレクト: タブ区切りで全件出力（ヘッダ・色・アイコンなし）。
- 隠しファイルはデフォルト非表示、`-H/--hidden` で表示。
- オプトインのNerd Fontアイコン: `--icons`（要 `cargo build --features icons`）。

## Install / Build
```bash
cargo build
# Nerd Fontアイコンを使う場合
cargo build --features icons
```

## Usage
```bash
ftime [OPTIONS] [PATH]
```

主なオプション:
- `-a, --all`   : Historyも展開して表示
- `-H, --hidden`: ドットファイルを含める
- `-I, --icons` : バケット見出しをNerd Fontグリフに（feature iconsビルド時のみ）

環境変数:
- `NO_COLOR`        : 色を無効化（最優先）
- `FTIME_FORCE_TTY` : パイプ先でもTTYレイアウトを強制（色の有無は NO_COLOR に従う）

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

## Notes
- ソート安定性: `mtime` DESC、同値は `name` ASC。
- symlink: TTYでは `name -> target`、Pipeではパスのみ。
- ディレクトリ: TTYでは末尾`/`付き、Pipeではパスのみ。
