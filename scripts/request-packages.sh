#!/usr/bin/env bash
# Asks GitHub for installers of the current branch: adds an empty commit
# whose message carries the formats asked for and pushes the branch,
# which starts .github/workflows/packages.yml. Every Linux package is
# installed and run with no network on its distribution before the
# files land on the repository's Releases page, all together, under a
# new build-<date>-<commit> tag.
# Usage: scripts/request-packages.sh [exe] [appimage] [deb] [rpm] [linux] [all]
#   (no argument: all; commit the work to build first)
set -euo pipefail

formats=("$@")
[[ ${#formats[@]} -gt 0 ]] || formats=(all)
tokens=""
for f in "${formats[@]}"; do
    case "$f" in
        exe|appimage|deb|rpm|linux|all) tokens+="[$f]" ;;
        *) echo "error: unknown format '$f' (exe, appimage, deb, rpm, linux, all)" >&2; exit 1 ;;
    esac
done

if [[ -n "$(git status --porcelain)" ]]; then
    echo "error: uncommitted changes; commit what the build should contain first" >&2
    exit 1
fi
branch=$(git symbolic-ref --short HEAD)

git commit --allow-empty -q -s -m "Request installers ${tokens}"
git push -u origin "$branch"

sha=$(git rev-parse --short=7 HEAD)
repo=$(git remote get-url origin | sed -E 's#^git@github.com:##; s#^https://[^/]*/##; s#\.git$##')
echo "commit: ${sha} on ${branch}, formats: ${tokens}"
echo "build: https://github.com/${repo}/actions/workflows/packages.yml"
echo "files (when done): https://github.com/${repo}/releases (tag build-<date>-${sha})"
echo "check: curl -s https://api.github.com/repos/${repo}/releases | grep -o '\"tag_name\": \"build-[^\"]*${sha}\"'"
