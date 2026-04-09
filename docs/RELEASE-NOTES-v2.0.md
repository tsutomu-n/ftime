# ftime v2.0.5 Release Notes

## Patch highlights

- The default human view now renders a stable `type | name | size | time` layout.
- `--self-update` docs now clearly distinguish GitHub Releases installs from `cargo install` installs.
- Static contract tests now lock the Windows PowerShell installer and uninstaller defaults, error guidance, and user-visible messages.
- Maintainer guidance now maps PowerShell installer changes to a dedicated contract test.

## Breaking changes

- `--no-hints` was removed and replaced by `--hints`.
- Human output no longer shows symlink targets in the default view.
- Human output now starts with a type label column.

## Highlights

- hidden files stay visible by default, while hidden directories stay hidden.
- `--all-history` expands only the `History` bucket.
- `--files-only`, `--hints`, and `--color <auto|always|never>` were added.
- plain output is now `path<TAB>bucket<TAB>time`.
