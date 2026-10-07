#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
if rg -n 'gho_[A-Za-z0-9]{20,}|AKIA[0-9A-Z]{16}|-----BEGIN (RSA |OPENSSH )?PRIVATE KEY-----' "$ROOT" --glob '!target/**' 2>/dev/null; then
  echo "secret scan: suspected credential material"
  exit 1
fi
echo "secret scan: clean"
