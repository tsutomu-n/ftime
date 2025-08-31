# ftime — Simple File Time Viewer

Translations: [日本語](README-ja.md) · [中文](README-zh.md)

A tiny, dependency‑light CLI to list files with their modified and created times.

<p align="left">
  <img src="./media/basic.gif"   alt="ftime: see modified/created/name at a glance" width="600" />
  
</p>

| Column   | Meaning                                                                              |
|----------|--------------------------------------------------------------------------------------|
| mark     | One‑char indicator placed before "modified". `+` means the file was modified after it was created; blank otherwise (yellow when color is enabled). |
| modified | When the file content was last changed (format: `MM-DD HH:MM`)                      |
| created  | When the file was created (colored by recency; or `-` if not available)   |
| name     | File or directory name (colored by type/extension when color is enabled)            |

Designed to be friendly for junior engineers and non‑native English speakers. Features improved error messages and a beginner-friendly help system.

---

## Requirements

- GNU coreutils: `stat`, `date`
- GNU findutils: `find` with `-printf`/`-print0` and GNU `sort` (with `-z`)
- Bash shell (`#!/usr/bin/env bash`)

macOS note:
- Supported when GNU tools are installed. Install via Homebrew:
  ```bash
  brew install coreutils findutils   # provides gstat/gdate/gfind/gsort
  ```
  The script auto-detects `gstat/gdate/gfind/gsort` and falls back to `stat/date/find/sort` when GNU variants are the defaults.
  Alternatively, add Homebrew's "gnubin" to PATH to use GNU names without the `g` prefix (as documented by Homebrew):
  ```bash
  PATH="$(brew --prefix)/opt/coreutils/libexec/gnubin:$(brew --prefix)/opt/findutils/libexec/gnubin:$PATH"
  ```

---

## Uninstall

```bash
rm ~/.local/bin/ftime
```

---

## Install (one‑liner: download only) – Recommended

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

<details>
  <summary><strong>Install (from a cloned repo) – Optional</strong></summary>

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

</details>

**Notes**

- If your shell still does not find `ftime`, open a new terminal or run `source ~/.zshrc`.
- This tool requires Linux with GNU `stat` and `date`, and Bash.

---

## Usage

### Quick Start


```bash
ftime               # List files in current directory
ftime -a            # Show relative age instead of absolute timestamps
ftime -s time       # Sort by modified time (newest first)
ftime -R -d 2 md    # Recurse up to depth 2 and list *.md
ftime --git-only    # Show only files changed/staged/untracked in the current git repo
ftime --help        # Show detailed help
ftime --help-short  # Short help (3 lines)
ftime --version     # Show version
```

### Full Syntax

```bash
ftime [DIR] [PATTERN ...]
```

- **DIR (optional)**: directory to scan. Default: current directory.
- **PATTERN(s) (optional, OR)**. Token rules:
  - contains `*` or `?` → used as‑is (glob)
  - starts with `.` → prepend `*` (e.g., `.log` → `*.log`)
  - otherwise → treat as extension (e.g., `md` → `*.md`)

### Options

- **-a, --age**: show relative time (e.g., `5m`, `3h`) instead of absolute timestamps
- **-s, --sort time|name**: sort key (default: name; `time` = modified time)
- **-r, --reverse**: reverse the sort order
- **-R, --recursive**: recurse into subdirectories
- **-d, --max-depth N**: limit recursion depth to N (requires -R)
- **--git-only**: show only files changed/staged/untracked in the current git repo (falls back to full list if not a repo). Respects `.gitignore` via `--exclude-standard`, works when invoked from subdirectories.
- **-h, --help**: show full help
- **--help-short**: show short help
- **-V, --version**: show version

### Examples (combinations)

```bash
# Recurse the entire tree (may be large)
ftime -R

# Recurse up to depth 3
ftime -R -d 3

# Depth 2 under docs/, only *.md
ftime -R -d 2 docs md

# Sort by modified time; recurse 1 level
ftime -s time -R -d 1

# Oldest first by modified time
ftime -s time -r -R
```

### Common pitfalls

- **Provide a number with -d**
  ```bash
  ftime -d          # Error: --max-depth expects a positive integer
  ftime -d -R       # Error: --max-depth expects a positive integer
  ftime -R -d 3     # OK
  ftime -d 3 -R     # OK (option order doesn't matter)
  ```

- **Use -R with -d**
  ```bash
  ftime -d 3        # Error: --max-depth requires --recursive (-R)
  ftime -R -d 3     # OK
  ```

- **Quote patterns to prevent shell expansion**
  ```bash
  ftime '*.md'      # OK: pattern handled by ftime as a filter
  ftime *.md        # Your shell expands to file names; may behave unexpectedly
  ```

- **Depth is relative to the starting DIR**
  ```bash
  ftime -R -d 1 docs   # docs/ and its direct children (not grandchildren)
  ```

- **Basename-only filtering**
  Patterns apply to the filename only (not directories). For path globs like `docs/*.md`, use `-R` and a pattern like `md` or `*.md`.

Notes:
- **Precedence**: CLI options override environment variables, which override defaults.

Timezone: default is your machine’s local timezone. Override via env var `FTL_TZ` (e.g., `FTL_TZ=Asia/Tokyo ftime md`).

### Git-only details

Internals follow Git porcelain-friendly plumbing with null-delimited paths:

- Changed in worktree: `git -C "$dir" ls-files -z -m --`
- Staged changes: `git -C "$dir" diff --name-only -z --cached --`
- Untracked, honoring ignore rules: `git -C "$dir" ls-files -z -o --exclude-standard --`

Paths are correctly mapped when running from subdirectories.

### Configuration file (XDG)

- Path: `$XDG_CONFIG_HOME/ftime/config` or `~/.config/ftime/config`
- Format: simple `KEY=VALUE` lines. Unknown keys are ignored for safety.
- Allowed keys: `FTL_TZ`, `FTL_FORCE_COLOR`, `FTL_NO_COLOR`, `FTL_NO_TIME_COLOR`, `FTL_ACTIVE_HOURS`, `FTL_RECENT_HOURS`.
- Precedence: CLI > environment > config file > defaults.

Example `~/.config/ftime/config`:

```ini
FTL_TZ=UTC
FTL_ACTIVE_HOURS=4
FTL_RECENT_HOURS=24
```

<details>
  <summary><strong>Display customization (optional)</strong></summary>

## Color

- Auto on TTY
- Force ON for pipes/pagers: `FTL_FORCE_COLOR=1 ftime | less -R`
- Turn OFF all colors: `NO_COLOR=1` or `FTL_NO_COLOR=1`
- Follows the NO_COLOR informal standard; `FTL_FORCE_COLOR=1` explicitly overrides `NO_COLOR` when needed.

### What is colorized
- **Modified** and **Created** time columns are colorized by recency (active/recent/old)
- Name column is colorized by type/extension
- Mark column shows `+` in yellow when a file was modified since creation (blank otherwise)

### Time‑based coloring (configurable)
- Active (default 4h): bright green
- Recent (default 24h): default color (no extra tint)
- Old (older than recent threshold; default 24h+): gray
- Disable time coloring: `FTL_NO_TIME_COLOR=1`
- Configure thresholds: `FTL_ACTIVE_HOURS=4 FTL_RECENT_HOURS=24`

</details>

<details>
  <summary><strong>Environment variables (optional)</strong></summary>

### How to Use Environment Variables (Examples)

Prepend the variable to make a temporary, one-time setting for that command. This setting is not permanent. You can combine multiple variables.

```bash
# Change timezone to New York
FTL_TZ=America/New_York ftime

# Change the 'active' threshold to 1 hour
FTL_ACTIVE_HOURS=1 ftime

# Combine multiple variables
FTL_TZ=UTC FTL_RECENT_HOURS=48 ftime

 
```

### Environment Variables (Reference)
- `FTL_TZ`: override timezone (e.g., `Asia/Tokyo`)
- `FTL_FORCE_COLOR`: force color even when piping
- `NO_COLOR` / `FTL_NO_COLOR`: disable all color
- `FTL_NO_TIME_COLOR`: disable time‑based coloring only
- `FTL_ACTIVE_HOURS`, `FTL_RECENT_HOURS`: thresholds for recency coloring (in hours)
 

</details>

### Tips: Aliases

- Quick alias:
  ```bash
  alias f='ftime'
  ```
- Example to prefer relative time and time sort:
  ```bash
  alias ft='ftime -a -s time'
  ```

---

## Security / Limitations

- Creation time depends on filesystem/kernel/tools; it may show `-`.
- Filenames can contain control characters. Colors are added only to the name column. Use caution when copying colored output to places that interpret ANSI.
- macOS is supported when GNU tools are installed (e.g., via Homebrew). Default BSD `stat`/`date` are not supported; GNU features are required.

---

## License

This project is released under the MIT License. See `LICENSE`.
