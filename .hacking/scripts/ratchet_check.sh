#!/usr/bin/env bash
# Checks that GitHub Actions are pinned using ratchet.

set -euo pipefail

RATCHET="${RUNFILES_DIR:-$0.runfiles}/+tools+ratchet/ratchet"

if [[ ! -x "$RATCHET" ]]; then
    echo "ERROR: ratchet binary not found at $RATCHET"
    exit 1
fi

"$RATCHET" lint .github/workflows/*.yml
