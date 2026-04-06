# ftime ユーザーガイド

`ftime` の日常的な使い方だけに絞った短いガイドです。細かい契約は `CLI-ja.md` を参照してください。

## 30秒まとめ

- まず `ftime`
- hidden も見たければ `ftime -a`
- hidden を全部消したければ `ftime --hide-dots`
- `History` を全部開くなら `ftime --all-history`
- 機械処理なら `ftime --plain` か `ftime --json`
- `--no-hints` で child hint を消せます
- 公開版の更新確認は `ftime --check-update`

## 典型的な使い方

```bash
ftime
ftime --all-history
ftime --hide-dots
ftime --plain
ftime --json | jq -r '.path'
```

## 使い分け

- 人間が読む: デフォルトの human view
- TSV が欲しい: `--plain`
- ツール連携したい: `--json`
- hidden directory も必要: `ftime -a`
- hidden entry を全部消したい: `ftime --hide-dots`
- 新しい公開版を見たい: `ftime --check-update`

## 次に読む文書

- `README-ja.md`
- `CLI-ja.md`
- `ftime-overview-ja.md`
