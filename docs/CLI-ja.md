# ftime CLI リファレンス（日本語）

`ftime` は 1 フォルダの Context Recovery（作業文脈の再構築）のための read-only CLI です。英語の正本は `CLI.md` です。

## コマンド署名

```text
ftime [PATH] [-a|--all] [--hide-dots] [--no-ignore] [--ext <csv>] [--files-only] [--all-history] [-A|--absolute] [--no-hints] [--plain|--json] [--color <auto|always|never>] [-I|--icons]
```

## オプション一覧

- `-a, --all`: hidden file と hidden directory を両方表示
- `--all-history`: `History` bucket の preview 制限を外す
- `--hide-dots`: hidden entry を全部隠す
- `--no-ignore`: built-in ignore と `~/.ftimeignore` / local `.ftimeignore` を無効化
- `--ext <csv>`: regular file だけを拡張子で絞る
- `--files-only`: regular file だけ表示
- `-A, --absolute`: `time` を `YYYY-MM-DD HH:MM:SS (UTC±HH:MM)` にする
- `--no-hints`: directory の `[child: ...]` hint を消す
- `--plain`: `path<TAB>bucket<TAB>time`
- `--json`: JSON Lines
- `--color <auto|always|never>`: human output の色制御
- `-I, --icons`: icons build で Nerd Font icon を表示
- `--check-update`: 新しい公開版があるか確認
- `--self-update`: 現在の install 先を最新公開版へ更新

## 組み合わせ制約

- `--plain` と `--json` は同時指定不可
- `-a` と `--hide-dots` は同時指定不可
- `--json` は `--absolute`, `--all-history`, `--no-hints`, `--icons`, 明示的 `--color` を受け付けない
- `--plain` は `--all-history`, `--no-hints`, `--icons`, 明示的 `--color` を受け付けない
- `--check-update` / `--self-update` は scan flag や `PATH` と同時指定不可

## 出力契約

- デフォルトは human view
- human row は `<name>  <size>  <time>  <optional-suffix>`
- 列揃えは raw 文字数ではなく Unicode 表示幅ベース
- 長い名前は human view だけ省略表示されるが、`--plain` / `--json` は完全値を保つ
- `--plain` は `path<TAB>bucket<TAB>time`
- `--json` は JSON Lines
- hidden file は既定で見せ、hidden directory は既定で隠す
- `No matching entries`
- `Skipped N unreadable entries`

## 関連文書

- `README-ja.md`
- `USER-GUIDE-ja.md`
- `COMMANDS.md`
- `INSTALL.md`
- `CLI.md`
