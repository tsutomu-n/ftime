# ftime Maintainer Guide

This note is for maintainers. Public user-facing behavior still lives in `docs/CLI.md`, `README.md`, and the release notes.

## Sync Order

When behavior changes, update in this order:

`tests -> code/help -> docs/CLI.md -> README/translated docs -> demo -> release notes -> cargo check/test`

Use `docs/CLI.md` as the canonical contract for flags, validation rules, and output shape.

## Canonical Public Surface

Keep these files in sync whenever public behavior changes:

- `README.md`
- `docs/COMMANDS.md`
- `docs/INSTALL.md`
- `docs/CLI.md`
- `docs/CLI-ja.md`
- `docs/README-ja.md`
- `docs/README-zh.md`
- `docs/USER-GUIDE-ja.md`
- `docs/ftime-overview-ja.md`
- `docs/RELEASE-NOTES-v2.0.md`
- `demo/README.md`
- `demo/tapes/demo_ftime.tape`

`tests/release_docs.rs` is the public drift check for this set.

## Demo and Asset Rules

- Update `demo/README.md` and `demo/tapes/demo_ftime.tape` when command examples or public output examples change.
- Run `demo/render-assets.sh` only for human-visible output changes.
- Help text changes, docs-only edits, and invisible internal refactors do not require demo asset regeneration.
- If `assets/demo_ftime.gif` or `assets/demo_ftime.mp4` are regenerated, verify the rendered scene still matches the current bucket contract and flag defaults.

## Release and Version Rules

- When bumping the package version, update `Cargo.toml`, `docs/CLI.md`, and `docs/RELEASE-NOTES-v2.0.md` together.
- Keep docs tests aligned with the current version and current release notes wording.
- Before finishing any behavior change, run `cargo check` and `cargo test --quiet`.
