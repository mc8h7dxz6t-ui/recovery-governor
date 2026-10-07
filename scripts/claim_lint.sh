#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FORBIDDEN=(
  'externally qualified'
  'production-ready external effect'
  'trust-boundary passport'
  'SAFE_TO_RETRY'
  'exactly-once'
  'RECOVERY_GOVERNOR_QUALIFIED=true'
)
FILES=("$ROOT/README.md" "$ROOT/RECOVERY_GOVERNOR_FIRST_PRODUCT_BUILD_REPORT.md")
while IFS= read -r -d '' f; do FILES+=("$f"); done < <(find "$ROOT/docs" -name '*.md' -print0 2>/dev/null)
fail=0
for phrase in "${FORBIDDEN[@]}"; do
  if rg -i -n "$phrase" "${FILES[@]}" 2>/dev/null; then
    echo "claim lint: forbidden phrase: $phrase"
    fail=1
  fi
done
exit "$fail"
