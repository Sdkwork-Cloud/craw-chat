#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/plan-release-server.sh [--release-gate-path <path-to-release-gate.json>] [--platform <all|linux|macos|windows>] [--output-format <text|json>]

Summarize the sdkwork-im-server release plan from the machine-readable release-gate, package-catalog, and release-execution contracts.
The emitted plan keeps checksum and artifact-file-list contract pointers visible for operators and automation.
EOF
}

release_gate_path=""
platform="all"
output_format="text"
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --release-gate-path)
      release_gate_path="$2"
      shift 2
      ;;
    --platform)
      platform="$2"
      shift 2
      ;;
    --output-format)
      output_format="$2"
      shift 2
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

if [[ -z "$release_gate_path" ]]; then
  echo "release-gate-path is required" >&2
  exit 1
fi

helper_path="${script_dir}/plan-release-server-contracts.mjs"
node "$helper_path" --release-gate-path "$release_gate_path" --platform "$platform" --format "$output_format"
