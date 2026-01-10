# ftime 機能フルガイド（第三者向け・実装準拠）  
更新日: 2026-01-10 / 対象バージョン: v1.0 系（FS Edition）

この文書は、開発者以外の第三者でも `ftime` の目的・仕様・出力・運用上の注意を短時間で把握できるよう、実装済み機能に基づいて詳細に説明します。設計・試験方針は `ARCHITECTURE.md` / `SPEC-v1.0.md` / `TESTPLAN-v1.0.md` を参照してください。本書はそれらの統合的な読み物です。

---

## 1. 何を解決するツールか
- 直近に触ったファイルを「人間の時間感覚」に合わせて可視化したい。
- `ls -lt` では行数が多く、過去ノイズに埋もれる課題を解消したい。
- パイプやツール連携（fzf / jq / 自作スクリプト）を阻害しないシンプルな出力が欲しい。

フトコロ事情：
- 再帰やGit連携はあえて外し、「深さ1 + 時間バケット」の最小核に絞って高速性と明確さを担保。
- v1.0 で `--json` / 拡張子フィルタ / ignore は実装済み。軽量出自ラベルは `Fresh` のみ実装済み（その他は未着手）。

---

## 2. 基本仕様サマリ
- 深さ1のみを `read_dir` で列挙（サブディレクトリには潜らない）。
- 4バケット: Active(<1h), Today(今日), ThisWeek(<7d), History(それ以外)。未来時刻は Active。
- ソート: `mtime` DESC、同一 `mtime` は `name` ASC で安定化。
- TTY: 色＋アイコン（オプトイン）＋History折り畳み＋各バケット20件上限。
- 非TTY: TSV 2カラム（パス, 相対時間）、全件、ヘッダなし、色なし。
- JSON Lines: `--json` で固定フィールドを1行1オブジェクト。TTY判定・色・上限の影響なし。
- デフォルト除外: `.DS_Store`, `Thumbs.db`（`--hidden` でも除外）。
- 拡張子フィルタ: `--ext rs,toml`（大小無視、カンマ区切り）。ファイルのみ対象、ディレクトリと拡張子なしは除外。

---

## 3. CLIインターフェース
```
ftime [OPTIONS] [PATH]
```
PATH省略時はカレント。ファイルを渡すとエラー（終了コード1）。

### 主なオプション
- `--json`  
  - JSON Lines出力。フィールド凍結:  
    `path`, `bucket`, `mtime`(RFC3339/UTC), `relative_time`, `is_dir`, `is_symlink`, `symlink_target`(symlink解決時のみ), `label`(Fresh時のみ)  
  - 色/アイコン/20件上限なし。TTY/非TTYに依存しない。
- `--ext rs,toml`  
  - 拡張子ホワイトリスト（大小無視）。カンマ区切り。ファイルのみ対象。ディレクトリ・拡張子なしは除外。
- `-a, --all` : Historyを展開（TTYのみ、20件上限は維持）。
- `-H, --hidden` : ドットファイルを含める（デフォルトは除外）。`.DS_Store` / `Thumbs.db` は常に除外。
- `-I, --icons` : Nerd Fontアイコン（`--features icons` ビルド時、未導入でもフォールバック）。
- `-h, --help` / `-V, --version`

### 環境変数
- `NO_COLOR` : 最優先で色無効（空文字でも無効扱い）。
- `FTIME_FORCE_TTY` : 非TTYでもTTYレイアウトを強制（色の有無は `NO_COLOR` に従う）。

---

## 4. 出力詳細
### TTY モード
- 見出し: アイコン＋バケット名（例 `🔥 Active Context (< 1h)`）。`--icons` なしでも絵文字デフォルト。
- 行: `"  • {名前}  {relative_time}"`。  
  - ディレクトリ: 末尾 `/` + Bold Blue。  
  - symlink: `name -> target`（Yellow + dimmed）。未解決は `<unresolved>`。  
  - 通常ファイル: 無色（`NO_COLOR` 時は全て無色）。
- バケット上限: 各20件。21件以降は `... and N more items`。
- History: デフォルト折り畳み `💤 History (N files hidden)`、`-a` で展開。

### 非TTY（パイプ/リダイレクト）
- TSV 2カラム: `<path>\t<relative_time>`。ヘッダなし・全件・色なし・バケットなし。
- symlinkターゲットは表示しない（パスのみ）。

### JSON Lines
- `--json` 指定時のみ。1行1オブジェクトで固定フィールド（互換性重視、変更はメジャーのみ）。
- `symlink_target` と `label` は該当時のみ出力（それ以外は省略）。
- フィールド値の定義:
  - `bucket`: `"active" | "today" | "this_week" | "history"` の4種固定（小文字スネークケース）。
  - `symlink_target`: `is_symlink=true` かつ解決成功時のみ文字列（ベース相対優先）。解決失敗または非symlinkは出力しない。
  - `label`: Fresh のときのみ `"fresh"` を出力。該当しない場合は出力しない。
  - `relative_time`: TTY/TSV と同じ表記（下記「相対時間ルール」）。
- TTY/非TTYや色・上限ロジックに影響されない。

---

## 5. フィルタリング仕様
- 隠しファイル: デフォルト除外。`--hidden` で含める（ただし `.DS_Store` / `Thumbs.db` は常に除外）。
- 拡張子: `--ext` でホワイトリスト。ファイルのみ判定、拡張子なしは除外。大小無視。PATH がファイルの場合は先にエラー終了（コード1）するため `--ext` は実質無効。
- 追加の ignore 設定ファイルは実装済み。グローバルは `~/.ftimeignore`（`FTIME_IGNORE` で上書き）、ローカルは対象ディレクトリ直下の `.ftimeignore`。`--no-ignore` で無効化できる。
- ラベル: 約5分以内の更新に `Fresh` を付与。TTYでバッジ、JSONは `label` フィールド、TSVは非表示。`--no-labels` で無効化。

---

## 6. エラーと終了コード
- `0`: 成功（個別エントリ権限エラーはスキップ継続）。
- `1`: ルートが存在しない/ファイル/`read_dir` 不能など致命的。  
  ※ 個別エントリの読み取り失敗は stderr 通知なしでスキップ（将来 verbose 検討）。

---

## 7. パフォーマンスの目安
- 約2,000ファイル（devビルド, /dev/null）:  
  - TSV/TTY ~0.06s  
  - JSON ~0.25s  
- ソートは O(N log N)、深さ1限定。数千件規模で実用的速度。
- Nerd Fontアイコンをオフにしても性能差は軽微。

---

## 8. 代表的な利用例
- 最近の変更だけ見る:  
  `ftime`
- 昨日以前も確認（折り畳み解除）:  
  `ftime -a`
- 隠し込みで全体俯瞰:  
  `ftime -a -H`
- 特定拡張子のみ:  
  `ftime --ext rs,toml`
- 機械処理用JSON:  
  `ftime --json | jq '.bucket'`
- TTY書式をパイプでも強制（デモ/テスト）:  
  `FTIME_FORCE_TTY=1 ftime | head`
- 色なし決定版:  
  `NO_COLOR=1 ftime`

---

## 9. 実装上のポイント（挙動理解用）
- バケット順は Active → Today → ThisWeek → History で固定。早期一致。
- 未来時刻は Active に入れる（時計ずれ対策）。
- 表示上限は TTY のみ（JSON/TSVは全件）。
- ソートとバケットは engine 層で完結。ビューは順序を崩さない。
- 相対時間ルール（英語固定）
  - `<60s` : `just now`
  - `1m` : `1 min ago`
  - `2–59m` : `{N} mins ago`
  - `1h` : `1 hour ago`
  - `2–23h` : `{N} hours ago`
  - `Yesterday`
  - `2–6d` : `{N} days ago`
  - `>=7d` : `YYYY-MM-DD`（ローカル日付）
- シンボリックリンクのターゲット表示は TTY のみ。TSV はパスのみ。JSON は `symlink_target` を解決成功時のみ出力（失敗/非symlinkは省略）。`label` は Fresh のみ（約5分以内）で、JSON は `label` を出力（それ以外は省略）。TTYは小バッジで表示、TSVは表示しない。

---

## 10. 互換性ポリシー（抜粋）
- JSONフィールドは凍結（path, bucket, mtime, relative_time, is_dir, is_symlink, symlink_target, label）。変更はメジャーのみ。
- デフォルト動作（バケット定義、20件上限、TSVフォーマット、デフォルト ignore）は v1.0 以降も維持予定。
- 新オプション追加時は後方互換を最優先し、既存挙動を壊さない。

---

## 11. 非対応事項（v0.2 時点）
- 再帰走査、複数パス指定、glob/regexフィルタ、Git連携、言語/日付フォーマット切替、ページング。
- 出力ロケールは英語固定（相対時間）。TTY幅制御なし（折り返しは端末依存）。

---

## 12. 参考ドキュメント
- 仕様: `docs/SPEC-v1.0.md` / `docs/SPEC-ja.md`
- 設計: `docs/ARCHITECTURE.md` / `docs/ARCHITECTURE-ja.md`
- CLI契約: `docs/CLI.md` / `docs/CLI-ja.md`
- 試験計画: `docs/TESTPLAN-v1.0.md` / `docs/TESTPLAN-ja.md`
- ロードマップ: `docs/12-10_ROADMAP.md`

---

## 13. 運用のヒント
- CIでの決定論的出力: `NO_COLOR=1 FTIME_FORCE_TTY=1 ftime ...`
- fzf連携: `ftime --ext rs | fzf --with-nth=1 | cut -f1`
- 大量履歴でTTYを抑制: パイプ出力に切り替え (`ftime | head`) すれば上限も色も無効化。
- ベンチ再計測: `/tmp` に多数ファイルを生成し `time ftime --json /tmp/dir` で確認（JSONのオーバーヘッドを把握）。

---

## 14. 今後の拡張（ロードマップ抜粋）
- Phase 2: JSON必須化済み、拡張子フィルタ実装済み。Optionalとして軽量出自ラベル検討（Freshは実装済み）。
- Phase 3: 互換性ポリシー凍結、ignore の高度化（gitignore 互換・複数ルート等）、配布パッケージ、SPEC/TESTPLAN v1.0 作成。
- 将来: Gitモード、TUI (`--explore`)、再帰や since フィルタなどは別フェーズで検討。

---

### Appendix: 現状のフェーズ対応表

| 機能                                | 実装状況 | ロードマップ上の位置付け          |
|-------------------------------------|----------|-----------------------------------|
| 深さ1スキャン                       | 実装済   | Phase 1 Core                      |
| 時間バケット(Active〜History)       | 実装済   | Phase 1 Core                      |
| TSV(パイプ)出力                     | 実装済   | Phase 1 Core                      |
| JSON Lines (`--json`)               | 実装済   | Phase 2 Must（前倒し完了）        |
| 拡張子フィルタ (`--ext`)            | 実装済   | Phase 2                           |
| デフォルト ignore (.DS_Store 等)    | 実装済   | Phase 2                           |
| Nerd Fontアイコン (`--icons`)       | 実装済   | Phase 1+α（オプトイン）          |
| 軽量出自ラベル                      | 一部実装済（Freshのみ） | Phase 2 Optional                  |
| `~/.ftimeignore` 等の ignore         | 実装済   | Phase 2                           |
| 再帰・depth指定                     | 未実装   | 未来検討                          |
| Gitモード / TUI (`--explore`)       | 未実装   | 未来検討                          |

---

このガイドを読めば、第三者でも `ftime` の目的・挙動・入力/出力仕様・運用上の注意を把握し、環境やユースケースに合わせて最適なモードを選択できます。***
