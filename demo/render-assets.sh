#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEMO_DIR="${ROOT}/demo"
TAPE="${DEMO_DIR}/tapes/demo_ftime.tape"
ASSETS_DIR="${ROOT}/assets"

# Public outputs:
# - assets/demo_ftime.gif
# - assets/demo_ftime.mp4

if [[ ! -f "${TAPE}" ]]; then
    printf 'missing tape: %s\n' "${TAPE}" >&2
    exit 1
fi

if ! command -v vhs >/dev/null 2>&1; then
    printf 'vhs is required to render demo assets\n' >&2
    exit 1
fi

cargo build --release --bin ftime

SCENE_PARENT="$(mktemp -d "${TMPDIR:-/tmp}/ftime-render.XXXXXX")"
SCENE_DIR="${SCENE_PARENT}/scene"
"${DEMO_DIR}/setup-scene.sh" "${SCENE_DIR}" >/dev/null

export FTIME_DEMO_DIR="${SCENE_DIR}"
export PATH="${ROOT}/target/release:${PATH}"

vhs "${TAPE}" \
    -o "${ASSETS_DIR}/demo_ftime.gif" \
    -o "${ASSETS_DIR}/demo_ftime.mp4"

printf 'Rendered demo assets:\n'
printf '  %s\n' "${ASSETS_DIR}/demo_ftime.gif"
printf '  %s\n' "${ASSETS_DIR}/demo_ftime.mp4"
printf 'Scene used: %s\n' "${SCENE_DIR}"
