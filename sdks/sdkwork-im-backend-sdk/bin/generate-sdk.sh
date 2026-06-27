#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LANGUAGES="${LANGUAGES:-typescript,flutter,rust,java,csharp,swift,kotlin,go,python}"
BASE_URL="${BASE_URL:-http://127.0.0.1:18079}"
FIXED_SDK_VERSION="${FIXED_SDK_VERSION:-}"
SCHEMA_URL="${SCHEMA_URL:-}"
REFRESH_LIVE="${REFRESH_LIVE:-false}"
EXTRA_ARGS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --language|--languages)
      LANGUAGES="$2"
      shift 2
      ;;
    --fixed-sdk-version)
      FIXED_SDK_VERSION="$2"
      shift 2
      ;;
    --base-url)
      BASE_URL="$2"
      shift 2
      ;;
    --schema-url)
      SCHEMA_URL="$2"
      shift 2
      ;;
    --refresh-live)
      REFRESH_LIVE="true"
      shift
      ;;
    *)
      EXTRA_ARGS+=("$1")
      shift
      ;;
  esac
done

ARGS=()
IFS=',' read -ra LANGUAGE_VALUES <<< "${LANGUAGES}"
for LANGUAGE_VALUE in "${LANGUAGE_VALUES[@]}"; do
  LANGUAGE="$(echo "${LANGUAGE_VALUE}" | xargs)"
  if [[ -n "${LANGUAGE}" ]]; then
    ARGS+=(--language "${LANGUAGE}")
  fi
done

if [[ -n "${FIXED_SDK_VERSION}" ]]; then
  ARGS+=(--fixed-sdk-version "${FIXED_SDK_VERSION}")
fi
if [[ -n "${BASE_URL}" ]]; then
  ARGS+=(--base-url "${BASE_URL}")
fi
if [[ -n "${SCHEMA_URL}" ]]; then
  ARGS+=(--schema-url "${SCHEMA_URL}")
fi
if [[ "${REFRESH_LIVE}" == "true" ]]; then
  ARGS+=(--refresh-live)
fi

exec node "${SCRIPT_DIR}/generate-sdk.mjs" "${ARGS[@]}" "${EXTRA_ARGS[@]}"
