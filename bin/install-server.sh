#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/install-server.sh [--instance <name>] [--install-root <path>] [--config-dir <path>] [--data-dir <path>] [--log-dir <path>] [--run-dir <path>] [--non-interactive] [--force]

Create the sdkwork-api-im-standalone-gateway install/config/data/log/run directory skeleton and stage canonical payload examples.
EOF
}

instance_name="default"
install_root="/opt/sdkwork/im"
config_dir="/etc/sdkwork/im"
data_dir="/var/lib/sdkwork/im"
log_dir="/var/log/sdkwork/im"
run_dir="/run/sdkwork/im"
non_interactive=0
force_copy=0

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
      config_dir="$(server_path_for_instance "/etc/sdkwork/im" "$instance_name")"
      data_dir="$(server_path_for_instance "/var/lib/sdkwork/im" "$instance_name")"
      log_dir="$(server_path_for_instance "/var/log/sdkwork/im" "$instance_name")"
      run_dir="$(server_path_for_instance "/run/sdkwork/im" "$instance_name")"
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
    --data-dir)
      data_dir="$2"
      shift 2
      ;;
    --log-dir)
      log_dir="$2"
      shift 2
      ;;
    --run-dir)
      run_dir="$2"
      shift 2
      ;;
    --non-interactive)
      non_interactive=1
      shift
      ;;
    --force)
      force_copy=1
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

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
install_json="${config_dir}/install.json"

resolve_template_path() {
  local packaged_relative_paths="$1"
  local source_relative_path="$2"
  local source_path="${ROOT_DIR}/${source_relative_path}"

  IFS='|' read -r -a candidates <<<"$packaged_relative_paths"
  for relative_path in "${candidates[@]}"; do
    local packaged_path="${ROOT_DIR}/${relative_path}"
    if [[ -f "$packaged_path" ]]; then
      printf '%s\n' "$packaged_path"
      return 0
    fi
  done
  if [[ -f "$source_path" ]]; then
    printf '%s\n' "$source_path"
    return 0
  fi

  echo "Missing sdkwork-api-im-standalone-gateway template. Expected packaged path '${packaged_relative_paths}' or source path '${source_path}'." >&2
  return 1
}

mkdir -p "$install_root" "$config_dir" "$data_dir" "$log_dir" "$run_dir"

copy_if_needed() {
  local source_path="$1"
  local dest_path="$2"
  if [[ ! -f "$dest_path" || "$force_copy" -eq 1 ]]; then
    cp "$source_path" "$dest_path"
  fi
}

copy_if_needed "$(resolve_template_path "config/config.toml.example|config/server.yaml.example" "deployments/templates/config.toml.example")" "${config_dir}/config.toml.example"
copy_if_needed "$(resolve_template_path "config/server.env.example" "deployments/templates/server.env.example")" "${config_dir}/server.env.example"
copy_if_needed "$(resolve_template_path "config/postgresql.yaml.example|config/storage/postgresql.yaml.example" "deployments/templates/postgresql.yaml.example")" "${config_dir}/postgresql.yaml.example"

cat >"$install_json" <<EOF
{
  "product": "chat",
  "appCode": "chat",
  "instance": "${instance_name}",
  "installRoot": "${install_root}",
  "configDir": "${config_dir}",
  "dataDir": "${data_dir}",
  "logDir": "${log_dir}",
  "runDir": "${run_dir}",
  "nonInteractive": ${non_interactive}
}
EOF

echo "Prepared sdkwork-api-im-standalone-gateway directories for instance '${instance_name}'."
echo "ConfigDir: ${config_dir}"
echo "DataDir: ${data_dir}"
echo "LogDir: ${log_dir}"
echo "RunDir: ${run_dir}"
