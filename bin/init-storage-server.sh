#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/init-storage-server.sh [--instance <name>] [--config-dir <path>] [--mode <verify-only|bootstrap-schema|create-db-and-schema>] [--output-format <text|json>]

Validate the file-based postgresql storage contract, summarize the selected storage mode, and write a storage report.
EOF
}

instance_name="default"
config_dir="/etc/sdkwork/chat"
mode="verify-only"
output_format="text"

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
    --mode)
      mode="$2"
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

postgresql_yaml="${config_dir}/postgresql.yaml"
report_path="${config_dir}/storage-init-report.json"
missing=()

if [[ ! -f "$postgresql_yaml" ]]; then
  missing+=("postgresql.yaml")
  postgresql_content=""
else
  postgresql_content="$(cat "$postgresql_yaml")"
fi

for contract in "provider: postgresql" "connection:" "database:" "username:" "passwordFile:" "migrationMode:"; do
  if [[ "$postgresql_content" != *"$contract"* ]]; then
    missing+=("$contract")
  fi
done

config_valid=true
if [[ ${#missing[@]} -gt 0 ]]; then
  config_valid=false
fi

ready="$config_valid"

cat >"$report_path" <<EOF
{
  "product": "sdkwork-im-server",
  "instance": "${instance_name}",
  "mode": "${mode}",
  "storage": "postgresql",
  "configPath": "${postgresql_yaml}",
  "report": "${report_path}",
  "configValid": ${config_valid},
  "ready": ${ready},
  "note": "First landing validates the file-based PostgreSQL contract and writes a truthful report. Live database connectivity checks are the next stage."
}
EOF

if [[ "$output_format" == "json" ]]; then
  cat "$report_path"
else
  echo "sdkwork-im-server storage report"
  echo "mode: ${mode}"
  echo "storage: postgresql"
  echo "configValid: ${config_valid}"
  echo "ready: ${ready}"
  echo "report: ${report_path}"
  if [[ ${#missing[@]} -gt 0 ]]; then
    echo "missing: ${missing[*]}"
  fi
fi
