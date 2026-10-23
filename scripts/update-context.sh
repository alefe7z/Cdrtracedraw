#!/usr/bin/env bash
# Regenerate docs/agents/STATUS.md from git: the branch, the last
# commits with their authors, the files touched lately, the open claims
# of docs/agents/CONTEXT.md and the uncommitted changes. The pre-commit
# hook runs it, so the file is current in every commit.
# Usage: scripts/update-context.sh
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
out=docs/agents/STATUS.md
branch=$(git symbolic-ref --short HEAD 2>/dev/null || echo "(detached)")
{
    echo "# Agents' status (generated)"
    echo
    echo "Written by \`scripts/update-context.sh\` on every commit; do not"
    echo "edit by hand. The hand-written context is \`docs/agents/CONTEXT.md\`."
    echo
    echo "- Branch: \`$branch\`"
    echo "- Last commit before this one: \`$(git log -1 --format='%h %s' 2>/dev/null)\`"
    echo "- Commits on the branch: $(git rev-list --count HEAD 2>/dev/null)"
    echo
    echo "## Last 20 commits"
    echo
    git log -20 --format='- `%h` %ad %an: %s' --date=format:'%Y-%m-%d %H:%M' \
        | grep -v "Update the agents' status file" || true
    echo
    echo "## Files changed in the last 10 commits"
    echo
    git log -10 --name-only --format='' | grep -v '^$' | grep -v '^docs/agents/STATUS.md$' \
        | sort | uniq -c | sort -rn | head -30 | awk '{print "- `"$2"` ("$1")"}'
    echo
    echo "## Open claims (from CONTEXT.md)"
    echo
    claims=$(awk '/^## Claims/{f=1;next} /^## /{f=0} f && /^\| / && !/^\| *(Agent|---)/' docs/agents/CONTEXT.md 2>/dev/null || true)
    if [[ -n "$claims" ]]; then
        echo "| Agent | Area / files | Since | Note |"
        echo "| --- | --- | --- | --- |"
        echo "$claims"
    else
        echo "None."
    fi
    echo
    echo "## Uncommitted when generated"
    echo
    changes=$(git status --porcelain | grep -v ' docs/agents/STATUS.md$' || true)
    if [[ -n "$changes" ]]; then
        echo "$changes" | sed 's/^/    /'
    else
        echo "Nothing."
    fi
} > "$out.tmp"
mv "$out.tmp" "$out"
