#!/usr/bin/env bash
# Asks GitHub for a Linux AppImage of the current branch: adds an empty
# commit whose message carries [appimage] and pushes the branch, which
# starts .github/workflows/linux-appimage.yml. The AppImage is built on
# Ubuntu 22.04, run on Fedora with no network (it must open no socket
# but local Unix ones), and lands on the repository's Releases page
# under a new appimage-<date>-<commit> tag.
# Usage: scripts/request-appimage.sh   (commit the work to build first)
set -euo pipefail

if [[ -n "$(git status --porcelain)" ]]; then
    echo "error: uncommitted changes; commit what the build should contain first" >&2
    exit 1
fi
branch=$(git symbolic-ref --short HEAD)

git commit --allow-empty -q -s -m "Request a Linux AppImage [appimage]"
git push -u origin "$branch"

sha=$(git rev-parse --short=7 HEAD)
repo=$(git remote get-url origin | sed -E 's#^git@github.com:##; s#^https://[^/]*/##; s#\.git$##')
echo "commit: ${sha} on ${branch}"
echo "build: https://github.com/${repo}/actions/workflows/linux-appimage.yml"
echo "files (when done): https://github.com/${repo}/releases (tag appimage-<date>-${sha})"
echo "check: curl -s https://api.github.com/repos/${repo}/releases | grep -o '\"tag_name\": \"appimage-[^\"]*${sha}\"'"
