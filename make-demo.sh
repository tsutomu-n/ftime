#!/usr/bin/env bash
# make-demo.sh - Create a small demo directory with mixed CJK + ASCII files
# SPDX-License-Identifier: MIT
set -euo pipefail

usage(){ cat <<'H'
Usage:
  ./make-demo.sh [--force] [--reset]

Options:
  --force   Overwrite existing demo files (re-create placeholders)
  --reset   Remove the demo directory before creating (fresh start)

This creates ./demo with English (en), Japanese (ja), Simplified Chinese (zh)
content and a few extensions (.md/.py/.ts/.svelte/.log/.png) for SVG demos.
H
}

force=0
reset=0
for a in "$@"; do
  case "$a" in
    -h|--help) usage; exit 0;;
    --force) force=1;;
    --reset) reset=1;;
    *) echo "Unknown option: $a"; usage; exit 1;;
  esac
done

root=$(pwd)
demo="$root/demo"

if (( reset==1 )) && [ -d "$demo" ]; then
  echo "[reset] removing $demo"
  rm -rf -- "$demo"
fi

mkdir -p "$demo"/{docs,addon,api,en}

write(){ # write only if missing or --force
  local path="$1"; shift
  if (( force==1 )) || [ ! -f "$path" ]; then
    printf "%s\n" "$*" > "$path"
    echo "[write] $path"
  else
    echo "[skip]  $path (exists)"
  fi
}
touch_if(){ [ -f "$1" ] || { : > "$1"; echo "[touch] $1"; }; }


# Mixed-language code/data
write "$demo/日本語のメモ.md"       "メモです"
write "$demo/レポート2025.md"       "レポート"
write "$demo/使い方.md"           "使い方"
write "$demo/docs/HowTo.md"       "HowTo"
write "$demo/en/howto.md"         "How to use."
write "$demo/script.py"           "print('hello')"

write "$demo/app.ts"               "export const x=1;"
write "$demo/コンポーネント.svelte" "<script>export let x</script>"

touch_if "$demo/api_server.log"
touch_if "$demo/エラー.log"

touch_if "$demo/スクリーンショット.png"
touch "$demo/addon/.keep" "$demo/api/.keep"

# Set a variety of modification times to demonstrate color grading
echo "[touch] Setting a variety of modification times for demo purposes..."

# A few seconds ago
write "$demo/NEW.md" "A very new file created just now."
sleep 1
touch -m "$demo/app.ts"

# A few hours ago
touch -m -d "2 hours ago" "$demo/script.py"
touch -m -d "8 hours ago" "$demo/コンポーネント.svelte"

# A few days ago

touch -m -d "5 days ago" "$demo/docs/HowTo.md"

# A few weeks ago
touch -m -d "2 weeks ago" "$demo/レポート2025.md"


# Very old
write "$demo/old.md" "Old file from 10 months ago"
touch -d "10 months ago" "$demo/old.md"

# An ancient file
write "$demo/ancient.log" "Ancient log file from 2 years ago"
touch -d "2 years ago" "$demo/ancient.log"

cat <<'NOTE'

[done] Demo directory prepared at ./demo
Try:
  cd demo && ftime
  cd demo && ftime md
  cd demo && ftime .log
  cd demo && FTL_TZ=Asia/Tokyo ftime

To record the four demos (from repo root):
  make rec-basic    # (cd demo) -> ftime -> exit
  make rec-pattern  # (cd demo) -> ftime md / .log / md py -> exit
  make rec-dir      # (cd repo) -> ftime demo md -> exit
  make rec-tz       # (cd demo) -> ftime / FTL_TZ=Asia/Tokyo ftime -> exit
  make demos        # builds media/*.min.svg

NOTE

