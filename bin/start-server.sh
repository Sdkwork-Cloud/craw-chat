#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/start-server.sh [--instance <name>] [--install-root <path>] [--config-dir <path>] [--log-dir <path>] [--run-dir <path>] [--env-file <path>] [--binary-path <path>] [--release] [--foreground] [--health-url <url>] [--skip-health-check]

Start the sdkwork-im-server runtime service for an instance with config loading, binary resolution, log and run directory management, health checks, and status-friendly foreground or background execution.
EOF
}

instance_name="default"
install_root="/opt/sdkwork/chat"
config_dir="/etc/sdkwork/chat"
log_dir="/var/log/sdkwork/chat"
run_dir="/run/sdkwork/chat"
env_file=""
binary_path=""
release_mode=0
foreground=0
health_url=""
skip_health_check=0

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
      run_dir="$(server_path_for_instance "/run/sdkwork/chat" "$instance_name")"
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
    --run-dir)
      run_dir="$2"
      shift 2
      ;;
    --env-file)
      env_file="$2"
      shift 2
      ;;
    --binary-path)
      binary_path="$2"
      shift 2
      ;;
    --release)
      release_mode=1
      shift
      ;;
    --foreground)
      foreground=1
      shift
      ;;
    --health-url)
      health_url="$2"
      shift 2
      ;;
    --skip-health-check)
      skip_health_check=1
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
if [[ -z "$env_file" ]]; then
  env_file="${config_dir}/server.env"
fi
server_yaml="${config_dir}/chat.toml"

read_yaml_value() {
  local file="$1"
  local key="$2"
  awk -F'[:=]' -v key="$key" '
    $1 ~ key"$" {
      value = $2
      gsub(/"/, "", value)
      gsub(/\047/, "", value)
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", value)
      print value
      exit
    }
  ' "$file"
}

load_env_file() {
  local env_file_path="$1"
  local raw_line
  local line
  local normalized_line
  local key
  local value
  [[ -f "$env_file_path" ]] || return 0

  while IFS= read -r raw_line || [[ -n "$raw_line" ]]; do
    line="${raw_line%$'\r'}"
    [[ -z "${line//[[:space:]]/}" || "$line" == \#* ]] && continue
    normalized_line="$line"
    if [[ "$normalized_line" == export\ * ]]; then
      normalized_line="${normalized_line#export }"
    fi
    [[ "$normalized_line" == *=* ]] || continue
    key="${normalized_line%%=*}"
    value="${normalized_line#*=}"
    key="$(printf '%s' "$key" | xargs)"
    [[ -n "$key" ]] || continue
    if [[ ${value:0:1} == '"' && ${value: -1} == '"' ]]; then
      value="${value:1:${#value}-2}"
    elif [[ ${value:0:1} == "'" && ${value: -1} == "'" ]]; then
      value="${value:1:${#value}-2}"
    fi
    if [[ -z "${!key+x}" ]]; then
      export "${key}=${value}"
    fi
  done <"$env_file_path"
}

resolve_binary_path() {
  local explicit="$1"
  local prefer_release="$2"
  if [[ -n "$explicit" && -x "$explicit" ]]; then
    printf '%s\n' "$explicit"
    return 0
  fi
  if [[ -n "${SDKWORK_IM_SERVER_BINARY_PATH:-}" && -x "${SDKWORK_IM_SERVER_BINARY_PATH}" ]]; then
    printf '%s\n' "${SDKWORK_IM_SERVER_BINARY_PATH}"
    return 0
  fi

  for candidate in \
    "${install_root}/bin/sdkwork-im-server" \
    "${install_root}/bin/sdkwork-im-cloud-gateway"; do
    if [[ -x "$candidate" ]]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done

  local debug_candidate="${ROOT_DIR}/target/debug/sdkwork-im-server"
  local release_candidate="${ROOT_DIR}/target/release/sdkwork-im-server"
  local legacy_debug_candidate="${ROOT_DIR}/target/debug/sdkwork-im-cloud-gateway"
  local legacy_release_candidate="${ROOT_DIR}/target/release/sdkwork-im-cloud-gateway"
  if [[ "$prefer_release" -eq 1 ]]; then
    for candidate in "$release_candidate" "$debug_candidate" "$legacy_release_candidate" "$legacy_debug_candidate"; do
      [[ -x "$candidate" ]] && printf '%s\n' "$candidate" && return 0
    done
  else
    for candidate in "$debug_candidate" "$release_candidate" "$legacy_debug_candidate" "$legacy_release_candidate"; do
      [[ -x "$candidate" ]] && printf '%s\n' "$candidate" && return 0
    done
  fi

  if command -v cargo >/dev/null 2>&1; then
    if [[ "$prefer_release" -eq 1 ]]; then
      cargo build --release -p sdkwork-im-cloud-gateway --offline
    else
      cargo build -p sdkwork-im-cloud-gateway --offline
    fi
    resolve_binary_path "" "$prefer_release"
    return 0
  fi
  return 1
}

resolve_health_url() {
  local explicit="$1"
  local bind_address="$2"
  if [[ -n "$explicit" ]]; then
    printf '%s\n' "$explicit"
    return 0
  fi
  local host="${bind_address%:*}"
  local port="${bind_address##*:}"
  if [[ -z "$host" || "$host" == "0.0.0.0" || "$host" == "::" || "$host" == "[::]" ]]; then
    host="127.0.0.1"
  fi
  printf 'http://%s:%s/healthz\n' "$host" "$port"
}

load_env_file "$env_file"

if [[ -n "${SDKWORK_IM_CONFIG_FILE:-}" ]]; then
  server_yaml="${SDKWORK_IM_CONFIG_FILE}"
elif [[ ! -f "$server_yaml" && -f "${config_dir}/server.yaml" ]]; then
  server_yaml="${config_dir}/server.yaml"
fi
[[ -f "$server_yaml" ]] || { echo "Missing server config. Run init-config-server first: ${server_yaml}" >&2; exit 1; }

bind_address="${SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND:-}"
if [[ -z "$bind_address" ]]; then
  bind_address="$(read_yaml_value "$server_yaml" "bind_address")"
fi
if [[ -z "$bind_address" ]]; then
  bind_address="$(read_yaml_value "$server_yaml" "bindAddress")"
fi
[[ -n "$bind_address" ]] || bind_address="127.0.0.1:18079"
resolved_binary="$(resolve_binary_path "$binary_path" "$release_mode" || true)"
[[ -n "$resolved_binary" ]] || { echo "Unable to resolve sdkwork-im-server binary. Set --binary-path, install a packaged binary, or build sdkwork-im-cloud-gateway." >&2; exit 1; }
resolved_health_url="$(resolve_health_url "$health_url" "$bind_address")"

mkdir -p "$log_dir" "$run_dir"
stdout_log="${log_dir}/sdkwork-im-server.out.log"
stderr_log="${log_dir}/sdkwork-im-server.err.log"
pid_file="${run_dir}/sdkwork-im-server.pid"
process_info="${run_dir}/sdkwork-im-server.process.json"
process_name="$(basename "${resolved_binary}")"

if [[ -f "$pid_file" ]]; then
  current_pid="$(head -n 1 "$pid_file" | tr -d '\r\n')"
  if [[ -n "$current_pid" ]] && kill -0 "$current_pid" >/dev/null 2>&1; then
    current_name="$(ps -p "$current_pid" -o comm= | tr -d '[:space:]' || true)"
    if [[ "$current_name" == "${process_name}" || "$current_name" == "${process_name%.*}" ]]; then
      echo "sdkwork-im-server is already running with PID ${current_pid}." >&2
      exit 1
    fi
  fi
fi

export SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND="$bind_address"
export SDKWORK_IM_WEB_GATEWAY_BIND="$bind_address"
server_args=(--config "$server_yaml")

if [[ "$foreground" -eq 1 ]]; then
  exec "$resolved_binary" "${server_args[@]}"
fi

"$resolved_binary" "${server_args[@]}" >"$stdout_log" 2>"$stderr_log" &
server_pid=$!
printf '%s\n' "$server_pid" >"$pid_file"
cat >"$process_info" <<EOF
{
  "binaryPath": "${resolved_binary}",
  "processName": "${process_name}",
  "bindAddress": "${bind_address}",
  "healthUrl": "${resolved_health_url}"
}
EOF

if [[ "$skip_health_check" -eq 0 ]]; then
  for _ in $(seq 1 30); do
    sleep 1
    if ! kill -0 "$server_pid" >/dev/null 2>&1; then
      rm -f "$pid_file"
      echo "sdkwork-im-server exited before becoming healthy. Check logs: ${stderr_log}" >&2
      exit 1
    fi
    if command -v curl >/dev/null 2>&1; then
      if curl -fsS "$resolved_health_url" >/dev/null 2>&1; then
        echo "Started sdkwork-im-server in background on ${resolved_health_url}"
        exit 0
      fi
    elif command -v wget >/dev/null 2>&1; then
      if wget -q -O - "$resolved_health_url" >/dev/null 2>&1; then
        echo "Started sdkwork-im-server in background on ${resolved_health_url}"
        exit 0
      fi
    fi
  done
  kill "$server_pid" >/dev/null 2>&1 || true
  rm -f "$pid_file"
  echo "sdkwork-im-server did not become healthy within 30 seconds: ${resolved_health_url}" >&2
  exit 1
fi

echo "Started sdkwork-im-server in background without health wait."
