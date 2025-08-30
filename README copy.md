# ftime — Simple File Time Viewer

A tiny, dependency‑light CLI to list files with their modified and created times.

| Column   | Meaning                                                                              |
|----------|--------------------------------------------------------------------------------------|
| modified | When the file content was last changed (format: `MM/DD_HH:MM`)                      |
| created  | When the file was created (or `-` if your filesystem doesn’t store creation time)   |
| name     | File or directory name (colored by type/extension when color is enabled)            |

Designed to be friendly for junior engineers and non‑native English speakers.

---

## Requirements

- Linux with GNU coreutils: `stat`, `date`
- Bash shell (`#!/usr/bin/env bash`)

---

## Install (recommended: from a cloned repo)

Use a symlink in `~/.local/bin`. This makes a real command named `ftime`. It works in scripts and in CI.

1) Clone anywhere you like

```bash
git clone https://github.com/tsutomu-n/ftime.git
cd ftime   # go into the repo root
```

2) Make the script executable

```bash
chmod +x ftime-list.sh
```

3) Ensure `~/.local/bin` is on PATH (zsh/bash auto‑detect; creates rc file if missing)

```bash
if [ -n "$ZSH_VERSION" ]; then
  rc="${ZDOTDIR:-$HOME}/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
  rc="$HOME/.bashrc"
else
  rc="$HOME/.profile"
fi
mkdir -p "$(dirname "$rc")"
grep -q '\.local/bin' "$rc" 2>/dev/null || \
  echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$rc"
. "$rc"
```

4) Create a command named `ftime`

```bash
mkdir -p ~/.local/bin
ln -sf "$PWD/ftime-list.sh" ~/.local/bin/ftime
```

5) Refresh and test

```bash
hash -r
ftime --help
```

### Uninstall

```bash
rm ~/.local/bin/ftime
```

---

## Install (one‑liner: download only)

You do not need to clone the repo. Download the script and make it executable.

```bash
mkdir -p ~/.local/bin
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/ftime-list.sh \
  -o ~/.local/bin/ftime
chmod +x ~/.local/bin/ftime

# test
hash -r
ftime --help
```

**Notes**

- If your shell still does not find `ftime`, open a new terminal or run `source ~/.zshrc`.
- This tool requires Linux with GNU `stat` and `date`, and Bash.

---

## Usage

```bash
ftime [DIR] [PATTERN ...]
```

- **DIR (optional)**: directory to scan. Default: current directory.
- **PATTERN(s) (optional, OR)**. Token rules:
  - contains `*` or `?` → used as‑is (glob)
  - starts with `.` → prepend `*` (e.g., `.log` → `*.log`)
  - otherwise → treat as extension (e.g., `md` → `*.md`)

### Examples

```bash
ftime                 # list everything in current dir
ftime md              # only *.md
ftime py              # only *.py
ftime .log            # only *.log
ftime docs md         # *.md inside ./docs
ftime '*.test.*'      # explicit glob
```

---

## Demo (SVG) – drop‑in placeholders

<!--
  How to use:
  1) Generate SVGs into ./media via `make rec-<name>`, `make demos`, or manual svg-term + svgo.
  2) Keep width between 640–960px to avoid line wrapping.
  3) Update alt texts if you change the scenario.
-->

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

> TIP: If SVGs aren’t ready yet, keep these tags as is. Once you add files under `./media/`, they will render automatically on GitHub.


### SVG Demo Workflow (asciinema → svg-term → SVGO)

Quickest path to a crisp animated SVG for your README.

1) Record (.cast)

```bash
# Press Ctrl-D (or type 'exit') to finish
asciinema rec demo.cast
# run a few ftime commands during recording
```

2) Convert to SVG (uses npx)

```bash
npx -y svg-term --cast demo.cast --out demo.svg --window --no-cursor
```

3) Optimize (smaller SVG)

```bash
npx -y svgo --multipass -o demo.min.svg demo.svg
```

4) Embed in README

```markdown
![ftime demo](./demo.min.svg)
```

### Make targets

```bash
make rec-basic     # records basic.cast
make rec-pattern   # records pattern.cast
make rec-dir       # records dir.cast
make rec-tz        # records tz.cast
make demos         # builds media/{basic,pattern,dir,tz}.min.svg
```

> Tip: Fix your terminal width (e.g., 80–100 cols). Use `--no-cursor` and keep clips under ~10s.

---

## Timezone

- Default: your machine’s local timezone
- Override: env var `FTL_TZ` (example: `FTL_TZ=Asia/Tokyo ftime md`)

## Color

- Auto on TTY; only the name column is colorized (by extension)
- Force ON for pipes/pagers: `FTL_FORCE_COLOR=1 ftime | less -R`
- Turn OFF: `NO_COLOR=1 ftime` or `FTL_NO_COLOR=1 ftime`

## Legend

- Shown by default on TTY; disable with `FTL_NO_LEGEND=1`
- Short form example (default): `Legend (tz:local)  ·  '-' = unknown created`

---

## Security / Limitations

- Creation time depends on filesystem/kernel/tools; it may show `-`.
- Filenames can contain control characters. Colors are added only to the name column. Use caution when copying colored output to places that interpret ANSI.
- Linux/GNU only. macOS/BSD use different `stat`/`date` flags.

---

## License

This project is released under the MIT License. See `LICENSE`.
