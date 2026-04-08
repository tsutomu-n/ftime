# ftime（日本語 README）

[English](../README.md) | 日本語 | [中文](README-zh.md)

`ftime` は、次の 1 問に素早く答えるための読み取り専用 CLI です。

> このフォルダで最近何が変わった？

- 読み取り専用
- 深さ1固定
- `Active / Today / This Week / History` の bucket 表示
- デフォルトは人間向け bucket view
- hidden file は既定で見せ、hidden directory は既定で隠す

## 代表例

```bash
ftime
ftime -a
ftime --all-history
ftime --hide-dots
ftime --files-only --ext rs,toml
ftime --plain
ftime --json | jq -r '.path'
ftime --hints
ftime --check-update
```

## 位置づけ

`ftime` は Context Recovery（作業文脈の再構築）のための道具です。再帰検索や Git 状態確認の代わりではありません。

代表フラグ: `--all-history`, `--hide-dots`, `--plain`, `--hints`

## 詳しく読む

- [使い方ガイド](USER-GUIDE-ja.md)
- [CLI リファレンス](CLI-ja.md)
- [詳しいコマンド比較（English）](COMMANDS.md)
- [Install / Update / Uninstall（English）](INSTALL.md)
