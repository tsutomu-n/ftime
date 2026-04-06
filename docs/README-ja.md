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
ftime --check-update
```

## コマンド早見表

既定の `ftime` と比べて何が変わるかを一覧にするとこうなります。

| コマンド | 使いどころ | 変化 |
| --- | --- | --- |
| `ftime` | カレントディレクトリを human view で見る | bucket / size / time / suffix を表示 |
| `ftime [PATH]` | 別ディレクトリを対象にする | 出力形は同じで対象だけ変わる |
| `ftime -a` | hidden directory も見たい | hidden file を残したまま hidden directory を追加 |
| `ftime --hide-dots` | hidden entry を全部消したい | hidden file / dir / symlink を全部隠す |
| `ftime --no-ignore` | ignore 済みのものも見たい | built-in ignore と `.ftimeignore` を無効化 |
| `ftime --ext rs,toml` | 特定拡張子だけに寄せたい | regular file だけを拡張子で絞る |
| `ftime --files-only` | file だけ見たい | directory と symlink を消す |
| `ftime --all-history` | `History` を全部見たい | `History` の preview 制限を外す |
| `ftime -A` | 厳密な時刻を見たい | 相対時刻を絶対時刻へ置き換える |
| `ftime --no-hints` | `[child: ...]` を消したい | directory の child hint suffix を消す |
| `ftime --plain` | スクリプトに渡したい | `path<TAB>bucket<TAB>time` の TSV |
| `ftime --json` | 構造化出力が欲しい | JSON Lines |
| `ftime --check-update` | 新しい公開版があるか見たい | update 可否だけ出す |
| `ftime --self-update` | 今の install 先を更新したい | 最新の公開版へ更新 |
| `ftime --help` | 全オプションを見たい | usage と制約を表示 |
| `ftime --version` | 現在の版を知りたい | version を表示 |

hidden entry の違い:

```text
$ ftime
Today (2)
  .env       312 B   2h
  README.md  8.4 KiB 3h

This Week (1)
  src/           —   2d [child: today]
```

```text
$ ftime -a
Today (2)
  .env       312 B   2h
  README.md  8.4 KiB 3h

This Week (3)
  .git/          —   1d [child: active]
  .cache/        —   1d
  src/           —   2d [child: today]
```

```text
$ ftime --hide-dots
Today (1)
  README.md  8.4 KiB 3h

This Week (1)
  src/           —   2d [child: today]
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
- `--check-update`: 新しい公開版があるか確認
- `--self-update`: 最新の公開版へ更新

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
