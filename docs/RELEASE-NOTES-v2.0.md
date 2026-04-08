# ftime v2.0.4 Release Notes

## Patch highlights

- The default human view now renders a stable `type | name | size | time` layout.
- Human rows now show `[FIL]`, `[DIR]`, and `[LNK]`, while directories and symlinks use `<dir>` / `<lnk>` placeholders in the size column.
- Directory child hints are now opt-in through `--hints`, and the docs were updated to match the new contract.

## Breaking changes

- `--no-hints` was removed and replaced by `--hints`.
- Human output no longer shows symlink targets in the default view.
- Human output now starts with a type label column.

## Highlights

- hidden files stay visible by default, while hidden directories stay hidden.
- `--all-history` expands only the `History` bucket.
- `--files-only`, `--hints`, and `--color <auto|always|never>` were added.
- plain output is now `path<TAB>bucket<TAB>time`.
