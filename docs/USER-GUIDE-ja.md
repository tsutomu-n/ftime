# ftime ユーザーガイド

この文書は、`ftime` を日常作業でどう使い分けるかに絞ったガイドです。インストールは `README-ja.md`、オプションや出力契約の正確な確認は `CLI-ja.md` を参照してください。

## 30秒まとめ

- `ftime` は深さ 1 の最近更新一覧を出す読み取り専用 CLI
- まず `ftime`、古い履歴も見たければ `ftime -a`
- ドットファイルを消したければ `--exclude-dots`
- 機械処理なら `--json` かパイプ出力を使う

## 典型的な使い方

### 1. 作業を再開するとき

```bash
ftime
```

最初に見るべきコマンドです。`Active` と `Today` を中心に、直近で触ったファイルを短時間で思い出せます。`History` は折りたたまれるので、古いノイズを最初から見なくて済みます。

### 2. 昨日以前も含めて俯瞰したいとき

```bash
ftime -a
```

`History` を展開して確認したい場面です。TTY では各バケット 20 件上限のままなので、全件が必要ならパイプ出力に切り替えます。

### 3. dotfiles を一時的に外したいとき

```bash
ftime --exclude-dots
```

`.env`、`.gitignore`、設定ファイルが多いディレクトリでノイズを減らしたいときに向いています。逆に設定ファイルも追いたいなら、何も付けないのが基本です。

### 4. 特定の拡張子だけ見たいとき

```bash
ftime --ext rs,toml
```

コードや設定だけを見たい場面向けです。`--ext` はファイルだけに効き、ディレクトリは結果から落ちます。

### 5. 他のツールに渡したいとき

```bash
ftime | cut -f1
ftime --json | jq '.path'
```

テキスト出力は `path<TAB>time`、`--json` は 1 行 1 JSON です。`fzf`、`jq`、シェルスクリプトにそのまま渡せます。

## 出力の読み方

### TTY 出力

- `Active / Today / This Week / History` の順で出ます
- `History` はデフォルトで折りたたみです
- 行は `name | size | time` の形です
- ディレクトリ行では `[child: active]` / `[child: today]` が付くことがあります
- これはフォルダ自身より新しい直下の子要素があるサインです
- 未来時刻は `Skew` として強調されます
- 最後に `Current Timezone: UTC±HH:MM` が付きます

最初の確認では、`Active` と `Today` だけで十分なことが多いです。`History` を毎回開くより、必要になった時だけ `-a` を付ける運用のほうが速くなります。

### パイプ / リダイレクト出力

- バケット見出しは消えます
- 20 件上限はなくなり、全件出ます
- 形式は `path<TAB>time` です

一覧を加工したいときは、TTY ではなくこちらを基準に考えると扱いやすいです。

### JSON Lines

- `--json` を付けると 1 行 1 JSON になります
- 主なキーは `path`, `bucket`, `mtime`, `relative_time` です
- 人が読むというより、ツール連携向けです
- 機械処理したいときは、child hint が出ないパイプ出力か `--json` を使います。

出力キーの正確な契約は `CLI-ja.md` と `CLI.md` を見てください。

## フィルタの使い分け

### `--exclude-dots`
dotfiles を見たくない時だけ使います。常用するより、必要なときだけ付けるほうが意図が明確です。

### `--ext`
「対象を絞る」ためのフラグです。たとえば Rust と TOML だけ見たい時に有効です。ディレクトリを含めて俯瞰したい場面では向きません。

### ignore ルール
`.DS_Store` と `Thumbs.db` は built-in で除外されます。さらに `~/.ftimeignore`、`<PATH>/.ftimeignore`、`FTIME_IGNORE` を使って自分のノイズを消せます。

### `--no-ignore`
ignore が効きすぎているか確認したいときの切り戻し用です。普段使いではなく、挙動確認のためのフラグとして考えると分かりやすいです。

## よくある組み合わせ

```bash
ftime -a
ftime --exclude-dots --ext rs,toml
NO_COLOR=1 ftime
FTIME_FORCE_TTY=1 ftime
```

- `ftime -a`: 履歴まで含めてざっと把握
- `ftime --exclude-dots --ext rs,toml`: ふだん触るコードだけ確認
- `NO_COLOR=1 ftime`: ログに貼りたいとき
- `FTIME_FORCE_TTY=1 ftime`: TTY 表示の確認やスナップショット用

## 困ったときの見方

### 何も出ない
空ディレクトリか、`--exclude-dots` / `--ext` / ignore で全件落ちている可能性があります。まずはフラグを外して再実行します。

### 未来時刻が出る
`+Ns [Skew]` や `+Nm [Skew]` は、ファイルの mtime が現在より未来にある状態です。システム時計や同期ズレの確認ポイントです。

### パスに対してエラーになる
`ftime` はディレクトリを対象にするツールです。ファイルを直接渡すと終了コード 1 で失敗します。

## 次に読む文書

- 最初の入口に戻る: `README-ja.md`
- フラグや出力契約を確認する: `CLI-ja.md`
- どの文書を読むべきか迷ったら: `ftime-overview-ja.md`
