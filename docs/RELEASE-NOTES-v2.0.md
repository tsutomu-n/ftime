# ftime v2.0.3 Release Notes

## Patch highlights

- The root README is now a lighter landing page focused on value, common examples, and the shortest install path.
- Deep command comparisons moved to `docs/COMMANDS.md`, which now serves as the canonical task-oriented command guide.
- Install, update, and uninstall instructions moved to `docs/INSTALL.md`, reducing duplication across README variants.

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
