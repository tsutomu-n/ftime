# ftime（日本語 README）

[English](../README.md) | 日本語 | [中文](README-zh.md)

`ftime` は、指定ディレクトリ直下だけを読み取り専用で走査し、更新時刻 (`mtime`) の新しい順に並べて時間バケットで見せる CLI です。再帰しないので、いま触ったものを短時間で確認したい場面に向いています。

- 深さ 1 固定、読み取り専用
- `Active / Today / This Week / History` の 4 バケット
- 端末では見やすく、パイプや JSON では処理しやすく出力

## インストール

### GitHub Releases（推奨）
`main` 上の最新 installer script を取得し、その script が公開済みの最新 release を入れます。未リリースの `main` は入りません。

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/install.sh | bash

# Windows (PowerShell)
powershell -ExecutionPolicy Bypass -Command "iwr https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/install.ps1 -UseBasicParsing | iex"
```

### ソースから入れる
Rust/Cargo 1.92+ が必要です。

```bash
cargo install --path . --force
hash -r
ftime --version
```

Windows installer は現状 x86_64 / AMD64 を対象にしています。

アンインストール手順は下の `## アンインストール` にまとめています。custom install 先を使った場合の戻し方もそこにあります。

## クイックスタート

```bash
ftime
ftime ~/project
ftime -a
ftime --exclude-dots
ftime --ext rs,toml
ftime --json
```

よく使うフラグ:

- `-a, --all`: TTY で `History` を展開
- `-A, --absolute`: `2026-03-16 20:49:28 (UTC+09:00)` のような絶対時刻で表示
- `--exclude-dots`: ドットファイルを除外
- `--no-ignore`: built-in / `.ftimeignore` を無効化

## アンインストール

### GitHub Releases install

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/uninstall.sh | bash
```

custom な install 先を使った場合は、同じ場所を macOS / Linux では `INSTALL_DIR`、Windows では `-InstallDir` で再指定します。

```bash
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/uninstall.sh | env INSTALL_DIR=/custom/bin bash
```

Windows PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -Command "iwr https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/uninstall.ps1 -UseBasicParsing | iex"
```

```powershell
powershell -ExecutionPolicy Bypass -Command "& ([scriptblock]::Create((iwr https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/uninstall.ps1 -UseBasicParsing).Content)) -InstallDir 'C:\custom\bin'"
```

### `cargo install` / `cargo install --path .`

```bash
cargo uninstall ftime
```

## 詳細ドキュメント

- [読み分け案内](ftime-overview-ja.md)
- [使い方ガイド](USER-GUIDE-ja.md)
- [CLI リファレンス](CLI-ja.md)
- [仕様の正本](CLI.md)

最初に全体像だけ知りたいなら `README-ja.md`、実際の使い方を追いたいなら `USER-GUIDE-ja.md`、フラグや出力契約を確認したいなら `CLI-ja.md` を開いてください。
