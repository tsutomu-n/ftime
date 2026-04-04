#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMPLATE_DIR="${ROOT}/template"

if [[ ! -d "${TEMPLATE_DIR}" ]]; then
    printf 'missing template directory: %s\n' "${TEMPLATE_DIR}" >&2
    exit 1
fi

if [[ $# -gt 1 ]]; then
    printf 'usage: %s [OUTPUT_DIR]\n' "$0" >&2
    exit 1
fi

if [[ $# -eq 1 ]]; then
    OUTPUT_DIR="$1"
    if [[ -e "${OUTPUT_DIR}" ]]; then
        printf 'output already exists: %s\n' "${OUTPUT_DIR}" >&2
        exit 1
    fi
    mkdir -p "${OUTPUT_DIR}"
else
    OUTPUT_DIR="$(mktemp -d "${TMPDIR:-/tmp}/ftime-demo.XXXXXX")"
fi

cp -R "${TEMPLATE_DIR}/." "${OUTPUT_DIR}/"

for i in $(seq 1 21); do
    printf '# history note %02d\n' "${i}" > "${OUTPUT_DIR}/history-note-${i}.md"
done

# Core fixture files:
# - template/docs/guide.md
# - template/target/app.bin
# - template/tests/cli.rs

set_mtime() {
    local offset="$1"
    local path="$2"
    perl -e '
        my ($offset, $path) = @ARGV;
        my $ts = time() + $offset;
        utime $ts, $ts, $path or die "utime($path): $!";
    ' -- "${offset}" "${path}"
}

# Active
set_mtime -720 "${OUTPUT_DIR}/Cargo.toml"

# Today
set_mtime -14400 "${OUTPUT_DIR}/README.md"
set_mtime -10800 "${OUTPUT_DIR}/docs"
set_mtime -900 "${OUTPUT_DIR}/docs/guide.md"

# This Week
set_mtime -93600 "${OUTPUT_DIR}/Cargo.lock"
set_mtime -172800 "${OUTPUT_DIR}/target"
set_mtime -600 "${OUTPUT_DIR}/target/app.bin"
set_mtime -259200 "${OUTPUT_DIR}/assets"
set_mtime -250000 "${OUTPUT_DIR}/assets/demo.txt"

# History
set_mtime -864000 "${OUTPUT_DIR}/tests"
set_mtime -18000 "${OUTPUT_DIR}/tests/cli.rs"
for i in $(seq 1 21); do
    set_mtime "-$((950400 + (i * 3600)))" "${OUTPUT_DIR}/history-note-${i}.md"
done

printf 'Generated demo scene: %s\n' "${OUTPUT_DIR}"
printf 'Next:\n'
printf '  cd %q\n' "${OUTPUT_DIR}"
printf '  ftime\n'
printf '  ftime -a\n'
printf "  ftime --json | jq -r '.path'\n"
