# ftime Command Guide

This document is the task-oriented guide to `ftime` commands. For the strict output contract and validation rules, see [CLI.md](CLI.md).

## Quick reference

| Command | Use when | What changes |
| --- | --- | --- |
| `ftime` | Scan the current directory in the default human view | Shows buckets, size, time, and optional suffixes |
| `ftime [PATH]` | Scan another directory | Same output shape, different target folder |
| `ftime -a` | Show hidden directories too | Keeps hidden files visible and adds hidden directories |
| `ftime --hide-dots` | Remove all hidden entries | Hides hidden files, hidden directories, and hidden symlinks |
| `ftime --no-ignore` | Show ignored entries too | Disables built-in ignore rules and `.ftimeignore` handling |
| `ftime --ext rs,toml` | Focus on selected file extensions | Filters regular files by extension while keeping dirs/symlinks |
| `ftime --files-only` | Remove directories and symlinks | Leaves only regular files |
| `ftime --files-only --ext rs,toml` | Show only selected regular files | Combines file-only filtering with extension filtering |
| `ftime --all-history` | Expand the History bucket | Removes the default `History` preview limit |
| `ftime -A` | Inspect exact timestamps | Replaces relative times with local absolute timestamps |
| `ftime --no-hints` | Silence `[child: ...]` hints | Directory rows keep their bucket but lose child activity suffixes |
| `ftime --color never` | Force plain human output | Keeps the human view but strips ANSI color |
| `ftime --color always` | Keep color through pipes | Forces ANSI color in the human view |
| `ftime -I` | Enable Nerd Font icons | Adds bucket icons in builds with the `icons` feature |
| `ftime --plain` | Feed scripts with compact text | Emits `path<TAB>bucket<TAB>time` and removes headers, size, color, and hints |
| `ftime --plain -A` | Feed scripts with exact timestamps | Same TSV shape, but the `time` field becomes absolute |
| `ftime --json` | Feed scripts with structured output | Emits one JSON object per visible entry as JSON Lines |
| `ftime --check-update` | Check for a newer published release | Prints whether a newer GitHub release exists |
| `ftime --self-update` | Update the installed binary | Downloads and installs the latest published release |
| `ftime --help` | Show the CLI contract quickly | Prints usage, options, and validation constraints |
| `ftime --version` | Print the installed version | Emits the current binary version |

## Hidden-entry comparison

```text
$ ftime
Today (2)
  .env       312 B   2h
  README.md  8.4 KiB 3h

This Week (1)
  src/           —   2d [child: today]
```

```text
$ ftime -a
Today (2)
  .env       312 B   2h
  README.md  8.4 KiB 3h

This Week (3)
  .git/          —   1d [child: active]
  .cache/        —   1d
  src/           —   2d [child: today]
```

```text
$ ftime --hide-dots
Today (1)
  README.md  8.4 KiB 3h

This Week (1)
  src/           —   2d [child: today]
```

## Output-mode comparison

| Mode | Shape | Includes |
| --- | --- | --- |
| default human view | bucketed text | headers, `name`, `size`, `time`, optional suffix, optional color |
| `--plain` | `path<TAB>bucket<TAB>time` | no headers, no size, no color, no child hint |
| `--json` | JSON Lines | structured fields for scripts and tooling |

Examples:

```text
$ ftime --plain
README.md	today	2h
src	history	2026-04-01
```

```json
{"path":"README.md","bucket":"today","mtime":"2026-04-06T00:31:44Z","relative_time":"2h","is_dir":false,"is_symlink":false,"size":8601}
{"path":"src","bucket":"history","mtime":"2026-04-01T03:10:00Z","relative_time":"2026-04-01","is_dir":true,"is_symlink":false}
```

## Validation notes

- `--plain` and `--json` cannot be combined
- `-a` and `--hide-dots` cannot be combined
- Update commands cannot be combined with scan flags or `PATH`
- For the exact contract, human-row details, and full validation matrix, see [CLI.md](CLI.md)
