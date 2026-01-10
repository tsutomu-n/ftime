# ftime ユーザーガイド（エンドユーザー向け・詳細版）

このガイドは、 **「何ができるか」「どう使うか」「出力をどう読むか」** を迷わず理解できる粒度でまとめた手引きです。仕様・設計の詳細は `SPEC-ja.md` と `ARCHITECTURE-ja.md` にあります。

---

## 0. まず知っておくべき要点（30秒まとめ）
- **非再帰（深さ1）** の最新ファイル一覧ツール。
- **読み取り専用**、ディレクトリ配下のファイルを **mtime降順** で並べる。
- **Active / Today / This Week / History** の4バケットで視認性を高める。
- TTY時は色・見出し・折り畳み、パイプ時は **タブ区切り** の機械向け出力。
- JSON Lines出力に対応（`--json`）。

---

## 1. ftime とは
ftime は **「最近更新したファイルを素早く見つける」** ためのCLIです。  
特徴は以下のとおりです。

- **非再帰（深さ1）**: 対象ディレクトリ直下のみをスキャン（サブディレクトリは潜らない）。
- **読み取り専用**: 変更・削除は一切しない。
- **mtime降順ソート**: 新しい順に並ぶ（同時刻は `name` 昇順）。
- **4バケット表示**: Active / Today / This Week / History に分類。
- **出力モード自動切替**: TTYなら人向け、パイプなら機械向け。

---

## 2. インストール / セットアップ
要件: Rust/Cargo 1.92+（edition 2024）

### 2.1 GitHub Releases からインストール（推奨）
```
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/install.sh | bash

# Windows (PowerShell)
powershell -ExecutionPolicy Bypass -Command "iwr https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/install.ps1 -UseBasicParsing | iex"
```

### 2.2 crates.io からインストール（公開済みの場合）
```
cargo install ftime
```
未公開の場合は、**GitHub Releases** または **ソースからビルド** を利用してください。

### 2.3 ソースからビルド
```
cargo build --release
```
生成物: `target/release/ftime`

実行例:
```
./target/release/ftime
```

どのディレクトリでも `ftime` だけで使いたい場合は、次のどちらかを選びます（環境によりPATH設定が必要です）:
```
cargo install --path .
# または（Linux/macOS）: ln -s /path/to/ftime/target/release/ftime ~/bin/ftime
```

### 2.4 追加ビルドオプション
- **Nerd Fontアイコン**: `cargo build --release --features icons`
- **JSON無効ビルド**: `cargo build --release --no-default-features`
  - JSONはデフォルトで有効（`json` feature）。
  - `--no-default-features` でビルドすると `--json` オプションは使えない。

---

## 3. 基本コマンド
```
ftime [OPTIONS] [PATH]
```

- `PATH` 省略時は **カレントディレクトリ**。
- 出力のパスは **PATHからの相対パス**。

---

## 4. 出力形式の全体像

| モード | いつ起きるか | 形式 | 見出し/色 | 件数制限 | History |
| --- | --- | --- | --- | --- | --- |
| TTY | 通常の端末 | 人向け | あり | 1バケット最大20件 | 折り畳み（`-a`で展開） |
| Pipe/リダイレクト | `|` や `>` のとき | タブ区切り | なし | なし | 常に出力 |
| JSON Lines | `--json` | 1行1JSON | なし | なし | 常に出力 |

### 4.1 TTY出力（人間向け）
例:
```
🔥 Active Context (< 1h)
  • src/main.rs  12 mins ago  ✨ Fresh
  • docs/CLI.md  2 hours ago

☕ Today's Session
  • README.md  3 hours ago

📅 This Week
  • tests/cli.rs  2 days ago
  ... and 5 more items

💤 History (42 files hidden)
```

ポイント:
- **バケット順固定**: Active → Today → This Week → History
- **1バケット最大20件**。超過分は `... and N more items` と表示。
- **Historyはデフォルト折り畳み**。件数だけ表示。`-a` で展開。
- ディレクトリは末尾 `/` 付きで表示。
- シンボリックリンクは `name -> target`。

### 4.2 Pipe出力（機械向け）
- 形式: `<path>\t<relative_time>`
- **見出しなし / 色なし / 件数制限なし**
- **全ファイルをmtime降順で出力**

例:
```
src/main.rs\t12 mins ago
docs/CLI.md\t2 hours ago
README.md\t3 hours ago
```

### 4.3 JSON Lines出力
`--json` 指定時のみ有効（json feature ビルド時）。

1行1オブジェクト:
```
{"path":"src/main.rs","bucket":"active","mtime":"2026-01-10T05:12:20.214004873+00:00","relative_time":"just now","is_dir":false,"is_symlink":false,"label":"fresh"}
```

フィールド:
- `path`: PATHからの相対パス
- `bucket`: `active` / `today` / `this_week` / `history`
- `mtime`: **RFC3339 (UTC)**
- `relative_time`: 人間向け相対時間（下記参照）
- `is_dir`: ディレクトリかどうか
- `is_symlink`: シンボリックリンクかどうか
- `symlink_target`: ターゲットが取得できた場合のみ出力
- `label`: `fresh` のみ（該当時のみ出力）

順序:
- 出力順は **Active → Today → This Week → History** の順でバケットごと。
- 各バケット内は mtime降順 / name昇順。

---

## 5. 時間バケットの定義（厳密）

| バケット | 条件 |
| --- | --- |
| Active | `now - mtime < 1時間` または `mtime` が未来で `duration_since` が失敗 |
| Today | ローカル時刻の **当日0:00以降** |
| This Week | `now - mtime < 7日` かつ Today ではない |
| History | 上記以外 |

補足:
- **ローカルタイムゾーン依存**。`TZ` の設定で境界が変わる。
- DST（サマータイム）切り替えはローカル時刻境界に影響する。

---

## 6. relative_time の出力ルール

| 経過時間 | 表示 |
| --- | --- |
| < 60秒 | `just now` |
| = 1分 | `1 min ago` |
| 2–59分 | `N mins ago` |
| = 1時間 | `1 hour ago` |
| 2–23時間 | `N hours ago` |
| = 1日 | `Yesterday` |
| < 7日 | `N days ago` |
| >= 7日 | `YYYY-MM-DD`（ローカル日付） |

---

## 7. ラベル（Fresh）
- **5分以内** の変更は `✨ Fresh` が付く。
- `--no-labels` で無効化可能。
- JSONでは `label: "fresh"` を出力。
- 未来時刻のファイルは Fresh にならない。

---

## 8. オプション詳細

| オプション | 内容 | 使いどころ |
| --- | --- | --- |
| `-a, --all` | Historyを展開 | 7日以上前も一覧したい |
| `-H, --hidden` | ドットファイルを含める | `.env` や `.gitignore` を含めたい |
| `--no-ignore` | built-in / ignoreファイルを無効化 | `.DS_Store` なども含めたい |
| `--no-labels` | Freshラベルを無効化 | 出力を簡潔にしたい |
| `--ext rs,toml` | 拡張子フィルタ（ファイルのみ） | 特定拡張子だけ見たい |
| `-I, --icons` | Nerd Fontアイコン | 文字アイコンに切替 |
| `--json` | JSON Lines出力 | 機械処理したい |

`--icons` 補足:
- `icons` feature でビルドされていない場合、指定しても **絵文字アイコンにフォールバック**。

---

## 9. ignore ルール（重要）

### 9.1 built-in ignore
- デフォルトで **`.DS_Store` と `Thumbs.db` を除外**。
- `--hidden` を付けても除外のまま。
- `--no-ignore` を付けると **built-in も含めて無効化**。

### 9.2 グローバル ignore（`~/.ftimeignore`）
- 1行1パターン。
- `#` から始まる行と空行は無視。
- `FTIME_IGNORE` でファイルパスを上書き可能。

### 9.3 ローカル ignore（`./.ftimeignore`）
- 対象ディレクトリ直下の `.ftimeignore` を読み込む。
- フォーマットはグローバルと同じ。

### 9.4 パターン仕様（簡易グロブ）
- `*` = 0文字以上
- `?` = 1文字
- `\\` エスケープや `[]` 文字クラスはなし
- `/` を含むパターンは **相対パス** にマッチ、それ以外は **ファイル名** にマッチ

---

## 10. 環境変数

| 変数 | 内容 |
| --- | --- |
| `NO_COLOR` | 色出力を無効化（空文字でも無効扱い） |
| `FTIME_FORCE_TTY` | パイプ時でもTTY書式を強制 |
| `FTIME_IGNORE` | グローバル ignore ファイルのパスを上書き |
| `TZ` | タイムゾーンを明示（ローカル境界確認用） |

---

## 11. 対象範囲とソート
- **非再帰（深さ1）** のみ。
- **mtime降順**、同時刻は **name昇順**。
- `--ext` 指定時:
  - **ファイルのみ**が対象。
  - ディレクトリ・シンボリックリンクは除外。

---

## 12. ディレクトリ / シンボリックリンクの扱い
- ディレクトリ:
  - TTY: 末尾 `/` を付与
  - Pipe/JSON: パスのみ
- シンボリックリンク:
  - TTY: `name -> target` 表示
  - `read_link` 失敗時は `<unresolved>`
  - 破損リンクでも `read_link` が成功すれば target は表示される
  - Pipe: パスのみ
  - JSON: `is_symlink=true`、`symlink_target` は取得できた時のみ出力

---

## 13. エラーと終了コード

| 状況 | 出力 | 終了コード |
| --- | --- | --- |
| PATHがファイル | エラーメッセージ | 1 |
| ディレクトリ読み取り失敗 | エラーメッセージ | 1 |
| 該当ファイルなし | `No recent files found`（TTY） / 空出力（Pipe/JSON） | 0 |

---

## 14. 代表的ユースケース

### 14.1 朝イチで作業再開
```
ftime
```
Active / Today で「直近の作業ファイル」を即確認。

### 14.2 昨日以前も含めて確認
```
ftime -a
```
Historyも含めて一覧。

### 14.3 dotfileを含める
```
ftime -H
```
`.env` や `.gitignore` の変更確認に有効。

### 14.4 拡張子フィルタ
```
ftime --ext rs,toml
```
Rustや設定ファイルだけ抽出。

### 14.5 パイプ連携
```
ftime | head -n 10
```
最初の10件だけ確認。

### 14.6 JSONで収集
```
ftime --json | head -n 5
```
JSONで取得し、別処理へ。

---

## 15. トラブルシュート

| 症状 | 原因/対処 |
| --- | --- |
| 何も出ない | 空ディレクトリ or 直下に対象ファイルがない |
| Historyが出ない | `-a` を付ける |
| 色が出ない | `NO_COLOR` が設定されている |
| Todayの境界がずれる | `TZ` が想定と違う |
| パス処理で区切りが崩れる | パイプはタブ区切り、`cut -f1` などで処理 |

---

## 16. FAQ

**Q: 再帰しない理由は？**  
A: 深さ1で高速・シンプルに使えるように設計。

**Q: 20件上限を変えられる？**  
A: 現行は固定。全件はパイプ/JSONで取得。

**Q: 未来時刻のファイルは？**  
A: Activeに入る（未来時刻は「直近」とみなす）。

**Q: JSONのmtimeはなぜUTC？**  
A: 互換性と機械処理の安定性を優先。

---

## 17. コマンド早見表

| 目的 | コマンド |
| --- | --- |
| 直近を見る | `ftime` |
| Historyまで全部 | `ftime -a` |
| 隠しファイル込み | `ftime -H` |
| 拡張子フィルタ | `ftime --ext rs,toml` |
| パイプ処理 | `ftime | head -n 10` |
| JSON取得 | `ftime --json` |

---

## 18. 参考リンク
- 仕様: `SPEC-ja.md`
- 設計: `ARCHITECTURE-ja.md`
- CLI詳細: `CLI-ja.md`
- テスト計画: `TESTPLAN-ja.md`
