#!/usr/bin/env bash
# Asks GitHub for a Windows build of the current commit: pushes a tag
# named exe-<date>-<commit>, which starts .github/workflows/windows-exe.yml.
# The files land on the repository's Releases page under that tag.
# Usage: scripts/request-exe.sh [commit]   (default: HEAD, must be pushed)
set -euo pipefail

rev="${1:-HEAD}"
sha=$(git rev-parse --verify "${rev}^{commit}")
short=${sha:0:7}

# The commit has to be on the remote or the tag push carries it along;
# either works, but say so when it is not on any remote branch.
if ! git branch -r --contains "$sha" 2>/dev/null | grep -q .; then
    echo "note: ${short} is not on a remote branch yet; the tag push uploads it" >&2
fi

tag="exe-$(date -u +%Y%m%d-%H%M)-${short}"
git tag -a "$tag" -m "Windows build of ${short}" "$sha"
git push origin "refs/tags/${tag}"

url=$(git remote get-url origin | sed -E 's#^git@github.com:#https://github.com/#; s#\.git$##')
echo "tag: ${tag}"
echo "build: ${url}/actions/workflows/windows-exe.yml"
echo "files (when done): ${url}/releases/tag/${tag}"
