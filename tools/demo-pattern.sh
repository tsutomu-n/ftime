#!/usr/bin/env bash
# Demo: pattern filtering with ftime
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

printf -- "ftime: Filtering with Patterns\n"
printf -- "- Focus on specific extensions or strings\n\n"
pa 1.0
clear

cd "$(dirname "$0")/../demo"
printf -- "\n$ ftime md\n"
pa 0.6
FTL_FORCE_COLOR=1 "$FTIME_CMD" md
pa 0.8

printf -- "\n$ touch en/new.md\n"
pa 0.4
touch en/new.md
pa 0.4

printf -- "\n$ ftime md\n"
pa 0.6
FTL_FORCE_COLOR=1 "$FTIME_CMD" md
pa 0.8

printf -- "\n$ ftime .log\n"
pa 0.6
FTL_FORCE_COLOR=1 "$FTIME_CMD" .log
pa 0.8

printf -- "\n$ printf 'x' >> api_server.log\n"
pa 0.4
printf 'x' >> api_server.log
pa 0.4

printf -- "\n$ ftime .log\n"
pa 0.6
FTL_FORCE_COLOR=1 "$FTIME_CMD" .log
pa 0.8

printf -- "\n$ ftime md py\n"
pa 0.6
FTL_FORCE_COLOR=1 "$FTIME_CMD" md py
pa 0.8

printf -- "\n$ : > script2.py\n"
pa 0.4
: > script2.py
pa 0.4

printf -- "\n$ ftime md py\n"
pa 0.6
FTL_FORCE_COLOR=1 "$FTIME_CMD" md py
pa 1.6
