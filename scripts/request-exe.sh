#!/usr/bin/env bash
# Asks GitHub for a Windows build only; see scripts/request-packages.sh.
exec "$(dirname "$0")/request-packages.sh" exe
