#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/verify-server.sh [--instance <name>] [--config-dir <path>] [--release-gate-path <path-to-release-gate.json>] [--output-format <text|json>]

Validate config, storage wiring, and ready state for sdkwork-im-server, and optionally audit the machine-readable release-gate bundle for semantic contract drift, decisionStatus, contractsValid, platforms, and semanticIssues.
EOF
}

instance_name="default"
config_dir="/etc/sdkwork/chat"
release_gate_path=""
output_format="text"
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

escape_json() {
  local value="$1"
  value="${value//\\/\\\\}"
  value="${value//\"/\\\"}"
  value="${value//$'\r'/}"
  value="${value//$'\n'/\\n}"
  printf '%s' "$value"
}

json_array_strings() {
  local first=1
  printf '['
  for value in "$@"; do
    if [[ $first -eq 0 ]]; then
      printf ', '
    fi
    first=0
    printf '"%s"' "$(escape_json "$value")"
  done
  printf ']'
}

server_config_dir_for_instance() {
  local name="$1"
  if [[ "$name" == "default" ]]; then
    printf '/etc/sdkwork/chat\n'
  else
    printf '/etc/sdkwork/chat/instances/%s\n' "$name"
  fi
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --instance)
      instance_name="$2"
      config_dir="$(server_config_dir_for_instance "$instance_name")"
      shift 2
      ;;
    --config-dir)
      config_dir="$2"
      shift 2
      ;;
    --release-gate-path)
      release_gate_path="$2"
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

server_yaml="${config_dir}/server.yaml"
legacy_chat_toml="${config_dir}/chat.toml"
postgresql_yaml="${config_dir}/postgresql.yaml"
storage_postgresql_yaml="${config_dir}/storage/postgresql.yaml"
missing=()
config_missing=()
storage_missing=()

if [[ -f "$server_yaml" ]]; then
  server_config_path="$server_yaml"
elif [[ -f "$legacy_chat_toml" ]]; then
  server_config_path="$legacy_chat_toml"
else
  server_config_path=""
  config_missing+=("server.yaml")
fi

if [[ -f "$postgresql_yaml" ]]; then
  storage_config_path="$postgresql_yaml"
elif [[ -f "$storage_postgresql_yaml" ]]; then
  storage_config_path="$storage_postgresql_yaml"
else
  storage_config_path=""
  storage_missing+=("postgresql.yaml")
fi

server_content="$(cat "$server_config_path" 2>/dev/null || true)"
storage_content="$(cat "$storage_config_path" 2>/dev/null || true)"

if [[ -n "$server_config_path" ]]; then
  if [[ "$(basename "$server_config_path")" == "server.yaml" ]]; then
    for contract in "instance:" "name:" "network:" "bindAddress:" "publicEndpoints:" "baseUrl:" "runtime:" "dataRoot:"; do
      [[ "$server_content" == *"$contract"* ]] || config_missing+=("$contract")
    done
  else
    for contract in "[runtime]" "deployment_mode = \"server\"" "app_code = \"chat\"" "[server]" "bind_address ="; do
      [[ "$server_content" == *"$contract"* ]] || config_missing+=("$contract")
    done
  fi
fi

if [[ -n "$storage_config_path" ]]; then
  for contract in "provider: postgresql" "passwordFile:" "migrationMode:"; do
    [[ "$storage_content" == *"$contract"* ]] || storage_missing+=("$contract")
  done

  password_file="$(printf '%s\n' "$storage_content" | awk -F: '/^[[:space:]]*passwordFile:/ {sub(/^[[:space:]]+/, "", $2); sub(/[[:space:]]+$/, "", $2); gsub(/^["'\'']|["'\'']$/, "", $2); print $2; exit}')"
  if [[ -z "$password_file" ]]; then
    storage_missing+=("passwordFile target")
  elif [[ "$password_file" = /* ]]; then
    [[ -f "$password_file" ]] || storage_missing+=("passwordFile target")
  else
    storage_root="$(cd "$(dirname "$storage_config_path")" && pwd)"
    [[ -f "${config_dir}/${password_file}" || -f "${storage_root}/${password_file}" ]] || storage_missing+=("passwordFile target")
  fi
fi

missing=("${config_missing[@]}" "${storage_missing[@]}")

config_valid=true
if [[ ${#config_missing[@]} -gt 0 ]]; then
  config_valid=false
fi
storage_valid=true
if [[ ${#storage_missing[@]} -gt 0 ]]; then
  storage_valid=false
fi

release_contracts_enabled=false
release_contracts_valid=true
release_contracts_json='{"enabled": false}'
release_contracts_summary=""

if [[ -n "$release_gate_path" ]]; then
  release_contracts_enabled=true
  verifier_path="${script_dir}/verify-server-release-contracts.mjs"
  release_contracts_json="$(node "$verifier_path" --release-gate-path "$release_gate_path" --format json)"
  release_contracts_valid="$(node "$verifier_path" --release-gate-path "$release_gate_path" --field contractsValid)"
  if [[ "$output_format" != "json" ]]; then
    release_contracts_summary="$(node "$verifier_path" --release-gate-path "$release_gate_path" --format text)"
  fi
fi

ready=false
if [[ "$config_valid" == true && "$storage_valid" == true ]]; then
  ready=true
fi
if [[ "$release_contracts_enabled" == true && "$release_contracts_valid" != true ]]; then
  ready=false
fi

if [[ "$output_format" == "json" ]]; then
  cat <<EOF
{
  "product": "sdkwork-im-server",
  "instance": "${instance_name}",
  "config": "${config_dir}",
  "configValid": ${config_valid},
  "storageValid": ${storage_valid},
  "ready": ${ready},
  "output": "${output_format}",
  "missing": $(json_array_strings "${missing[@]}"),
  "releaseContracts": ${release_contracts_json}
}
EOF
else
  echo "sdkwork-im-server verify report"
  echo "config: ${config_dir}"
  echo "configValid: ${config_valid}"
  echo "storageValid: ${config_valid}"
  echo "ready: ${ready}"
  if [[ ${#missing[@]} -gt 0 ]]; then
    echo "missing: ${missing[*]}"
  fi
  if [[ "$release_contracts_enabled" == true ]]; then
    printf '%s\n' "$release_contracts_summary"
  fi
fi
