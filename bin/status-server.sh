#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/status-server.sh [--instance <name>] [--config-dir <path>] [--release-gate-path <path-to-release-gate.json>] [--output-format <text|json>]

Show sdkwork-im-server status, generated service contracts, storage report paths, and optionally summarize the machine-readable release-gate bundle, decisionStatus, contractsValid, platforms, and semanticIssues.
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

systemd_contract_path="${config_dir}/generated/sdkwork-im-server.service"
launchd_contract_path="${config_dir}/generated/com.sdkwork.im.server.plist"
windows_service_contract_path="${config_dir}/generated/SdkworkImServer.xml"
windows_service_install_script_path="${config_dir}/generated/install-SdkworkImServer.ps1"
windows_service_uninstall_script_path="${config_dir}/generated/uninstall-SdkworkImServer.ps1"
storage_report_path="${config_dir}/storage-init-report.json"

systemd_exists=false
launchd_exists=false
windows_service_exists=false
windows_service_install_script_exists=false
windows_service_uninstall_script_exists=false
storage_report_exists=false
[[ -f "$systemd_contract_path" ]] && systemd_exists=true
[[ -f "$launchd_contract_path" ]] && launchd_exists=true
[[ -f "$windows_service_contract_path" ]] && windows_service_exists=true
[[ -f "$windows_service_install_script_path" ]] && windows_service_install_script_exists=true
[[ -f "$windows_service_uninstall_script_path" ]] && windows_service_uninstall_script_exists=true
[[ -f "$storage_report_path" ]] && storage_report_exists=true

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

if [[ "$output_format" == "json" ]]; then
  cat <<EOF
{
  "product": "sdkwork-im-server",
  "instance": "$(escape_json "$instance_name")",
  "config": "$(escape_json "$config_dir")",
  "status": "configuration-only skeleton",
  "output": "$(escape_json "$output_format")",
  "serviceContracts": {
    "systemd": {
      "path": "$(escape_json "$systemd_contract_path")",
      "exists": ${systemd_exists}
    },
    "launchd": {
      "path": "$(escape_json "$launchd_contract_path")",
      "label": "com.sdkwork.im.server",
      "exists": ${launchd_exists}
    },
    "windowsService": {
      "path": "$(escape_json "$windows_service_contract_path")",
      "target": "SdkworkImServer",
      "installScriptPath": "$(escape_json "$windows_service_install_script_path")",
      "uninstallScriptPath": "$(escape_json "$windows_service_uninstall_script_path")",
      "exists": ${windows_service_exists},
      "installScriptExists": ${windows_service_install_script_exists},
      "uninstallScriptExists": ${windows_service_uninstall_script_exists}
    }
  },
  "storageReport": {
    "path": "$(escape_json "$storage_report_path")",
    "exists": ${storage_report_exists}
  },
  "releaseContracts": ${release_contracts_json}
}
EOF
  exit 0
fi

echo "sdkwork-im-server status"
echo "instance: ${instance_name}"
echo "config: ${config_dir}"
echo "status: configuration-only skeleton"
echo "systemd contract: ${systemd_contract_path}"
echo "launchd contract: ${launchd_contract_path}"
echo "launchd label: com.sdkwork.im.server"
echo "windows service contract: ${windows_service_contract_path}"
echo "windows service install script: ${windows_service_install_script_path}"
echo "windows service uninstall script: ${windows_service_uninstall_script_path}"
echo "windows service target: SdkworkImServer"
echo "storage report: ${storage_report_path}"
if [[ "$release_contracts_enabled" == true ]]; then
  printf '%s\n' "$release_contracts_summary"
fi
