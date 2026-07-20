#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/install-service-server.sh [--instance <name>] [--install-root <path>] [--config-dir <path>] [--log-dir <path>] [--service-mode <auto|systemd|launchd|windows-service>]

Render the sdkwork-api-im-standalone-gateway service contract, generate systemd and launchd targets, generate Windows Service wrapper targets, and report install status.
EOF
}

instance_name="default"
install_root="/opt/sdkwork/chat"
config_dir="/etc/sdkwork/chat"
log_dir="/var/log/sdkwork/chat"
service_mode="auto"

server_path_for_instance() {
  local root="$1"
  local name="$2"
  if [[ "$name" == "default" ]]; then
    printf '%s\n' "$root"
  else
    printf '%s/instances/%s\n' "$root" "$name"
  fi
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --instance)
      instance_name="$2"
      config_dir="$(server_path_for_instance "/etc/sdkwork/chat" "$instance_name")"
      log_dir="$(server_path_for_instance "/var/log/sdkwork/chat" "$instance_name")"
      shift 2
      ;;
    --install-root)
      install_root="$2"
      shift 2
      ;;
    --config-dir)
      config_dir="$2"
      shift 2
      ;;
    --log-dir)
      log_dir="$2"
      shift 2
      ;;
    --service-mode)
      service_mode="$2"
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

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
systemd_template="${ROOT_DIR}/deployments/systemd/sdkwork-api-im-standalone-gateway.service"
launchd_template="${ROOT_DIR}/deployments/launchd/com.sdkwork.im.api-standalone-gateway.plist"
windows_service_template="${ROOT_DIR}/deployments/windows-service/sdkwork-api-im-standalone-gateway-service.xml"
generated_dir="${config_dir}/generated"
mkdir -p "$generated_dir" "$log_dir"
generated_unit="${generated_dir}/sdkwork-api-im-standalone-gateway.service"
generated_launchd_plist="${generated_dir}/com.sdkwork.im.api-standalone-gateway.plist"
generated_windows_service_xml="${generated_dir}/sdkwork-api-im-standalone-gateway-service.xml"
generated_windows_service_install_script="${generated_dir}/install-sdkwork-api-im-standalone-gateway-service.ps1"
generated_windows_service_uninstall_script="${generated_dir}/uninstall-sdkwork-api-im-standalone-gateway-service.ps1"
service_binary_path="${install_root}/bin/sdkwork-api-im-standalone-gateway"
windows_service_wrapper_exe="${install_root}/bin/sdkwork-api-im-standalone-gateway-service.exe"
windows_service_wrapper_xml_target="${install_root}/bin/sdkwork-api-im-standalone-gateway-service.xml"
stdout_log_path="${log_dir}/sdkwork-api-im-standalone-gateway.out.log"
stderr_log_path="${log_dir}/sdkwork-api-im-standalone-gateway.err.log"

if [[ -f "$systemd_template" ]]; then
  sed \
    -e "s|WorkingDirectory=/opt/sdkwork/chat|WorkingDirectory=${install_root}|g" \
    -e "s|EnvironmentFile=/etc/sdkwork/chat/server.env|EnvironmentFile=${config_dir}/server.env|g" \
    -e "s|ExecStart=/opt/sdkwork/chat/bin/sdkwork-api-im-standalone-gateway|ExecStart=${service_binary_path}|g" \
    "$systemd_template" >"$generated_unit"
fi

if [[ -f "$launchd_template" ]]; then
  sed \
    -e "s|__INSTALL_ROOT__/bin/sdkwork-api-im-standalone-gateway|${service_binary_path}|g" \
    -e "s|__LOG_DIR__/sdkwork-api-im-standalone-gateway.out.log|${stdout_log_path}|g" \
    -e "s|__LOG_DIR__/sdkwork-api-im-standalone-gateway.err.log|${stderr_log_path}|g" \
    -e "s|__INSTALL_ROOT__|${install_root}|g" \
    -e "s|__CONFIG_DIR__|${config_dir}|g" \
    -e "s|__LOG_DIR__|${log_dir}|g" \
    "$launchd_template" >"$generated_launchd_plist"
fi

if [[ -f "$windows_service_template" ]]; then
  sed \
    -e "s|__INSTALL_ROOT__|${install_root}|g" \
    -e "s|__CONFIG_DIR__|${config_dir}|g" \
    -e "s|__LOG_DIR__|${log_dir}|g" \
    "$windows_service_template" >"$generated_windows_service_xml"
fi

cat >"$generated_windows_service_install_script" <<EOF
\$ErrorActionPreference = "Stop"
\$wrapperExePath = "${windows_service_wrapper_exe}"
\$wrapperConfigSourcePath = "${generated_windows_service_xml}"
\$wrapperConfigTargetPath = "${windows_service_wrapper_xml_target}"

if (-not (Test-Path \$wrapperExePath)) {
    throw "Missing Windows Service wrapper executable: \$wrapperExePath. Bundle a dedicated service-host wrapper before registration."
}
if (-not (Test-Path \$wrapperConfigSourcePath)) {
    throw "Missing generated Windows Service wrapper config: \$wrapperConfigSourcePath"
}

Copy-Item -LiteralPath \$wrapperConfigSourcePath -Destination \$wrapperConfigTargetPath -Force
& \$wrapperExePath install
EOF

cat >"$generated_windows_service_uninstall_script" <<EOF
\$ErrorActionPreference = "Stop"
\$wrapperExePath = "${windows_service_wrapper_exe}"
\$wrapperConfigTargetPath = "${windows_service_wrapper_xml_target}"

if (Test-Path \$wrapperExePath) {
    & \$wrapperExePath uninstall
}
if (Test-Path \$wrapperConfigTargetPath) {
    Remove-Item -LiteralPath \$wrapperConfigTargetPath -Force
}
EOF

cat >"${generated_dir}/service-install-report.json" <<EOF
{
  "product": "sdkwork-api-im-standalone-gateway",
  "instance": "${instance_name}",
  "installRoot": "${install_root}",
  "configDir": "${config_dir}",
  "logDir": "${log_dir}",
  "serviceMode": "${service_mode}",
  "systemdUnit": "${generated_unit}",
  "launchdPlist": "${generated_launchd_plist}",
  "launchdLabel": "com.sdkwork.im.api-standalone-gateway",
  "windowsServiceHostMode": "wrapper-required",
  "windowsServiceName": "sdkwork-api-im-standalone-gateway",
  "windowsServiceWrapperExe": "${windows_service_wrapper_exe}",
  "windowsServiceWrapperConfig": "${generated_windows_service_xml}",
  "windowsServiceInstallScript": "${generated_windows_service_install_script}",
  "windowsServiceUninstallScript": "${generated_windows_service_uninstall_script}"
}
EOF

echo "sdkwork-api-im-standalone-gateway service install summary"
echo "instance: ${instance_name}"
echo "install: ${install_root}"
echo "config: ${config_dir}"
echo "status: generated service contract"
echo "systemd template: ${systemd_template}"
echo "systemd unit: ${generated_unit}"
echo "launchd template: ${launchd_template}"
echo "launchd plist: ${generated_launchd_plist}"
echo "windows service template: ${windows_service_template}"
echo "windows service wrapper config: ${generated_windows_service_xml}"
echo "windows service install script: ${generated_windows_service_install_script}"
echo "windows service uninstall script: ${generated_windows_service_uninstall_script}"
echo "launchd target: com.sdkwork.im.api-standalone-gateway"
echo "windows service target: sdkwork-api-im-standalone-gateway"
