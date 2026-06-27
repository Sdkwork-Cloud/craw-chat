#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/uninstall-service-server.sh [--instance <name>] [--config-dir <path>]

Remove generated sdkwork-im-server service artifacts and summarize systemd/launchd/windows-service uninstall status.
EOF
}

instance_name="default"
config_dir="/etc/sdkwork/chat"

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

generated_dir="${config_dir}/generated"
rm -f \
  "${generated_dir}/sdkwork-im-server.service" \
  "${generated_dir}/com.sdkwork.im.server.plist" \
  "${generated_dir}/SdkworkImServer.xml" \
  "${generated_dir}/install-SdkworkImServer.ps1" \
  "${generated_dir}/uninstall-SdkworkImServer.ps1" \
  "${generated_dir}/service-install-report.json"
echo "Removed generated sdkwork-im-server service artifacts for instance '${instance_name}'."
echo "systemd target: sdkwork-im-server.service"
echo "launchd target: com.sdkwork.im.server"
echo "windows service target: SdkworkImServer"
