#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/start-prod.sh [--instance <name>] [--install-root <path>] [--config-dir <path>] [--log-dir <path>] [--run-dir <path>] [--env-file <path>] [--binary-path <path>] [--foreground] [--health-url <url>] [--skip-health-check]

Start packaged Sdkwork IM server in production/release mode.
EOF
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
args=(--release)

while [[ $# -gt 0 ]]; do
  case "$1" in
    --instance|--install-root|--config-dir|--log-dir|--run-dir|--env-file|--binary-path|--health-url)
      args+=("$1" "$2")
      shift 2
      ;;
    --foreground|--skip-health-check)
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

exec bash "$SCRIPT_DIR/start-server.sh" "${args[@]}"
