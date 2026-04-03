# ftime（日本語 README）

[English](../README.md) | 日本語 | [中文](README-zh.md)

`ftime` は、次の1問に素早く答えるための、読み取り専用 CLI です。

> このフォルダで最近何が変わった？

名前は `files by time` の略です。指定ディレクトリ直下だけを走査し、`mtime` の新しい順に並べて、`Active / Today / This Week / History` の時間バケットに分けて表示します。再帰しないので、フォルダ全体の「最近の動き」を短時間で把握できます。

- 読み取り専用: 削除・リネーム・書き込みなし
- 深さ1固定: 今見ているフォルダだけを対象
- 時間バケット: `Active` / `Today` / `This Week` / `History`
- TTY では人間可読サイズを表示し、非 TTY ではプレーンテキスト、`--json` では JSON Lines を使えます

## どんな時に使うか

- `~/Downloads` を整理したい
- `./target` や build 出力を確認したい
- ログフォルダの最近の動きを見たい
- sync フォルダや共有フォルダで「何か変わった？」を確認したい

## 例

```bash
ftime
ftime ~/Downloads
ftime ./target
ftime /var/log/app
ftime --exclude-dots
ftime --json | jq -r '.path'
```

`--json` は 1 行 1 JSON で出るので、スクリプトにもつなげやすいです。

## 出力例

```text
Active
  • Cargo.toml | 2.1 KiB | 12s ago
Today
  • README.md | 8.4 KiB | 2h ago
This Week
  • docs/ | - | 3d ago [child: today]
History
  • target/ | - | 2w ago [child: active]
```

ディレクトリは size 列に `-` を表示します。
ディレクトリ行では、直下の子要素のほうがフォルダ自身より新しい場合に、`[child: active]` / `[child: today]` の補助表示が付くことがあります。これは TTY 専用で、プレーンテキストや JSON Lines には出ません。
この補助表示はあくまでヒントで、親ディレクトリ自身の bucket や並び順はフォルダ自身の `mtime` を基準にしたままです。

## 他のツールとの違い

| ツール | 向いていること | `ftime` との違い |
| --- | --- | --- |
| `ls -lt` | 時刻順の素早い一覧確認 | 時間バケットには分かれない |
| `eza` | metadata を含む豊富な一覧表示 | 時間バケットは built-in ではない |
| `fd` | 再帰検索やフィルタ | 再帰前提の道具 |
| `bat` | ファイル内容の確認 | フォルダ全体の最近の動きは見ない |
| `ftime` | 1フォルダの最近の動きの把握 | バケット + サイズを一目で見られる |

## インストール

### GitHub Releases（推奨）

GitHub Releases 上の最新 installer を取得して、公開済みの最新 release を入れます。未リリースの `main` は入りません。
GitHub Releases installer には Rust は不要です。

```bash
# macOS / Linux
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.sh | bash

# Windows (PowerShell)
powershell -ExecutionPolicy Bypass -Command "iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.ps1 -UseBasicParsing | iex"
```

Windows の既定 install 先は `%LOCALAPPDATA%\Programs\ftime\bin` です。

Windows installer は現状 x86_64 / AMD64 を対象にしています。

### crates.io

crates.io に公開された crate からインストールします。

```bash
cargo install ftime --locked
ftime --version
```

### ソースから入れる

Rust/Cargo 1.92+ が必要です。

```bash
cargo install --path . --force
hash -r
ftime --version
```

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
- `--ext`: コードや設定だけ見たい時向け。ディレクトリは結果から落ちます
- `--json`: 1 行 1 JSON で出力
- `--check-update`: もっと新しい公開版があるかだけ確認
- `--self-update`: 今のインストール先に最新の公開版を上書き更新
- `--no-ignore`: ignore が効きすぎていないか確認するための切り戻し

## アップデート

```bash
ftime --check-update
ftime --self-update
```

よくある表示例:

```text
update available: 1.0.4 -> 1.0.5
ftime updated 1.0.4 -> 1.0.5 in /home/tn/.local/bin
ftime is already up to date at 1.0.4 in /home/tn/.local/bin
ftime now points to 1.0.4 (was 1.0.5) in /home/tn/.local/bin
```

symlink 経由で起動した場合、`ftime --self-update` はその symlink 側のディレクトリを更新します。

手元の binary が `--self-update` 実装前なら、最初の 1 回だけ GitHub Releases の installer を再実行してください。

## アンインストール

### GitHub Releases install

```bash
# macOS / Linux
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh | bash
```

custom な install 先を使った場合は、同じ場所を macOS / Linux では `INSTALL_DIR`、Windows では `-InstallDir` で再指定します。

```bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh | env INSTALL_DIR=/custom/bin bash
```

Windows PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -Command "iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1 -UseBasicParsing | iex"
```

```powershell
powershell -ExecutionPolicy Bypass -Command "& ([scriptblock]::Create((iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1 -UseBasicParsing).Content)) -InstallDir 'C:\custom\bin'"
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
