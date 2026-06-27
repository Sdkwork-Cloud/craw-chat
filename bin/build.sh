#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/build.sh [--target server|desktop|all] [--target-triple <triple>] [--platform <platform>] [--arch <arch>] [--dry-run] [--json]

Build Sdkwork IM production server binary/web assets and/or desktop installer artifacts.
EOF
}

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
args=("scripts/release/build-sdkwork-im-production.mjs")

while [[ $# -gt 0 ]]; do
  case "$1" in
    --target|--target-triple|--platform|--arch)
      args+=("$1" "$2")
      shift 2
      ;;
    --dry-run|--json)
      args+=("$1")
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

cd "$ROOT_DIR"
exec node "${args[@]}"
