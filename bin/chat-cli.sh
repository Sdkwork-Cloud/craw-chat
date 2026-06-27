#!/usr/bin/env bash
set -euo pipefail

script_source="${BASH_SOURCE[0]}"
script_dir="${script_source%/*}"
if [[ "$script_dir" == "$script_source" ]]; then
  script_dir="."
fi
ROOT_DIR="$(cd -- "$script_dir" && pwd)"
exec "${BASH:-bash}" "${ROOT_DIR}/chat-cli-local.sh" "$@"
