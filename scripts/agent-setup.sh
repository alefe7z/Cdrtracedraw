#!/usr/bin/env bash
# One-time setup of a clone for working on a branch shared with other
# agents: turns on the repository's git hooks (.githooks) and the merge
# driver that keeps docs/agents/STATUS.md from ever conflicting (it is
# regenerated after every merge or rebase anyway). Safe to run again.
# Usage: scripts/agent-setup.sh
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
git config core.hooksPath .githooks
git config merge.agent-status.name "regenerated status file"
git config merge.agent-status.driver true
git config pull.rebase true
git config rebase.autoStash true
chmod +x .githooks/* scripts/agent-*.sh scripts/update-context.sh 2>/dev/null || true
echo "agent setup done: hooks in .githooks, pull rebases, STATUS.md never conflicts"
