#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if command -v sccache >/dev/null 2>&1; then
    export RUSTC_WRAPPER="sccache"
fi

if [[ "$(uname -s)" == "Linux" ]]; then
    if command -v mold >/dev/null 2>&1; then
        LINKER="mold"
    elif command -v ld.lld >/dev/null 2>&1; then
        LINKER="lld"
    else
        LINKER=""
    fi

    if [[ -n "${LINKER}" ]]; then
        if [[ -n "${RUSTFLAGS:-}" ]]; then
            export RUSTFLAGS="${RUSTFLAGS} -C link-arg=-fuse-ld=${LINKER}"
        else
            export RUSTFLAGS="-C link-arg=-fuse-ld=${LINKER}"
        fi
    fi
fi

cd "$ROOT"
cargo build --release --timings --bin ftime "$@"
printf 'timings: %s\n' "$ROOT/target/cargo-timings/cargo-timing.html"
