#!/usr/bin/env bash
# Build the .rpm from programs built beforehand.
# Usage: build-rpm.sh <version> <release-dir> <output-dir>
# <version> is the workspace version, optionally with "-dev.<commit>":
# rpm takes 0.11.0 as Version and 0.dev.<commit> as Release.
set -euo pipefail
VERSION="$1"; BIN="$(realpath "$2")"; OUT="$(mkdir -p "$3" && realpath "$3")"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
base="${VERSION%%-*}"
if [[ "$VERSION" == *-* ]]; then release="0.${VERSION#*-}"; else release=1; fi
release="${release//-/.}"
TOP="$(mktemp -d)"
mkdir -p "$TOP/SOURCES" "$TOP/files"
cp "$BIN/tracedraw" "$BIN/tracedraw-cli" "$TOP/files/"
cp -R "$BIN/help" "$TOP/files/help"
cp "$ROOT/packaging/linux/tracedraw.desktop" "$TOP/files/"
cp "$ROOT/assets/icon/tracedraw-256.png" "$TOP/files/tracedraw.png"
cp "$ROOT/LICENSE-MIT" "$ROOT/LICENSE-APACHE" "$TOP/files/"
tar -C "$TOP/files" -czf "$TOP/SOURCES/tracedraw-files.tar.gz" .
rpmbuild -bb --define "_topdir $TOP" --define "pkg_version $base" \
    --define "pkg_release $release" "$ROOT/packaging/linux/tracedraw.spec"
find "$TOP/RPMS" -name '*.rpm' -exec cp {} "$OUT/" \;
rm -rf "$TOP"
ls "$OUT"/*.rpm
