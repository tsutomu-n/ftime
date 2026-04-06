# ftime v2.0.2 Release Notes

## Patch highlights

- Human TTY output now aligns columns by Unicode display width, so Japanese and other full-width names no longer skew `size` and `time`.
- Long names in the human view now truncate to a compact `~` form while preserving the file extension or trailing `/` where possible.
- `--plain` and `--json` continue to emit full names, so machine-readable output stays lossless.

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
