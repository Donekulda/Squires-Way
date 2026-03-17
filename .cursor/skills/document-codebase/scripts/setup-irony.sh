#!/usr/bin/env bash
# Clone IronyModManager into Examples/IronyModManager for documentation workflow

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

mkdir -p "$WORKSPACE_ROOT/Examples"
cd "$WORKSPACE_ROOT"

if [ -d "Examples/IronyModManager" ]; then
  echo "Examples/IronyModManager already exists. Pulling latest..."
  git -C Examples/IronyModManager pull
else
  git clone https://github.com/bcssov/IronyModManager.git Examples/IronyModManager
fi

echo "Done. Run: /document-codebase Examples/IronyModManager/src Examples/IronyModManager/docs"
