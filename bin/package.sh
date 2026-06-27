#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/package.sh [--package-id <id>] [--version <value>] [--staging-root <dir>] [--output-dir <dir>] [--all] [--stage] [--check] [--dry-run] [--json]

Stage and/or package Sdkwork IM release archives. Use --stage to stage production outputs before packaging.
EOF
}

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
stage=0
stage_args=("scripts/release/stage-sdkwork-im-release-package.mjs")
package_args=("scripts/release/build-sdkwork-im-install-package.mjs")

while [[ $# -gt 0 ]]; do
  case "$1" in
    --stage)
      stage=1
      shift
      ;;
    --package-id|--version|--staging-root)
      stage_args+=("$1" "$2")
      package_args+=("$1" "$2")
      shift 2
      ;;
    --output-dir)
      package_args+=("$1" "$2")
      shift 2
      ;;
    --all|--check|--dry-run|--json)
      stage_args+=("$1")
      package_args+=("$1")
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
if [[ "$stage" -eq 1 ]]; then
  node "${stage_args[@]}"
fi
exec node "${package_args[@]}"
