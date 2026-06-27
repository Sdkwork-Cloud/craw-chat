#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/dev.sh [--postgres] [--desktop]

Start Sdkwork IM development mode. Browser is default; --desktop starts the Tauri desktop dev flow; --postgres loads .env.postgres.
EOF
}

postgres=0
desktop=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --postgres)
      postgres=1
      shift
      ;;
    --desktop)
      desktop=1
      shift
      ;;
    -h|--help)
      show_help
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      show_help >&2
      exit 1
      ;;
  esac
done

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ "$desktop" -eq 1 && "$postgres" -eq 1 ]]; then
  script_name="tauri:dev:postgres"
elif [[ "$desktop" -eq 1 ]]; then
  script_name="tauri:dev"
elif [[ "$postgres" -eq 1 ]]; then
  script_name="dev:postgres"
else
  script_name="dev"
fi

cd "$ROOT_DIR"
exec pnpm "$script_name"
