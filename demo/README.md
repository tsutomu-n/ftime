# Demo Capture Workspace

`ftime`: human-first directory listing for fast context recovery.

This directory exists only for public demo capture.

## Recommended capture flow

```bash
cd /path/to/generated/demo-scene
ftime
ftime -a
ftime --all-history
ftime --plain
ftime --json | jq -r '.path'
```
