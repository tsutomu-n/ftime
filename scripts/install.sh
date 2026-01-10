#!/usr/bin/env bash
set -euo pipefail

REPO="tsutomu-n/ftime"
BIN="ftime"

die() {
    echo "error: $*" >&2
    exit 1
}

need_cmd() {
    command -v "$1" >/dev/null 2>&1 || die "missing required command: $1"
}

detect_target() {
    local os arch

    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux) os="linux" ;;
        Darwin) os="darwin" ;;
        *) die "unsupported OS: $os" ;;
    esac

    case "$arch" in
        x86_64) arch="x86_64" ;;
        arm64 | aarch64) arch="aarch64" ;;
        *) die "unsupported arch: $arch" ;;
    esac

    case "${os}-${arch}" in
        linux-x86_64) echo "x86_64-unknown-linux-gnu" ;;
        darwin-x86_64) echo "x86_64-apple-darwin" ;;
        darwin-aarch64) echo "aarch64-apple-darwin" ;;
        *) die "unsupported target: ${os}-${arch}" ;;
    esac
}

resolve_tag() {
    local version api tag

    version="${1:-latest}"
    if [[ "$version" != "latest" ]]; then
        echo "v${version#v}"
        return 0
    fi

    api="https://api.github.com/repos/${REPO}/releases/latest"
    if command -v jq >/dev/null 2>&1; then
        tag="$(curl -fsSL "$api" | jq -r '.tag_name // empty')"
    else
        tag="$(curl -fsSL "$api" \
            | sed -n 's/.*"tag_name":[[:space:]]*"\([^"]*\)".*/\1/p' \
            | head -n1)"
    fi

    if [[ -z "${tag}" ]]; then
        die "failed to resolve latest release tag"
    fi
    echo "$tag"
}

need_cmd curl
need_cmd tar
need_cmd install

target="$(detect_target)"
tag="$(resolve_tag "${1:-latest}")"

asset="${BIN}-${tag#v}-${target}.tar.gz"
url="https://github.com/${REPO}/releases/download/${tag}/${asset}"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

curl -fsSL "$url" -o "$tmpdir/$asset"
tar -xzf "$tmpdir/$asset" -C "$tmpdir"

install_dir="${INSTALL_DIR:-/usr/local/bin}"
if [[ ! -w "${install_dir}" ]]; then
    install_dir="${HOME}/.local/bin"
    mkdir -p "$install_dir"
fi

install -m 0755 "$tmpdir/$BIN" "$install_dir/$BIN"

printf '%s installed to %s\n' "$BIN" "$install_dir"
case ":$PATH:" in
    *":$install_dir:"*) ;;
    *) printf 'PATHに %s を追加してください\n' "$install_dir" ;;
esac
