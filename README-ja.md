# ftime — シンプルなファイル時刻ビューア

ファイルの「更新時刻」「作成時刻」「名前」を一覧表示する小さなCLIです。

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

## インストール（推奨: クローンして使う）

`~/.local/bin` にシンボリックリンクを置いて `ftime` コマンドとして利用します。

```bash
git clone https://github.com/tsutomu-n/ftime.git
cd ftime
chmod +x ftime-list.sh
mkdir -p ~/.local/bin
ln -sf "$PWD/ftime-list.sh" ~/.local/bin/ftime
hash -r
ftime --help
```

### アンインストール

```bash
rm ~/.local/bin/ftime
```

---

## インストール（ワンライナー: ダウンロードのみ）

```bash
mkdir -p ~/.local/bin
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/ftime-list.sh \
  -o ~/.local/bin/ftime
chmod +x ~/.local/bin/ftime
hash -r
ftime --help
```

> シェルが `ftime` を見つけない場合は新しいターミナルを開くか、`source ~/.zshrc` を実行してください。

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

注意:
- 優先順位は「オプション > 環境変数 > デフォルト」
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

---

## タイムゾーン

- デフォルト: マシンのローカルタイムゾーン
- 上書き: 環境変数 `FTL_TZ` で上書きできます（例: `FTL_TZ=Asia/Tokyo ftime md`）。

## 色

- 端末（TTY）での実行時は、自動で色付けされます。
- パイプやページャ経由で使う場合も、`FTL_FORCE_COLOR=1` を使えば色付けを強制できます（例: `ftime | less -R`）。
- すべての色を無効化する場合: `NO_COLOR=1` または `FTL_NO_COLOR=1` を指定します。

### どこが色付けされるか
- `modified` 列と `created` 列: 経過時間に応じて色が変わります。
- `name` 列: ファイルの種類や拡張子に応じて色分けされます。
- `mark` 列: 作成後に更新があった場合、`+` が黄色で表示されます。

### 時間ベースの色分け（デフォルト / 変更可能）
- **アクティブ**（デフォルト4時間以内）: 明るい緑
- **最近**（デフォルト24時間以内）: デフォルトカラー（色付けなし）
- **古い**（7日以上経過）: グレー
- 時間ベースの色付けを無効化する場合: `FTL_NO_TIME_COLOR=1`
- 閾値を調整する場合: `FTL_ACTIVE_HOURS=4 FTL_RECENT_HOURS=24`

### 環境変数の使い方（例）

`ftime` コマンドの前に変数を指定すると、そのコマンド実行中だけ有効な一時的な設定になります。この設定は永続的ではありません。複数組み合わせることも可能です。

```bash
# タイムゾーンをニューヨークに変更
FTL_TZ=America/New_York ftime

# 「アクティブ」の閾値を1時間に
FTL_ACTIVE_HOURS=1 ftime

# 複数指定
FTL_TZ=UTC FTL_RECENT_HOURS=48 ftime

# 絶対時刻の代わりに相対時間を表示
FTL_RELATIVE=1 ftime
# オプションで有効化
ftime -a
ftime --age
```

 

### 環境変数（リファレンス）
- `FTL_TZ`: タイムゾーンを上書きします（例: `Asia/Tokyo`）。
- `FTL_FORCE_COLOR`: パイプ経由でも色付けを強制します。
- `NO_COLOR` / `FTL_NO_COLOR`: すべての色付けを無効化します。
- `FTL_NO_TIME_COLOR`: 時間ベースの色付けだけを無効化します。
- `FTL_ACTIVE_HOURS`, `FTL_RECENT_HOURS`: 経過時間（アクティブ/最近）の閾値を時間単位で設定します。
- `FTL_RELATIVE`: 絶対時刻の代わりに相対時間を表示します（例: `5m`, `3h`）。

---

## セキュリティ / 制限事項

- 作成時刻はファイルシステム/カーネル/ツールに依存し、`-` となる場合があります。
- ファイル名に制御文字を含む場合があります。ANSI色が解釈される場所へ貼り付ける際は注意してください。
- Linux/GNU 専用。macOS/BSD の `stat`/`date` はオプションが異なります。

---

## ライセンス

MITライセンスです（`LICENSE` ファイルを参照してください）。
