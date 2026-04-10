# ftime v2.0.7 Release Notes

## Patch highlights

- The default human view now renders a stable `type | name | size | time` layout.
- The Windows PowerShell installer now adds the install directory to the user PATH and updates the current session PATH fallback.
- Output contract coverage now locks the unreadable footer and JSON field order, including `symlink_target` placement.
- The release workflow now uses `gh` CLI for release creation and asset upload, removing the Node-based release action warning path.

## Breaking changes

- `--no-hints` was removed and replaced by `--hints`.
- Human output no longer shows symlink targets in the default view.
- Human output now starts with a type label column.

## Highlights

- hidden files stay visible by default, while hidden directories stay hidden.
- `--all-history` expands only the `History` bucket.
- `--files-only`, `--hints`, and `--color <auto|always|never>` were added.
- plain output is now `path<TAB>bucket<TAB>time`.
