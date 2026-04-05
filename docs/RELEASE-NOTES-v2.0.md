# ftime v2.0.1 Release Notes

## Patch highlights

- Human TTY output now uses globally aligned `name  size  time` columns.
- Symlink rows keep the main columns aligned and move `-> target` to the suffix.
- Bucket colors were simplified for theme-safe scanning: `Active` stays strongest, `This Week` uses cyan, `History` falls back to the default foreground.

## Breaking changes

- `-a` now means hidden entries instead of History expansion.
- default output is now the human view.
- `--plain` adds the TSV contract.
- `Fresh` was removed.
- `--exclude-dots` and `--no-labels` were removed.

## Highlights

- hidden files stay visible by default, while hidden directories stay hidden.
- `--all-history` expands only the `History` bucket.
- `--files-only`, `--no-hints`, and `--color <auto|always|never>` were added.
- plain output is now `path<TAB>bucket<TAB>time`.
