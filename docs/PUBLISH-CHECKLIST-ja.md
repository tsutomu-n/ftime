# ftime 外部公開チェックリスト

この文書は、`ftime` の repo 内変更を外部サービスへ反映するときの、最短手順だけをまとめた maintainer 向けメモです。対象は `crates.io` 公開と GitHub リポジトリ設定の更新です。

## この文書を使う場面

- README や `Cargo.toml` の公開導線を更新した
- `cargo publish --dry-run --allow-dirty` は通っている
- まだ `crates.io` と GitHub の公開面には反映されていない

## 事前条件

### crates.io

- crates.io にログインできる
- メール確認が終わっている
- API token を発行済み
- `cargo login` が済んでいる、または `CARGO_REGISTRY_TOKEN` を設定済み

確認:

```bash
test -f "$HOME/.cargo/credentials.toml" && echo credentials-present || echo credentials-missing
test -n "$CARGO_REGISTRY_TOKEN" && echo token-env-present || echo token-env-missing
```

### GitHub

- `tsutomu-n/ftime` の settings を変更できる権限がある
- GitHub にログイン済みのブラウザが使える

## 実行手順

### 1. repo 側の最終確認

```bash
git status --short
CI=true timeout 30 cargo test --all-features
cargo publish --dry-run --allow-dirty
cargo package --list --allow-dirty
```

見るポイント:

- `cargo test` が通る
- `cargo publish --dry-run --allow-dirty` が通る
- package 内容に意図しない秘密情報や巨大ファイルが入っていない

### 2. commit を作る

`cargo publish` 前に tree を clean にします。

```bash
git add Cargo.toml README.md docs/README-ja.md docs/README-zh.md tests/release_docs.rs tests/support/mod.rs assets/social-preview.png
git commit -m "docs: add crates.io install path"
git status --short
```

`git status --short` が空であることを確認します。

### 3. crates.io に公開する

```bash
cargo publish
```

止まる典型例:

- `error: credentials not found`
- `error: the remote server responded with an error (status 403)`
- `error: crate version \`1.0.1\` is already uploaded`

`version already exists` の場合は、この場で無理に続けず version bump を別タスクに切り分けます。

### 4. repo 外で公開確認する

`cargo` の workspace 影響を避けるため、repo 外で確認します。

```bash
cd /tmp
cargo info ftime
cargo search ftime --limit 20
```

必要なら install まで確認します。

```bash
tmpdir="$(mktemp -d)"
cargo install ftime --locked --root "$tmpdir"
"$tmpdir/bin/ftime" --version
```

反映が遅いときは、最大 10 分程度、30 秒おきに再確認します。

### 5. publish 後に push する

README に `crates.io` 導線を書いているので、publish 成功を確認してから push します。

```bash
git push origin main
```

### 6. GitHub Settings を更新する

GitHub リポジトリの Settings で以下を反映します。

About:

```text
Read-only CLI to see recently changed files in time buckets, without recursive noise.
```

Topics:

```text
cli
rust
filesystem
terminal
productivity
developer-tools
shell
mtime
jsonl
```

Social preview:

- `assets/social-preview.png`

### 7. 公開面を確認する

確認項目:

- GitHub の public page で README 冒頭が新文面になっている
- About が旧文言ではない
- Topics が 9 個に増えている
- social preview が新画像になっている
- README の `crates.io` 導線から install 手順が読める

## 失敗時の切り分け

### `cargo publish` できない

- 認証が無い: `cargo login` または `CARGO_REGISTRY_TOKEN`
- version 重複: version bump を別タスク化
- package 内容が不正: `cargo package --list --allow-dirty` を見直す

### GitHub に反映されない

- push していない
- GitHub settings をまだ保存していない
- ログアウト状態で public view だけ見ている

## 今回の固定値

- GitHub Releases は引き続き主導線
- `crates.io` は Rust/Cargo 利用者向けの副導線
- Homebrew / Scoop / WinGet / Zenn / X はこの手順には含めない
