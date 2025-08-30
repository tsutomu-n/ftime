# ftime — シンプルなファイル時刻ビューア

ファイルの「更新時刻」「作成時刻」「名前」を一覧表示する小さなCLIです。

<p align="left">
  <img src="./media/basic.gif"   alt="ftime: modified/created/name をひと目で" width="600" />
  
</p>

初学者や非ネイティブにも読みやすい設計。わかりやすいエラーメッセージと初心者向けヘルプを備えています。

| 列       | 意味                                                                                         |
|----------|----------------------------------------------------------------------------------------------|
| mark     | 更新フラグです（1文字）。`+` は「作成後に更新があった」ことを示します。色分けが有効な場合は黄色で表示されます。 |
| modified | 最終更新時刻（`MM-DD HH:MM`）                                                                |
| created  | 作成時刻（経過時間で色分け。未対応時は `-`）                                              |
| name     | ファイル/ディレクトリ名（色有効時、拡張子や種別で色分け）                                    |

---

## 必要要件

- Linux + GNU coreutils: `stat`, `date`
- GNU findutils（`-printf`/`-print0` を備えた `find`）と GNU `sort`（`-z` 対応）
- Bash シェル（`#!/usr/bin/env bash`）

---

## インストール（ワンライナー: ダウンロードのみ） – 推奨

リポジトリをクローンする必要はありません。スクリプトをダウンロードして実行権限を付与します。

```bash
mkdir -p ~/.local/bin
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/ftime-list.sh \
  -o ~/.local/bin/ftime
chmod +x ~/.local/bin/ftime

# test
hash -r
ftime --help
```

### アンインストール

```bash
rm ~/.local/bin/ftime
```

---

<details>
  <summary><strong>インストール（リポジトリから） – 任意</strong></summary>

`~/.local/bin` にシンボリックリンクを置いて `ftime` コマンドとして利用します。

1) 任意の場所へクローン

```bash
git clone https://github.com/tsutomu-n/ftime.git
cd ftime   # リポジトリルートへ
```

2) 実行権限を付与

```bash
chmod +x ftime-list.sh
```

3) `~/.local/bin` を PATH に含める（zsh/bash を自動判定。rc が無ければ作成）

```bash
if [ -n "$ZSH_VERSION" ]; then
  rc="${ZDOTDIR:-$HOME}/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
  rc="$HOME/.bashrc"
else
  rc="$HOME/.profile"
fi
mkdir -p "$(dirname "$rc")"
grep -q '\\.local/bin' "$rc" 2>/dev/null || \
  echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$rc"
. "$rc"
```

4) `ftime` というコマンドを作成

```bash
mkdir -p ~/.local/bin
ln -sf "$PWD/ftime-list.sh" ~/.local/bin/ftime
```

5) リフレッシュして確認

```bash
hash -r
ftime --help
```

</details>

**注意**

- シェルが `ftime` を見つけない場合は新しいターミナルを開くか、`source ~/.zshrc` を実行してください。
- このツールは Linux の GNU `stat`/`date` と Bash を必要とします。

---

## 使い方

### クイックスタート

```bash
ftime               # カレントディレクトリを一覧
ftime -a            # 絶対時刻の代わりに相対時間を表示
ftime -s time       # 更新時刻でソート（新しい順）
ftime -R -d 2 md    # 深さ2で再帰し *.md を一覧
ftime --help        # 詳細ヘルプ
ftime --help-short  # 短いヘルプ（3行）
ftime --version     # バージョン表示
```

### 書式

```bash
ftime [DIR] [PATTERN ...]
```

- DIR（任意）: スキャンするディレクトリ。デフォルトはカレントディレクトリです。
- PATTERN（任意・OR条件）:
  - `*` または `?` を含む → そのままグロブとして使用
  - `.` で始まる → `*` を前置（例: `.log` → `*.log`）
  - 上記以外 → 拡張子扱い（例: `md` → `*.md`）

### オプション

- `-a, --age`: 絶対時刻の代わりに相対時間を表示（例: `5m`, `3h`）
- `-s, --sort time|name`: ソートキー（デフォルト: name、`time` は更新時刻）
- `-r, --reverse`: ソート順を反転
- `-R, --recursive`: サブディレクトリを再帰的に走査
- `-d, --max-depth N`: 再帰の深さを N に制限（`-R` が必要）
- `-h, --help`: 詳細ヘルプを表示
- `--help-short`: 短いヘルプを表示
- `-V, --version`: バージョンを表示

注記:
- 互換のため `FTL_RELATIVE` も利用できますが、`-a/--age` の使用を推奨します

### 例（組み合わせ）

```bash
# ツリー全体を再帰（大きくなる可能性あり）
ftime -R

# 深さ3で再帰
ftime -R -d 3

# docs/ 配下を深さ2、*.md のみ
ftime -R -d 2 docs md

# 更新時刻でソートし、1階層だけ再帰
ftime -s time -R -d 1

# 更新時刻の昇順（古い順）でツリー全体
ftime -s time -r -R
```

### よくあるつまずき

- `-d` には数値が必要
  ```bash
  ftime -d          # Error: --max-depth expects a positive integer
  ftime -d -R       # Error: --max-depth expects a positive integer
  ftime -R -d 3     # OK
  ftime -d 3 -R     # OK（オプション順は任意）
  ```

- `-d` を使うときは `-R` も指定
  ```bash
  ftime -d 3        # Error: --max-depth requires --recursive (-R)
  ftime -R -d 3     # OK
  ```

- シェル展開を避けるためにパターンはクォート
  ```bash
  ftime '*.md'      # OK: パターンは ftime がフィルタとして扱う
  ftime *.md        # シェルがファイル名へ展開し、意図通りでない場合あり
  ```

- 深さは起点の DIR 基準
  ```bash
  ftime -R -d 1 docs   # docs/ と直下のみ（孫以降は含まない）
  ```

----

**Notes**
- 優先順位: コマンドラインオプション > 環境変数 > デフォルト

タイムゾーン: デフォルトはマシンのローカル。環境変数 `FTL_TZ` で上書き可（例: `FTL_TZ=Asia/Tokyo ftime md`）。

<details>
  <summary><strong>表示のカスタマイズ（任意）</strong></summary>

## 色

- 端末（TTY）では自動で色付け。
- パイプ/ページャでも `FTL_FORCE_COLOR=1 ftime | less -R` で強制。
- すべての色を無効化: `NO_COLOR=1` または `FTL_NO_COLOR=1`。

### 色付けされるもの
- `modified` と `created` 列は経過時間で色付け
- `name` 列は種別/拡張子で色分け
- `mark` 列は作成後に更新があると `+` を黄色表示（それ以外は空欄）

### 時間ベースの色分け（設定可能）
- アクティブ（デフォルト4h）: 明るい緑
- 最近（デフォルト24h）: デフォルト色
- 古い（7日以上）: グレー
- 時間色付けを無効化: `FTL_NO_TIME_COLOR=1`
- 閾値調整: `FTL_ACTIVE_HOURS=4 FTL_RECENT_HOURS=24`

</details>

<details>
  <summary><strong>環境変数（任意）</strong></summary>

### 使い方（例）

コマンドの前に一時的に付与して実行します。複数同時指定も可能です。

```bash
# タイムゾーンをニューヨークに変更
FTL_TZ=America/New_York ftime

# 「アクティブ」の閾値を1時間に
FTL_ACTIVE_HOURS=1 ftime

# 複数指定
FTL_TZ=UTC FTL_RECENT_HOURS=48 ftime

# 相対時間表示を有効化
FTL_RELATIVE=1 ftime
# オプションでも可
ftime -a
ftime --age
```

### リファレンス
- `FTL_TZ`: タイムゾーン上書き（例: `Asia/Tokyo`）
- `FTL_FORCE_COLOR`: パイプ時も色付けを強制
- `NO_COLOR` / `FTL_NO_COLOR`: すべての色付けを無効化
- `FTL_NO_TIME_COLOR`: 時間ベース色付けのみ無効化
- `FTL_ACTIVE_HOURS`, `FTL_RECENT_HOURS`: 色分けの閾値（時間）
- `FTL_RELATIVE`: 相対時間表示（例: `5m`, `3h`）

</details>

---

## セキュリティ / 制限事項

- 作成時刻はファイルシステム/カーネル/ツールに依存し、`-` となる場合があります。
- ファイル名に制御文字を含む場合があります。ANSI色が解釈される場所へ貼り付ける際は注意してください。
- Linux/GNU 専用。macOS/BSD の `stat`/`date` はオプションが異なります。

---

## ライセンス

MITライセンスです（`LICENSE` ファイルを参照してください）。
