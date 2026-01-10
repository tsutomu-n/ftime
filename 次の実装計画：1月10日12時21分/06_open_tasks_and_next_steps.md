# 06 残タスクと次アクション

## 未完了（Phase3 の残り）
### プラットフォーム検証（未実施）
- 対象 OS の明示（Linux major distros / macOS など）
- DST/ローカル時刻境界の挙動確認
- TTY 判定差の確認（`IsTerminal`）
- symlink/パーミッションエラーの挙動差の確認

### ノイズ除外の整理（明文化の再確認）
- デフォルト ignore の最小セット定義の最終確認
- `~/.ftimeignore` の将来拡張余地を明文化（現状は記述済みだが最終整合が必要）

### 配布・公開準備（未実施）
- `cargo package --list`
- `cargo publish --dry-run`
- GitHub Releases のバイナリ作成（Linux/macOS）

### ドキュメント最終整合
- `docs/CLI.md` の v1.0 互換性ポリシーの明文化の最終チェック
- “Experimental” として明記すべき対象の洗い出し
- v1.0 リリースノート草案の作成

## 直近の次アクション（順番）
1) 対象 OS 明記とプラットフォーム検証の実施結果を docs に反映  
2) `cargo package --list` と `cargo publish --dry-run` を実行して配布チェック  
3) v1.0 リリースノート草案作成  
4) Phase3 完了条件チェック → 完了宣言
