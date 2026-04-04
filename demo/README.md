# Demo Capture Workspace

`ftime`: time-bucketed directory listing for fast context recovery.

This directory exists only for public demo capture.

Use it to generate a clean scene for GIF / MP4 recording without exposing repo-local noise such as maintainer files, agent scratch directories, or release tooling.

## Why this exists

- keep the recording scene separate from the real repository root
- show `ftime` in a folder that looks like a normal project
- make sure `[child: active]` and `[child: today]` appear on purpose

## Generate a fresh scene

```bash
./demo/setup-scene.sh
```

The script creates a new output directory outside the repo by default and prints the generated path.

You can also choose the output location explicitly:

```bash
./demo/setup-scene.sh /tmp/ftime-demo-scene
```

To rebuild the tracked public assets:

```bash
./demo/render-assets.sh
```

## Recommended capture flow

```bash
cd /path/to/generated/demo-scene
ftime
ftime -a
ftime --json | jq -r '.path'
```

Recommended clip structure:

1. `ftime`
2. `ftime -a`
3. `ftime --json | jq -r '.path'`

This gives you:

- bucketed TTY output
- directory hints such as `[child: active]` and `[child: today]`
- a visible `History` expansion on `ftime -a`
- one quick contrast shot where JSON stays machine-oriented and does not show child hints

## Intended visual shape

The generated scene is tuned so the first two TTY commands typically show something close to this:

```text
Active
  • Cargo.toml | 1.2 KiB | 12m ago
Today
  • docs/ | - | 3h ago [child: active]
  • README.md | 1.5 KiB | 4h ago
This Week
  • target/ | - | 2d ago [child: active]
History
  (22 files hidden)

Then `ftime -a` reveals the old entries:

```text
History
  • tests/ | - | 10d ago [child: today]
  • history-note-01.md | 27 B | 11d ago
  ...
```

Use this workspace for capture only. The real repository root is intentionally not the recording surface.
