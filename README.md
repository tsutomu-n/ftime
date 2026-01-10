# ftime

```text
  __ _   _                 
 / _| | (_)                
| |_| |_ _ _ __ ___   ___  
|  _| __| | '_ ` _ \ / _ \ 
| | | |_| | | | | | |  __/ 
|_|  \__|_|_| |_| |_|\___| 
```

最近更新したファイルを、**時間バケット**で一気に見渡すCLI（深さ1 / read-only / zero-panic）。

[![release](https://github.com/tsutomu-n/ftime/actions/workflows/release.yml/badge.svg)](https://github.com/tsutomu-n/ftime/actions/workflows/release.yml)

## Features
- mtime降順で4バケット分類: Active (<1h) / Today / This Week (<7d) / History
- TTY: カラー＆バケット表示、Historyはデフォルト折りたたみ（各バケット20件上限）
- Pipe/リダイレクト: タブ区切りで全件出力（ヘッダ・色・アイコンなし）
- JSON Lines: `--json`（1行1JSON、機械処理向け）
- フィルタ: `--ext`（拡張子）/ ignore（`~/.ftimeignore`、`FTIME_IGNORE`、`--no-ignore`）

## Quickstart
```bash
ftime
```

## Install
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

### Build from source
```bash
cargo build --release
./target/release/ftime
```

## Usage
```bash
ftime [OPTIONS] [PATH]
```

主なオプション:
- `-a, --all`   : Historyも展開して表示
- `-H, --hidden`: ドットファイルを含める
- `--json`      : JSON Linesで出力
- `--ext`       : 拡張子ホワイトリスト（例: `--ext rs,toml`）
- `--no-ignore` : デフォルト・ユーザーignoreを無効化

環境変数:
- `NO_COLOR`        : 色を無効化
- `FTIME_FORCE_TTY` : パイプ先でもTTYレイアウトを強制
- `FTIME_IGNORE`    : グローバル ignore のパス上書き

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

## Docs
- 日本語ユーザーガイド: `docs/USER-GUIDE-ja.md`
- CLI詳細: `docs/CLI-ja.md`
- 仕様: `docs/SPEC-ja.md`
- 設計: `docs/ARCHITECTURE-ja.md`

## License
MIT (see `LICENSE`)
