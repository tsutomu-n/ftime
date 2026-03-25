#!/usr/bin/env bash
set -euo pipefail

BIN="ftime"

remove_bin() {
    local path="$1"

    if [[ -e "$path" ]]; then
        rm -f "$path"
        printf '%s uninstalled from %s\n' "$BIN" "$(dirname "$path")"
        return 0
    fi

    return 1
}

install_dir="${INSTALL_DIR:-/usr/local/bin}"
fallback_dir="${HOME}/.local/bin"

if remove_bin "${install_dir}/${BIN}"; then
    exit 0
fi

if [[ "$fallback_dir" != "$install_dir" ]] && remove_bin "${fallback_dir}/${BIN}"; then
    exit 0
fi

printf '%s is not installed in %s or %s\n' "$BIN" "$install_dir" "$fallback_dir"
