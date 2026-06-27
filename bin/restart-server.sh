#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/restart-server.sh [--instance <name>] [--install-root <path>] [--config-dir <path>] [--log-dir <path>] [--run-dir <path>] [--binary-path <path>] [--release] [--foreground] [--health-url <url>] [--skip-health-check]

Restart sdkwork-im-server using the stop/start runtime service scripts and preserve instance/config/status semantics.
EOF
}

args=("$@")
for arg in "$@"; do
  if [[ "$arg" == "-h" || "$arg" == "--help" ]]; then
    show_help
    exit 0
  fi
done

bash "$(dirname "${BASH_SOURCE[0]}")/stop-server.sh" "${args[@]}" || exit $?
bash "$(dirname "${BASH_SOURCE[0]}")/start-server.sh" "${args[@]}"
