#!/usr/bin/env bash
# Demo: ftime overview (natural speed; paced by short sleeps)
set -euo pipefail
P=${PACE:-1.2}
pa(){ sleep "${1:-$P}"; }

# Resolve ftime command (PATH or local script)
repo_root=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
FTIME_CMD="ftime"
if ! command -v ftime >/dev/null 2>&1; then
  if [[ -x "$repo_root/ftime-list.sh" ]]; then
    FTIME_CMD="$repo_root/ftime-list.sh"
  else
    echo "Error: 'ftime' not found on PATH and '$repo_root/ftime-list.sh' not executable" >&2
    echo "Hint: ln -sf \"$repo_root/ftime-list.sh\" ~/.local/bin/ftime" >&2
    exit 1
  fi
fi

demo_dir="$repo_root/demo"
if [[ ! -d "$demo_dir" ]]; then
  echo "Error: demo directory not found: $demo_dir" >&2
  echo "Run: ./make-demo.sh (from repo root)" >&2
  exit 1
fi

printf -- "ftime Overview\n"
printf -- "- Lists files/directories by most recent update\n"
printf -- "- Works with any language or file extension\n\n"
pa 1.0
clear

cd "$(dirname "$0")/../demo"
printf -- "\n$ ftime\n"
pa 0.6
FTL_FORCE_COLOR=1 "$FTIME_CMD"
pa 1.0

printf -- "\n$ touch NEW.md\n"
pa 0.4
touch NEW.md
pa 0.4

printf -- "\n$ ftime\n"
pa 0.6
FTL_FORCE_COLOR=1 "$FTIME_CMD"
pa 0.8

printf -- "\n$ echo 'update' >> en/howto.md\n"
pa 0.4
echo 'update' >> en/howto.md
pa 0.4

printf -- "\n$ ftime\n"
pa 0.6
FTL_FORCE_COLOR=1 "$FTIME_CMD"
pa 1.6
