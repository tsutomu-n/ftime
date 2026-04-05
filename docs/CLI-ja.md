# ftime CLI リファレンス（日本語）

`ftime` は 1 フォルダの Context Recovery（作業文脈の再構築）のための read-only CLI です。英語の正本は `CLI.md` です。

## コマンド署名

```text
ftime [PATH] [-a|--all] [--hide-dots] [--no-ignore] [--ext <csv>] [--files-only] [--all-history] [-A|--absolute] [--no-hints] [--plain|--json] [--color <auto|always|never>] [-I|--icons]
```

## オプション一覧

- `-a, --all`
- `--all-history`
- `--hide-dots`
- `--no-ignore`
- `--ext`
- `--files-only`
- `-A, --absolute`
- `--no-hints`
- `--plain`
- `--json`
- `--color <auto|always|never>`
- `-I, --icons`

## 出力契約

- デフォルトは human view
- `--plain` は `path<TAB>bucket<TAB>time`
- `--json` は JSON Lines
- hidden file は既定で見せ、hidden directory は既定で隠す
- `No matching entries`
- `Skipped N unreadable entries`

## 関連文書

- `README-ja.md`
- `USER-GUIDE-ja.md`
- `CLI.md`
