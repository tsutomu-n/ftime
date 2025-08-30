#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
set -euo pipefail

# ftime-list.sh
# Show: modified(mtime), created(birth), name
# Timezone: local by default; override with env FTL_TZ (e.g., Asia/Tokyo)
# Notes: creation time may be unavailable on some filesystems (shows '-')

# ---- Small helpers & sanity checks ----
die() { echo "Error: $*" >&2; exit 1; }
have() { command -v "$1" >/dev/null 2>&1; }

print_version() {
  # Print version from VERSION file (same dir as script) if available
  local dir version_file
  dir=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
  version_file="$dir/VERSION"
  if [[ -f "$version_file" ]]; then
    cat "$version_file"
  else
    echo "ftime (version unknown)"
  fi
}

ensure_requirements() {
  # Ensure required GNU tools & features are available
  for cmd in date stat find sort; do
    have "$cmd" || die "Required command not found: $cmd"
  done
  # find -printf (GNU findutils)
  if ! find . -maxdepth 0 -printf '' >/dev/null 2>&1; then
    die "This script requires GNU findutils (find with -printf)"
  fi
  # sort -z (GNU sort)
  if ! printf 'a\0b\0' | LC_ALL=C sort -z >/dev/null 2>&1; then
    die "This script requires GNU sort with -z support"
  fi
  # stat -c %W (GNU coreutils)
  if ! stat -c '%W' -- "$0" >/dev/null 2>&1; then
    die "This script requires GNU coreutils stat with -c '%W' support"
  fi
  # date -d (GNU date)
  if ! date -d @0 +%s >/dev/null 2>&1; then
    die "This script requires GNU date with -d support"
  fi
}

usage() {
  cat <<'EOF'
ftime - list files with modified/created times
Usage: ftime [OPTIONS] [DIR] [PATTERN ...]
Options: -s/--sort time|name  -r/--reverse  -R/--recursive  -d N/--max-depth=N  -a/--age  -V/--version
Try:   ftime --help (details)  |  ftime -s time -R -d 2 md (*.md only)
EOF
}

# Format seconds into a short relative string (e.g., 45s, 12m, 3h, 5d, 2w, 1y)
fmt_relative() {
  local s=$1
  if (( s < 60 )); then
    echo "${s}s"
  elif (( s < 3600 )); then
    echo "$(( s / 60 ))m"
  elif (( s < 86400 )); then
    echo "$(( s / 3600 ))h"
  elif (( s < 604800 )); then
    echo "$(( s / 86400 ))d"
  elif (( s < 31536000 )); then
    echo "$(( s / 604800 ))w"
  else
    echo "$(( s / 31536000 ))y"
  fi
}

full_usage() {
  cat <<'EOF'
ftime - list files with modified/created times

What it shows
  modified  created  name
  - modified: when file content was last changed (MM-DD HH:MM)
  - created : when file was created ('-' if not supported by your filesystem)

How to use
  ftime [OPTIONS] [DIR] [PATTERN ...]
  - OPTIONS:
      -s, --sort time|name   Sort by modified time (default: newest first) or by name
      -r, --reverse          Reverse the sort order
      -R, --recursive        Recurse into subdirectories
      -d, --max-depth N      Limit recursion depth to N (requires -R)
      -a, --age              Show relative times instead of absolute (e.g., 5m, 3h). Env: FTL_RELATIVE=1
      -V, --version          Show version and exit
  - DIR: directory to scan (default: current directory)
  - PATTERN: filename filters (OR). Simple rules:
      * contains * or ?   -> use as-is (glob)
      * starts with '.'   -> *<token>   (example: .log -> *.log)
      * otherwise         -> *.<token>  (example: md   -> *.md)

Timezone
  - Default: your machine's local timezone
  - Override: set env var FTL_TZ (example: FTL_TZ=Asia/Tokyo ftime md)

Color (time-based + file name)
  - Auto on TTY
  - Force on:   FTL_FORCE_COLOR=1
  - Turn off:   NO_COLOR=1   or   FTL_NO_COLOR=1
  - With pager: FTL_FORCE_COLOR=1 ftime | less -R
  
Time-based coloring:
  - Active (4h):   bright green + edit marks (+)
  - Recent (24h):  white + edit marks if modified
  - Old (7d+):     gray
  - Disable:       FTL_NO_TIME_COLOR=1
  - Customize:     FTL_ACTIVE_HOURS=4 FTL_RECENT_HOURS=24

Examples
  ftime                      # list everything in the current dir
  ftime md                   # only *.md
  ftime -s time              # sort by modified time (newest first)
  ftime -s time -r           # sort by modified time (oldest first)
  ftime -R -d 2 md           # recurse up to depth 2, only *.md
  ftime docs md              # *.md inside ./docs
  ftime '*.test.*'           # explicit glob pattern

Notes
  - Requires GNU coreutils (stat, date)
  - Also requires GNU findutils (find with -printf/-print0) and GNU sort (-z)
  - created time may be unavailable on some filesystems (shows '-')
EOF
}

# Handle help
case "${1:-}" in
  -h|--help|help)
    full_usage; exit 0 ;;
  --help-short)
    usage; exit 0 ;;
  -V|--version)
    print_version; exit 0 ;;
esac

# Ensure required tools/features (skip for help/version handled above)
ensure_requirements

# ---- Option parsing ----
sort_key="name"
reverse=0
recursive=0
max_depth=""
relative=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --)
      shift; break ;;
    -s|--sort)
      val="${2:-}"
      if [[ -z "$val" || "$val" == -* ]]; then
        echo "Error: --sort requires a value (time|name)" >&2
        usage >&2; exit 1
      fi
      sort_key="$val"; shift ;;
    --sort=*)
      sort_key="${1#*=}" ;;
    -r|--reverse)
      reverse=1 ;;
    -R|--recursive)
      recursive=1 ;;
    -d)
      val="${2:-}"
      if [[ -z "$val" || "$val" == -* ]]; then
        echo "Error: --max-depth expects a positive integer" >&2
        exit 1
      fi
      max_depth="$val"; shift ;;
    --max-depth=*)
      max_depth="${1#*=}" ;;
    --max-depth)
      val="${2:-}"
      if [[ -z "$val" || "$val" == -* ]]; then
        echo "Error: --max-depth expects a positive integer" >&2
        exit 1
      fi
      max_depth="$val"; shift ;;
    -a|--age)
      relative=1 ;;
    -V|--version)
      print_version; exit 0 ;;
    -h|--help|help)
      full_usage; exit 0 ;;
    --help-short)
      usage; exit 0 ;;
    -* )
      echo "Error: Unknown option '$1'" >&2
      usage >&2; exit 1 ;;
    *)
      break ;;
  esac
  shift
done

# Validate options
case "$sort_key" in
  time|name) ;;
  *) echo "Error: Unknown sort key '$sort_key'. Use: time, name" >&2; exit 1 ;;
esac
if [[ -n "$max_depth" ]]; then
  if ! [[ "$max_depth" =~ ^[0-9]+$ ]]; then
    echo "Error: --max-depth expects a positive integer" >&2; exit 1
  fi
  if [[ "$recursive" -ne 1 ]]; then
    echo "Error: --max-depth requires --recursive (-R)" >&2; exit 1
  fi
fi

# Env override for relative time display
if [[ -n "${FTL_RELATIVE:-}" && "${FTL_RELATIVE}" != "0" ]]; then
  relative=1
fi

dir="."
# arg parsing: first arg may be a directory; remaining args are name filters
if [[ $# -ge 1 ]]; then
  # Check if first arg looks like a directory (contains / or is a known dir)
  if [[ "$1" == */* ]] || [[ "$1" == "." ]] || [[ "$1" == ".." ]] || [[ -d "$1" ]]; then
    if [[ ! -d "$1" ]]; then
      echo "Error: Directory '$1' not found" >&2
      echo "Try:   ftime  (current directory)" >&2
      usage >&2
      exit 1
    elif [[ ! -r "$1" ]]; then
      echo "Error: Cannot read directory '$1' (permission denied)" >&2
      exit 1
    fi
    dir="$1"; shift
  fi
fi

# build filter patterns: token "md" -> "*.md"; token starting with '.' -> "*<token>"; wildcard tokens pass through
filters=()
for tok in "$@"; do
  [[ -z "$tok" ]] && continue
  case "$tok" in
    *'*'*|*'?'*) filters+=("$tok") ;;
    .* )         filters+=("*${tok}") ;;
    * )          filters+=("*.${tok}") ;;
  esac
done

# 時間ベースの色分け設定
now=$(date +%s)
active_hours="${FTL_ACTIVE_HOURS:-4}"
recent_hours="${FTL_RECENT_HOURS:-24}"

# Validate env configuration
# - FTL_TZ validity: if invalid, warn and ignore
if [[ -n "${FTL_TZ:-}" ]]; then
  if ! TZ="$FTL_TZ" date +%s >/dev/null 2>&1; then
    echo "Warning: Invalid timezone in FTL_TZ ('$FTL_TZ'); ignoring." >&2
    unset FTL_TZ
  fi
fi
# - Hours numeric
if ! [[ "$active_hours" =~ ^[0-9]+$ ]]; then
  echo "Warning: FTL_ACTIVE_HOURS must be a non-negative integer; using default 4" >&2
  active_hours=4
fi
if ! [[ "$recent_hours" =~ ^[0-9]+$ ]]; then
  echo "Warning: FTL_RECENT_HOURS must be a non-negative integer; using default 24" >&2
  recent_hours=24
fi
if (( active_hours > recent_hours )); then
  echo "Warning: FTL_ACTIVE_HOURS ($active_hours) > FTL_RECENT_HOURS ($recent_hours); colors may look odd" >&2
fi
active_seconds=$((active_hours * 3600))
recent_seconds=$((recent_hours * 3600))

# Header: include a fixed 1-char mark column (blank) BEFORE MODIFIED
printf "%s %-12s %-12s %s\n" " " "MODIFIED" "CREATED" "NAME"

# Build find arguments
find_args=("$dir" -mindepth 1)
if [[ $recursive -eq 1 ]]; then
  if [[ -n "$max_depth" ]]; then
    find_args+=( -maxdepth "$((max_depth + 1))" )
  fi
else
  find_args+=( -maxdepth 1 )
fi

if [[ "$sort_key" == "time" ]]; then
  # Time sort: newest first by default; --reverse flips to oldest first
  sort_flags=(-z -t ' ' -k1,1n)
  if [[ $reverse -eq 0 ]]; then
    sort_flags+=(-r)
  fi
  LC_ALL=C find "${find_args[@]}" -printf '%T@ %P\0' \
    | LC_ALL=C sort "${sort_flags[@]}" \
    | while IFS= read -r -d '' rec; do
      f="${rec#* }"
      # apply filters early (basename only) to avoid unnecessary stat calls
      if (( ${#filters[@]} > 0 )); then
        keep=0
        base_f="${f##*/}"
        for pat in "${filters[@]}"; do
          if [[ "$base_f" == $pat ]]; then keep=1; break; fi
        done
        [[ $keep -eq 0 ]] && continue
      fi
      p="$dir/$f"
      m=$(stat -c '%Y' -- "$p" 2>/dev/null || echo -1)
      b=$(stat -c '%W' -- "$p" 2>/dev/null || echo -1)
      if [[ -n "${FTL_TZ:-}" ]]; then
        mt=$(TZ="$FTL_TZ" date -d "@${m}" +'%m-%d %H:%M' 2>/dev/null || echo '-')
      else
        mt=$(date -d "@${m}" +'%m-%d %H:%M' 2>/dev/null || echo '-')
      fi
      if [[ "$b" -gt 0 ]] 2>/dev/null; then
        if [[ -n "${FTL_TZ:-}" ]]; then
          bt=$(TZ="$FTL_TZ" date -d "@${b}" +'%m-%d %H:%M' 2>/dev/null || echo '-')
        else
          bt=$(date -d "@${b}" +'%m-%d %H:%M' 2>/dev/null || echo '-')
        fi
      else
        bt='-'
      fi
      # Relative time substitution if enabled
      if [[ $relative -eq 1 ]]; then
        if [[ "$m" -ge 0 ]] 2>/dev/null; then
          mt=$(fmt_relative $(( now - m )))
        else
          mt='-'
        fi
        if [[ "$b" -gt 0 ]] 2>/dev/null; then
          bt=$(fmt_relative $(( now - b )))
        else
          bt='-'
        fi
      fi
      
      # 時間経過による色分け
      time_color=""
      created_time_color=""
      edit_mark=""
      if [[ "$m" -ge 0 ]] 2>/dev/null; then
        age=$((now - m))
        if [[ $age -lt $active_seconds ]]; then
          time_color="\033[32;1m"  # 明るい緑
        elif [[ $age -lt $recent_seconds ]]; then
          time_color=""  # 白（デフォルト）
        else
          time_color="\033[90m"  # グレー
        fi

        if [[ "$b" -gt 0 ]] 2>/dev/null; then
            age_b=$((now - b))
            if [[ $age_b -lt $active_seconds ]]; then
                created_time_color="\033[32;1m"
            elif [[ $age_b -lt $recent_seconds ]]; then
                created_time_color=""
            else
                created_time_color="\033[90m"
            fi
        fi
        
        # 編集マークの追加（内容は'+'。色は表示時に付与）
        if [[ "$b" -gt 0 && "$m" != "$b" ]] 2>/dev/null; then
          edit_mark="+"
        fi
      fi
      
      # apply filters (if any). Match on basename only.
      if (( ${#filters[@]} > 0 )); then
        keep=0
        base_f="${f##*/}"
        for pat in "${filters[@]}"; do
          if [[ "$base_f" == $pat ]]; then keep=1; break; fi
        done
        [[ $keep -eq 0 ]] && continue
      fi

      name="$f"
      color_ok=0
      if [[ -n "${FTL_FORCE_COLOR:-}" ]]; then
        color_ok=1
      elif [[ -t 1 && -z "${NO_COLOR:-}" && -z "${FTL_NO_COLOR:-}" ]]; then
        color_ok=1
      fi
      if [[ $color_ok -eq 1 ]]; then
        esc=$'\033'
        if [[ -d "$p" ]]; then
          name="${esc}[1;34m${name}${esc}[0m"               # dir: bold blue
        else
          base="${f##*/}"; base_lc=${base,,}
          ext=${f##*.}; ext=${ext,,}
          # Special filenames without extensions
          case "$base_lc" in
            makefile|gnumakefile|cmakelists.txt|dockerfile|gemfile|rakefile)
              name="${esc}[34m${name}${esc}[0m";;           # code/build
          esac
          case "$ext" in
            # Shell / scripts
            sh|bash|zsh|fish|ps1|psm1|nu|bat|cmd)
              name="${esc}[1;32m${name}${esc}[0m";;
            # Python family
            py|ipynb)
              name="${esc}[36m${name}${esc}[0m";;
            # Web / frontend
            js|mjs|cjs|jsx|ts|tsx|vue|svelte|astro|css|scss|less|html|htm)
              name="${esc}[36m${name}${esc}[0m";;
            # General programming languages
            rs|go|c|h|hh|hpp|hxx|cpp|cxx|java|kt|kts|cs|swift|scala|sc|dart|zig|nim|hs|ex|exs|erl|rb|php|jl|lua|pl|pm|r|clj|cljs|cljc|edn)
              name="${esc}[34m${name}${esc}[0m";;
            # Data / config / markup
            md|markdown)
              name="${esc}[35m${name}${esc}[0m";;
            yml|yaml|json|jsonc|toml|ini|conf|cfg|env|dotenv|xml|sql|gql|graphql|proto|gradle)
              name="${esc}[33m${name}${esc}[0m";;
            # Images / media
            png|jpg|jpeg|gif|svg|webp|bmp|tiff|ico|mp4|mkv|mov|webm|mp3|wav|flac)
              name="${esc}[96m${name}${esc}[0m";;
            # Archives / packages
            zip|gz|bz2|xz|7z|rar|tar|tgz|tbz|txz|deb|rpm|apk)
              name="${esc}[31m${name}${esc}[0m";;
            # Logs / locks
            log|lock)
              name="${esc}[90m${name}${esc}[0m";;
          esac
        fi
      fi
      # 表示（時間色と編集マークを適用）
      # Prepare 1-char mark column (space if no mark)
      mark_col=" "
      if [[ -n "$edit_mark" ]]; then
        if [[ -n "${FTL_NO_TIME_COLOR:-}" || $color_ok -eq 0 ]]; then
          mark_col="+"
        else
          mark_col="${esc}[33m+${esc}[0m"
        fi
      fi
      if [[ -n "${FTL_NO_TIME_COLOR:-}" || $color_ok -eq 0 ]]; then
        printf "%s %-12s %-12s %s\n" "$mark_col" "$mt" "$bt" "$name"
      else
        # 色を付けて表示（表示が崩れないように調整）
        printf '%s ' "$mark_col"
        printf "${time_color}%-12s${esc}[0m " "$mt"
        printf "${created_time_color}%-12s${esc}[0m " "$bt"
        printf '%s\n' "$name"
      fi
    done
else
  # Name sort (default). Respect --reverse for descending name order.
  name_sort_flags=(-z)
  if [[ $reverse -eq 1 ]]; then
    name_sort_flags+=(-r)
  fi
  LC_ALL=C find "${find_args[@]}" -printf '%P\0' \
    | LC_ALL=C sort "${name_sort_flags[@]}" \
    | while IFS= read -r -d '' f; do
      # apply filters early (basename only) to avoid unnecessary stat calls
      if (( ${#filters[@]} > 0 )); then
        keep=0
        base_f="${f##*/}"
        for pat in "${filters[@]}"; do
          if [[ "$base_f" == $pat ]]; then keep=1; break; fi
        done
        [[ $keep -eq 0 ]] && continue
      fi
      p="$dir/$f"
      m=$(stat -c '%Y' -- "$p" 2>/dev/null || echo -1)
      b=$(stat -c '%W' -- "$p" 2>/dev/null || echo -1)
      if [[ -n "${FTL_TZ:-}" ]]; then
        mt=$(TZ="$FTL_TZ" date -d "@${m}" +'%m-%d %H:%M' 2>/dev/null || echo '-')
      else
        mt=$(date -d "@${m}" +'%m-%d %H:%M' 2>/dev/null || echo '-')
      fi
      if [[ "$b" -gt 0 ]] 2>/dev/null; then
        if [[ -n "${FTL_TZ:-}" ]]; then
          bt=$(TZ="$FTL_TZ" date -d "@${b}" +'%m-%d %H:%M' 2>/dev/null || echo '-')
        else
          bt=$(date -d "@${b}" +'%m-%d %H:%M' 2>/dev/null || echo '-')
        fi
      else
        bt='-'
      fi

      # Relative time substitution if enabled
      if [[ $relative -eq 1 ]]; then
        if [[ "$m" -ge 0 ]] 2>/dev/null; then
          mt=$(fmt_relative $(( now - m )))
        else
          mt='-'
        fi
        if [[ "$b" -gt 0 ]] 2>/dev/null; then
          bt=$(fmt_relative $(( now - b )))
        else
          bt='-'
        fi
      fi

      # 時間経過による色分け
      time_color=""
      created_time_color=""
      edit_mark=""
      if [[ "$m" -ge 0 ]] 2>/dev/null; then
        age=$((now - m))
        if [[ $age -lt $active_seconds ]]; then
          time_color="\033[32;1m"  # 明るい緑
        elif [[ $age -lt $recent_seconds ]]; then
          time_color=""  # 白（デフォルト）
        else
          time_color="\033[90m"  # グレー
        fi

        if [[ "$b" -gt 0 ]] 2>/dev/null; then
            age_b=$((now - b))
            if [[ $age_b -lt $active_seconds ]]; then
                created_time_color="\033[32;1m"
            elif [[ $age_b -lt $recent_seconds ]]; then
                created_time_color=""
            else
                created_time_color="\033[90m"
            fi
        fi

        # 編集マークの追加（内容は'+'。色は表示時に付与）
        if [[ "$b" -ge 0 && "$m" != "$b" ]] 2>/dev/null; then
          edit_mark="+"
        fi
      fi


      name="$f"
      color_ok=0
      if [[ -n "${FTL_FORCE_COLOR:-}" ]]; then
        color_ok=1
      elif [[ -t 1 && -z "${NO_COLOR:-}" && -z "${FTL_NO_COLOR:-}" ]]; then
        color_ok=1
      fi
      if [[ $color_ok -eq 1 ]]; then
        esc=$'\033'
        if [[ -d "$p" ]]; then
          name="${esc}[1;34m${name}${esc}[0m"               # dir: bold blue
        else
          base="${f##*/}"; base_lc=${base,,}
          ext=${f##*.}; ext=${ext,,}
          # Special filenames without extensions
          case "$base_lc" in
            makefile|gnumakefile|cmakelists.txt|dockerfile|gemfile|rakefile)
              name="${esc}[34m${name}${esc}[0m";;           # code/build
          esac
          case "$ext" in
            # Shell / scripts
            sh|bash|zsh|fish|ps1|psm1|nu|bat|cmd)
              name="${esc}[1;32m${name}${esc}[0m";;
            # Python family
            py|ipynb)
              name="${esc}[36m${name}${esc}[0m";;
            # Web / frontend
            js|mjs|cjs|jsx|ts|tsx|vue|svelte|astro|css|scss|less|html|htm)
              name="${esc}[36m${name}${esc}[0m";;
            # General programming languages
            rs|go|c|h|hh|hpp|hxx|cpp|cxx|java|kt|kts|cs|swift|scala|sc|dart|zig|nim|hs|ex|exs|erl|rb|php|jl|lua|pl|pm|r|clj|cljs|cljc|edn)
              name="${esc}[34m${name}${esc}[0m";;
            # Data / config / markup
            md|markdown)
              name="${esc}[35m${name}${esc}[0m";;
            yml|yaml|json|jsonc|toml|ini|conf|cfg|env|dotenv|xml|sql|gql|graphql|proto|gradle)
              name="${esc}[33m${name}${esc}[0m";;
            # Images / media
            png|jpg|jpeg|gif|svg|webp|bmp|tiff|ico|mp4|mkv|mov|webm|mp3|wav|flac)
              name="${esc}[96m${name}${esc}[0m";;
            # Archives / packages
            zip|gz|bz2|xz|7z|rar|tar|tgz|tbz|txz|deb|rpm|apk)
              name="${esc}[31m${name}${esc}[0m";;
            # Logs / locks
            log|lock)
              name="${esc}[90m${name}${esc}[0m";;
          esac
        fi
      fi
      # 表示（時間色と編集マークを適用）: 固定1桁のマーク列を使って揃える
      mark_col=" "
      if [[ -n "$edit_mark" ]]; then
        if [[ -n "${FTL_NO_TIME_COLOR:-}" || $color_ok -eq 0 ]]; then
          mark_col="+"
        else
          mark_col="${esc}[33m+${esc}[0m"
        fi
      fi
      if [[ -n "${FTL_NO_TIME_COLOR:-}" || $color_ok -eq 0 ]]; then
        printf "%s %-12s %-12s %s\n" "$mark_col" "$mt" "$bt" "$name"
      else
        # 色を付けて表示（表示が崩れないように調整）
        printf '%s ' "$mark_col"
        printf "${time_color}%-12s${esc}[0m " "$mt"
        printf "${created_time_color}%-12s${esc}[0m " "$bt"
        printf '%s\n' "$name"
      fi
    done
fi
