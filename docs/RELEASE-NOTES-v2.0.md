# ftime v2.0.0 Release Notes

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
