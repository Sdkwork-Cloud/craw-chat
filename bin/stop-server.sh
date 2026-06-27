#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/stop-server.sh [--instance <name>] [--config-dir <path>] [--run-dir <path>]

Stop the sdkwork-im-server runtime service for an instance by using the pid file under the run directory, honoring config ownership, and reporting status.
EOF
}

instance_name="default"
config_dir="/etc/sdkwork/chat"
run_dir="/run/sdkwork/chat"

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
      run_dir="$(server_path_for_instance "/run/sdkwork/chat" "$instance_name")"
      shift 2
      ;;
    --config-dir)
      config_dir="$2"
      shift 2
      ;;
    --run-dir)
      run_dir="$2"
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

pid_file="${run_dir}/sdkwork-im-server.pid"
process_info="${run_dir}/sdkwork-im-server.process.json"
if [[ ! -f "$pid_file" ]]; then
  echo "sdkwork-im-server is not running."
  exit 0
fi

server_pid="$(head -n 1 "$pid_file" | tr -d '\r\n')"
if [[ -z "$server_pid" ]]; then
  rm -f "$pid_file"
  echo "sdkwork-im-server pid file was empty and has been cleared."
  exit 0
fi

if kill -0 "$server_pid" >/dev/null 2>&1; then
  kill "$server_pid" >/dev/null 2>&1 || true
  for _ in $(seq 1 30); do
    if ! kill -0 "$server_pid" >/dev/null 2>&1; then
      break
    fi
    sleep 1
  done
  echo "Stopped sdkwork-im-server PID ${server_pid}"
else
  echo "sdkwork-im-server process from pid file is not running."
fi

rm -f "$pid_file" "$process_info"
