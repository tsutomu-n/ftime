# ftime カスタマイズガイド（SE向け）

このドキュメントは、**環境変数・ignore設定・ビルド機能・簡易スクリプト**などを用いて ftime を自分/チーム用に調整したいSE向けの詳細ガイドです。  
機能追加ではなく、**既存仕様の範囲での調整方法**に限定しています。

---

## 1. 変更できるポイント（全体像）

| 種別 | 何が変えられるか | 変更手段 |
| --- | --- | --- |
| 出力モード | TTY/パイプ/JSON | 実行環境と `--json` |
| 色 | 有効/無効 | `NO_COLOR` |
| TTY強制 | パイプ時でもTTY出力 | `FTIME_FORCE_TTY` |
| ignore | 除外パターン | `~/.ftimeignore` / `./.ftimeignore` / `FTIME_IGNORE` |
| 拡張子抽出 | 指定拡張子のみ | `--ext` |
| ラベル | Freshを付けない | `--no-labels` |
| History | 折り畳み解除 | `-a` |
| アイコン | Nerd Fontアイコン | build時 `--features icons` |
| JSON | JSON無効化 | build時 `--no-default-features` |

---

## 2. 出力モードの制御

### 2.1 通常（TTY）
端末で直接実行すると **TTY出力** になります。  
バケット見出し、色、折り畳み（Historyの20件制限）あり。

### 2.2 パイプ/リダイレクト（TSV）
```
ftime | cat
```
この場合は **タブ区切り** で全件出力されます（見出し/色なし）。

### 2.3 JSON Lines
```
ftime --json
```
**1行1JSON**。機械処理向け。
`--json` が指定されている場合、TTY/パイプ判定より **JSONが優先**。

### 2.4 パイプでもTTY表示にしたい
```
FTIME_FORCE_TTY=1 ftime | cat
```
パイプ時でもTTYレイアウトを強制できます。  
**注意**: 色の有無は `NO_COLOR` に従います。

---

## 3. ignore設定（最重要カスタマイズ）

### 3.1 built-in ignore
デフォルトで **`.DS_Store` / `Thumbs.db`** は必ず無視されます。  
`--no-ignore` で built-in を含めて無効化できます。

### 3.2 グローバル ignore
`~/.ftimeignore` に書くと全プロジェクトに適用されます。  
`FTIME_IGNORE` でファイルパスを上書き可能。

```
# ~/.ftimeignore の例
*.log
target/
node_modules/
```

### 3.3 ローカル ignore
対象ディレクトリ直下の `.ftimeignore` を読み込みます。  
書式はグローバルと同じです。

### 3.4 パターン仕様（簡易グロブ）
- `*` = 0文字以上
- `?` = 1文字
- `\\` エスケープや `[]` 文字クラスは **非対応**
- `/` を含むパターンは **相対パス** にマッチ  
  それ以外は **ファイル名** にマッチ

---

## 4. 拡張子フィルタ（ピンポイント抽出）
```
ftime --ext rs,toml
```
- 指定した拡張子のみ表示（大文字小文字は無視）。
- **ファイルのみ対象**。ディレクトリとシンボリックリンクは除外されます。

---

## 5. ラベル/Historyの制御

### 5.1 Freshラベルを消す
```
ftime --no-labels
```
Fresh（5分以内）表示を無効にできます。

### 5.2 Historyを全件表示
```
ftime -a
```
Historyの折り畳みが解除されます。

---

## 6. ビルド時のカスタマイズ

### 6.1 Nerd Fontアイコン
```
cargo build --release --features icons
```
- `--icons` オプションが有効になります。
- **未ビルド時**に `--icons` を指定しても絵文字にフォールバックします。

### 6.2 JSON無効ビルド
```
cargo build --release --no-default-features
```
- `--json` が使えなくなります。

---

## 7. スクリプト/ワークフローへの組み込み例

### 7.1 最新10件だけ取り出す
```
ftime | head -n 10
```

### 7.2 パスだけ抽出
```
ftime | cut -f1
```

### 7.3 fzfで選んで開く（例: VS Code）
```
ftime | fzf --with-nth=1 | cut -f1 | xargs -r code
```

### 7.4 JSONの機械処理
```
ftime --json | jq -r 'select(.bucket=="today") | .path'
```

---

## 8. 仕様変更レベルのカスタマイズ（コード編集）

必要なら **コード変更でカスタム仕様**にできます。  
この章は「どこを触れば何が変わるか」を示します。

### 8.1 1バケットの表示件数上限（TTY）
- `src/view/tty.rs` の `const LIMIT: usize = 20;`
- Active / Today / This Week / History の **全バケットに共通**

### 8.2 Fresh判定の時間
- `src/util/time.rs` の `FRESH_WINDOW_SECS`（デフォルト5分）

### 8.3 バケットの境界
- `src/util/time.rs` の `classify_bucket`

### 8.4 デフォルトignore
- `src/engine.rs` の `default_ignores`

### 8.5 アイコンの中身
- `src/view/icon.rs` の `DefaultIconProvider` / `NerdIconProvider`

**注意**: これらは仕様変更に相当するため、チーム運用では影響範囲（テスト/ドキュメント）を必ず確認してください。

---

## 9. チーム運用でのコツ

- `.ftimeignore` を共有テンプレ化するとノイズを統一できる。
- CIでは `NO_COLOR=1` を標準化するとログが安定する。
- `FTIME_FORCE_TTY=1` はログ共有用途（色なし）で便利。

---

## 10. よくある落とし穴

| 症状 | 原因/対策 |
| --- | --- |
| パイプ処理で列がズレる | TSVなので `cut -f1` を使う |
| 期待ファイルが出ない | `.ftimeignore` / `--no-ignore` の影響を確認 |
| アイコンが出ない | `--features icons` でビルドしているか確認 |

---

## 11. 参考リンク
- 仕様: `docs/SPEC-ja.md`
- CLI詳細: `docs/CLI-ja.md`
- テスト計画: `docs/TESTPLAN-ja.md`
