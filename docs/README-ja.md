# ftime（日本語 README）

[English](../README.md) | 日本語 | [中文](README-zh.md)

`ftime` は、次の 1 問に素早く答えるための読み取り専用 CLI です。

> このフォルダで最近何が変わった？

- 読み取り専用
- 深さ1固定
- `Active / Today / This Week / History` の bucket 表示
- デフォルトは人間向け bucket view
- hidden file は既定で見せ、hidden directory は既定で隠す

## 例

```bash
ftime
ftime -a
ftime --all-history
ftime --hide-dots
ftime --files-only --ext rs,toml
ftime --plain
ftime --json | jq -r '.path'
```

## よく使うフラグ

- `-a, --all`: hidden file / hidden directory を両方表示
- `--all-history`: `History` を全展開
- `--hide-dots`: hidden entry を全部隠す
- `--ext`: regular file だけを拡張子で絞る
- `--files-only`: regular file だけ表示
- `--plain`: `path<TAB>bucket<TAB>time`
- `--json`: JSON Lines
- `--no-hints`: `[child: ...]` を消す
- `--color <auto|always|never>`: 色制御

## 位置づけ

`ftime` は Context Recovery（作業文脈の再構築）のための道具です。再帰検索や Git 状態確認の代わりではありません。

## 詳細ドキュメント

- [読み分け案内](ftime-overview-ja.md)
- [使い方ガイド](USER-GUIDE-ja.md)
- [CLI リファレンス](CLI-ja.md)

## アンインストール

### GitHub Releases install

```bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh | bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh | env INSTALL_DIR=/custom/bin bash
```

Windows PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -Command "iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1 -UseBasicParsing | iex"
powershell -ExecutionPolicy Bypass -Command "& ([scriptblock]::Create((iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1 -UseBasicParsing).Content)) -InstallDir 'C:\custom\bin'"
```

custom install 先を使った場合は、macOS / Linux では `INSTALL_DIR`、Windows では `-InstallDir` を再指定します。

### `cargo install` / `cargo install --path .`

```bash
cargo uninstall ftime
```
