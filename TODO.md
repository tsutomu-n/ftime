# TODO – README向けSVGデモ（最短・必須）

> 目的: 非英語話者のジュニアSEでも、一目で「何ができるか」を理解できる短尺SVGを4本用意する。

---

## 前提条件（1回だけ）
- Linux端末、幅80〜100桁の等幅フォント（折り返し防止）
- Node.js（npxが使える）
- `asciinema` と `svg-term-cli` と `svgo` を用意
  - `sudo apt install asciinema`（または `pip install --user asciinema`）
  - `npm i -g svg-term-cli svgo`（グローバル導入が嫌なら npx/pnpm dlx を使う）
- このリポジトリの `ftime/` をカレントにして作業
  - `cd /home/tn/projects/ftime`
  - SVG保存場所: `ftime/media/`（既に作成済み）

ヒント: Makefile を用意済み。`make rec` / `make svg` / `make svg-min` が使える。

---

## すぐやること（チェックリスト）
- [ ] 端末サイズを固定（例: 100×28）し、CJK＋ASCII名が混在する小さなディレクトリで作業
- [ ] 4本の .cast を録画（Basic / Pattern / Directory / Timezone）
- [ ] .cast → .svg に変換（`--window --no-cursor`）
- [ ] .svg → .min.svg に最適化（svgo --multipass）
- [ ] READMEに `media/*.min.svg` を貼る（幅を 640〜960px に固定）

---

## 録画：4本（asciinema）
> 各クリップ 4〜8秒。入力間隔は短く、1コマンド=1メッセージ。
> 

1) Basic list（核：三列＋legend）
```bash
asciinema rec basic.cast   # Ctrl-D または exit で終了
ftime
exit
```

2) Pattern shorthand（md / .log / OR）
```bash
asciinema rec pattern.cast
ftime md
ftime .log
ftime md py
exit
```

3) Directory argument（DIR + パターン）
```bash
asciinema rec dir.cast
ftime docs md
exit
```

4) Timezone override（local ↔ 指定TZ）
```bash
asciinema rec tz.cast
ftime .
FTL_TZ=Asia/Tokyo ftime .
exit
```

---

## 変換：.cast → .svg（svg-term-cli）
> 端末フレーム `--window`、点滅抑止 `--no-cursor` を付ける。
```bash
npx -y svg-term --cast basic.cast   --out media/basic.svg   --window --no-cursor
npx -y svg-term --cast pattern.cast --out media/pattern.svg --window --no-cursor
npx -y svg-term --cast dir.cast     --out media/dir.svg     --window --no-cursor
npx -y svg-term --cast tz.cast      --out media/tz.svg      --window --no-cursor
```

（Makefileを使う場合の例）
```bash
make CAST=basic.cast  SVG=media/basic.svg  svg
make CAST=pattern.cast SVG=media/pattern.svg svg
make CAST=dir.cast    SVG=media/dir.svg    svg
make CAST=tz.cast     SVG=media/tz.svg     svg
```

---

## 最適化：.svg → .min.svg（SVGO）
```bash
npx -y svgo --multipass -o media/basic.min.svg   media/basic.svg
npx -y svgo --multipass -o media/pattern.min.svg media/pattern.svg
npx -y svgo --multipass -o media/dir.min.svg     media/dir.svg
npx -y svgo --multipass -o media/tz.min.svg      media/tz.svg
```

（Makefileを使う場合の例）
```bash
make SVG=media/basic.svg   MIN=media/basic.min.svg   svg-min
make SVG=media/pattern.svg MIN=media/pattern.min.svg svg-min
make SVG=media/dir.svg     MIN=media/dir.min.svg     svg-min
make SVG=media/tz.svg      MIN=media/tz.min.svg      svg-min
```

---

## READMEへの貼り付け（Demoセクション）
Markdown（最短）
```md
![ftime basic](./media/basic.min.svg)
![ftime patterns](./media/pattern.min.svg)
![ftime dir](./media/dir.min.svg)
![ftime tz](./media/tz.min.svg)
```

HTML（幅固定・代替テキスト強化）
```html
<p align="left">
  <img src="./media/basic.min.svg"   alt="ftime: see modified/created/name at a glance" width="720" />
</p>
<p align="left">
  <img src="./media/pattern.min.svg" alt="ftime: pattern shorthand (md / .log / OR)" width="720" />
</p>
<p align="left">
  <img src="./media/dir.min.svg"     alt="ftime: target another directory (docs md)" width="720" />
</p>
<p align="left">
  <img src="./media/tz.min.svg"      alt="ftime: switch timezone via env var (legend shows tz)" width="720" />
</p>
```

---

## 品質チェック（確認項目）
- [ ] 端末幅 80〜100 列で列折返しなし（timeカラムは固定幅）
- [ ] legend に `tz:local` / `tz:Asia/Tokyo` が出ている
- [ ] 名前列のみ色付き（パイプは不要。Pager例は今回は割愛）
- [ ] 1クリップ 4〜8秒。間延びがない
- [ ] alt テキストは「何が便利か」を一言で要約

---

##（任意）GitHub Actionsで生成したい
- `ftime/.github/workflows/svg.yml` は `demo.cast` → `demo.svg` → `demo.min.svg` を生成しコミット
- 使い方：`ftime/demo.cast` をコミット → Actions から手動トリガー（workflow_dispatch）

---

## よくあるハマり
- 文字がにじむ → ビデオ→GIFの劣化。**asciinema→SVG**ならクッキリ
- 列が折り返される → 端末幅固定／READMEで `<img width>` 指定
- 色が消える → Pager時は `less -R`、または SVGでは色付き録画を短尺で
- ファイルが重い → `svgo --multipass`、短めに録る
- .cast をGitに入れたくない → `.gitignore` に `*.cast` を追加
