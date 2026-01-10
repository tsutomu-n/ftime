#!/usr/bin/env bash
set -euo pipefail

REPO="tsutomu-n/CLI-Tools"
BIN="ftime"

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
    Linux) os="linux" ;;
    Darwin) os="darwin" ;;
    *) echo "unsupported OS: $os" >&2; exit 1 ;;
esac

case "$arch" in
    x86_64) arch="x86_64" ;;
    arm64|aarch64) arch="aarch64" ;;
    *) echo "unsupported arch: $arch" >&2; exit 1 ;;
esac

case "${os}-${arch}" in
    linux-x86_64) target="x86_64-unknown-linux-gnu" ;;
    darwin-x86_64) target="x86_64-apple-darwin" ;;
    darwin-aarch64) target="aarch64-apple-darwin" ;;
    *) echo "unsupported target: ${os}-${arch}" >&2; exit 1 ;;
esac

version="${1:-latest}"
if [[ "$version" == "latest" ]]; then
    tag="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | sed -n 's/.*"tag_name": *"\\([^"]*\\)".*/\\1/p' \
        | head -n1)"
    if [[ -z "${tag}" ]]; then
        echo "failed to resolve latest release tag" >&2
        exit 1
    fi
else
    tag="v${version#v}"
fi

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
