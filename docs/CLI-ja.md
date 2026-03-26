# ftime CLI リファレンス（日本語）

この文書は `ftime` の CLI 契約を日本語で引けるようにしたリファレンスです。導入は `README-ja.md`、使い方の流れは `USER-GUIDE-ja.md`、英語の正本は `CLI.md` を参照してください。

## コマンド署名

```text
ftime [OPTIONS] [PATH]
```

- `PATH` を省略すると `.` を対象にします
- `PATH` は 1 つだけ受け取ります
- ファイルを渡した場合はエラー終了します

## オプション一覧

| 短 | 長 | 内容 | 備考 |
| --- | --- | --- | --- |
| `-a` | `--all` | TTY で `History` を展開 | 20 件上限は維持 |
| `-A` | `--absolute` | 時刻を `YYYY-MM-DD HH:MM:SS (UTC±HH:MM)` で表示 | TTY / 非TTY 共通 |
|  | `--json` | JSON Lines 出力 | デフォルトビルドで有効 |
|  | `--self-update` | 現在のインストール先を最新の公開版へ更新 | PATH や走査系オプションとは併用不可 |
|  | `--ext rs,toml` | 拡張子ホワイトリスト | ファイルのみ対象 |
|  | `--exclude-dots` | dotfiles を除外 | 既定では表示 |
|  | `--no-ignore` | built-in / ignore ファイルを無効化 | 挙動確認向け |
|  | `--no-labels` | `Fresh` ラベルを無効化 | TTY / JSON に影響 |
| `-I` | `--icons` | Nerd Font アイコンに切替 | `icons` feature 時のみ有効 |
| `-h` | `--help` | ヘルプを表示して終了 |  |
| `-V` | `--version` | バージョンを表示して終了 |  |

補足:

- `--ext` は大小無視、カンマ区切りです
- `--json` は TTY 判定や色設定の影響を受けません
- `--icons` が無効なビルドでもエラーにせず no-op として扱います

## 環境変数

| 変数 | 内容 |
| --- | --- |
| `NO_COLOR` | 値に関係なく、設定されていれば色を無効化 |
| `FTIME_FORCE_TTY` | stdout が pipe でも TTY レイアウトを強制 |
| `FTIME_IGNORE` | グローバル ignore ファイルの場所を上書き |
| `TZ` | ローカル日付境界や absolute time の確認時に影響 |

## 終了コード

| コード | 意味 |
| --- | --- |
| `0` | 正常終了 |
| `1` | 対象パス不正、存在しない、ディレクトリを読めないなどの致命的エラー |

個別エントリの権限エラーは、その項目を読み飛ばして処理継続します。

## パス解釈

- 出力パスは原則として `PATH` からの相対パスです
- 深さ 1 固定なので、サブディレクトリの中には潜りません
- シンボリックリンクは一覧対象ですが、リンク先の中身を再帰走査しません

## バケット規則

| バケット | 条件 |
| --- | --- |
| `active` | `now - mtime < 1h` または未来時刻 |
| `today` | `active` 以外で、ローカル日の当日 00:00 以降 |
| `this_week` | `today` 以外で、`now - mtime < 7d` |
| `history` | 上記以外 |

未来時刻は `+Ns [Skew]` / `+Nm [Skew]` として表示します。

## 出力契約

### TTY

- バケット順は `Active -> Today -> This Week -> History`
- `History` は既定で折りたたみ、`-a` で展開
- 各バケットは最大 20 件まで表示
- 行形式は `name | size | time`
- 末尾に `Current Timezone: UTC±HH:MM` を表示

### 非TTY

- 形式は `path<TAB>time`
- ヘッダ、色、バケット見出しは出しません
- 件数上限なしで全件出力します
- `-A` 指定時は絶対時刻に切り替わります

### JSON Lines

- 1 行 1 JSON
- 主なキーは `path`, `bucket`, `mtime`, `relative_time`, `is_dir`, `is_symlink`
- `symlink_target`, `label`, `size` は条件付きキーです
- `mtime` は RFC 3339 UTC です

## ignore 契約

- built-in ignore は `.DS_Store`, `Thumbs.db`
- グローバル ignore は `~/.ftimeignore`、または `FTIME_IGNORE`
- ローカル ignore は `<PATH>/.ftimeignore`
- `--no-ignore` でまとめて無効化します

ignore ファイルのルール:

- 1 行 1 パターン
- 空行と `#` で始まる行は無視
- `*` と `?` のみ対応
- `/` を含むパターンは相対パス、含まないものは basename にマッチ

## 制限

- 深さ 1 固定
- 読み取り専用
- 複数パス指定、再帰、glob/regex フィルタ、出力ロケール切替は未対応

## トラブルシューティング

### 何も表示されない
空ディレクトリか、`--exclude-dots` / `--ext` / ignore で全件落ちている可能性があります。

### 色が出ない
`NO_COLOR` が入っていないか、端末が ANSI を扱えるか確認します。

### TTY レイアウトを確認したい
`FTIME_FORCE_TTY=1` を使います。ログ用途なら `NO_COLOR=1` と併用します。

## 関連文書

- 入口: `README-ja.md`
- 使い方ガイド: `USER-GUIDE-ja.md`
- 読み分け案内: `ftime-overview-ja.md`
- 英語の正本: `CLI.md`
