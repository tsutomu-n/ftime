# ftime v2.0.1 CLI Contract

`ftime` is a read-only CLI for Context Recovery in one folder.

## 1. Command Signature

```bash
ftime [PATH] [-a|--all] [--hide-dots] [--no-ignore] [--ext <csv>] [--files-only] [--all-history] [-A|--absolute] [--no-hints] [--plain|--json] [--color <auto|always|never>] [-I|--icons]
```

Default output is always the human view.

## 2. Core Behavior

- Scan only depth 1 of the target directory.
- Sort visible entries by `mtime` descending, then `name` ascending.
- Default hidden policy: hidden files and hidden symlinks stay visible, hidden directories stay hidden.
- Built-in ignore patterns are `.DS_Store` and `Thumbs.db`.
- `--ext` filters regular files only. Directories and symlinks stay visible unless `--files-only` is also set.

## 3. Flags

- `-a, --all`: show hidden files and hidden directories
- `--all-history`: expand the History bucket
- `--hide-dots`: hide all hidden entries
- `--no-ignore`: disable built-in ignore plus `FTIME_IGNORE`, `~/.ftimeignore`, and local `.ftimeignore`
- `--ext <csv>`: filter regular files by comma-separated extensions
- `--files-only`: only show regular files
- `-A, --absolute`: human/plain time column becomes `YYYY-MM-DD HH:MM:SS (UTC±HH:MM)`
- `--no-hints`: disable directory child hint calculation
- `--plain`: emit `path<TAB>bucket<TAB>time`
- `--json`: emit JSON Lines
- `--color <auto|always|never>`: human-output ANSI color control
- `-I, --icons`: enable icons only when the binary was built with the `icons` feature
- `--check-update`, `--self-update`: update flow commands

## 4. Validation Rules

- `--plain` and `--json` cannot be combined
- `-a` and `--hide-dots` cannot be combined
- `--json` rejects `--absolute`, `--all-history`, `--no-hints`, `--icons`, and explicit `--color`
- `--plain` rejects `--all-history`, `--no-hints`, `--icons`, and explicit `--color`
- Update commands cannot be combined with scan flags or `PATH`

## 5. Human Output

- Bucket order is `Active`, `Today`, `This Week`, `History`
- Preview limits are 20 / 20 / 20 / 5, unless `--all-history` is set
- Header shape is either `Active (3)` or `History (5/42)`
- Row structure is `<name>  <size>  <time>  <optional-suffix>`
- Columns align by Unicode display width, not raw character count
- Directories show `—` in the size column
- Directories end in `/`
- Human output may truncate long names to fit the name column; plain/json always keep the full value
- Symlinks keep `name  size  time` aligned and render `-> target` in the optional suffix
- child hint is advisory only and never changes bucket classification
- Empty state is `No matching entries`
- Optional footer for unreadable entries is `Skipped N unreadable entries`

## 6. Plain Output

- One line per visible entry
- Shape: `path<TAB>bucket<TAB>time`
- No color, no header, no size, no child hint
- Directories and symlinks use undecorated paths

## 7. JSON Lines

- One JSON object per visible entry
- Field order: `path`, `bucket`, `mtime`, `relative_time`, `is_dir`, `is_symlink`, optional `size`, optional `symlink_target`
- `mtime` is UTC RFC3339
- JSON Lines never include child hint, preview metadata, diagnostics footer, or label fields

## 8. Human Diagnostics

- Filters summary may appear after `No matching entries`
- `No matching entries`
- `Skipped N unreadable entries`

## 9. Environment

- `NO_COLOR` disables color only when `--color` is left at `auto`
- `FTIME_IGNORE` overrides the global ignore file path

## 10. Non-Goals

- Recursive search
- VCS state inspection
- Destructive actions
- Config files
