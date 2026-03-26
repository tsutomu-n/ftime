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

resolve_download() {
    local version target asset url

    version="${1:-latest}"
    target="$2"

    if [[ "$version" == "latest" ]]; then
        asset="${BIN}-${target}.tar.gz"
        url="https://github.com/${REPO}/releases/latest/download/${asset}"
        printf '%s\n%s\n' "latest" "$url"
        return 0
    fi

    version="v${version#v}"
    asset="${BIN}-${version#v}-${target}.tar.gz"
    url="https://github.com/${REPO}/releases/download/${version}/${asset}"
    printf '%s\n%s\n' "$version" "$url"
}

need_cmd curl
need_cmd tar
need_cmd install

target="$(detect_target)"
mapfile -t release_info < <(resolve_download "${1:-latest}" "$target")
tag="${release_info[0]}"
url="${release_info[1]}"
asset="${url##*/}"

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
