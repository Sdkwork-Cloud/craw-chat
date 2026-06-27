#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/init-config-server.sh [--instance <name>] [--config-dir <path>] [--data-dir <path>] [--log-dir <path>] [--run-dir <path>] [--bind-address <host:port>] [--base-url <url>] [--api-base-url <url>] [--websocket-base-url <url>] [--browser-origins <csv>] [--non-interactive] [--force]

Render sdkwork-im-server configuration files for the selected instance and preserve file-based PostgreSQL settings.
EOF
}

instance_name="default"
config_dir="/etc/sdkwork/chat"
data_dir="/var/lib/sdkwork/chat"
log_dir="/var/log/sdkwork/chat"
run_dir="/run/sdkwork/chat"
bind_address="0.0.0.0:18080"
base_url="http://127.0.0.1:18080"
api_base_url="http://127.0.0.1:18080"
websocket_base_url="ws://127.0.0.1:18080"
browser_origins="http://127.0.0.1:18080,http://localhost:18080"
non_interactive=0
force_write=0

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
      data_dir="$(server_path_for_instance "/var/lib/sdkwork/chat" "$instance_name")"
      log_dir="$(server_path_for_instance "/var/log/sdkwork/chat" "$instance_name")"
      run_dir="$(server_path_for_instance "/run/sdkwork/chat" "$instance_name")"
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
    --bind-address)
      bind_address="$2"
      shift 2
      ;;
    --base-url)
      base_url="$2"
      shift 2
      ;;
    --api-base-url)
      api_base_url="$2"
      shift 2
      ;;
    --websocket-base-url)
      websocket_base_url="$2"
      shift 2
      ;;
    --browser-origins)
      browser_origins="$2"
      shift 2
      ;;
    --non-interactive)
      non_interactive=1
      shift
      ;;
    --force)
      force_write=1
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

server_env="${config_dir}/server.env"
chat_toml="${config_dir}/chat.toml"
postgresql_yaml="${config_dir}/postgresql.yaml"
password_file="${config_dir}/database.secret"
redis_secret="${config_dir}/redis.secret"

mkdir -p "$config_dir" "$data_dir" "$log_dir" "$run_dir"

write_if_needed() {
  local path="$1"
  local content="$2"
  if [[ ! -f "$path" || "$force_write" -eq 1 ]]; then
    printf '%s' "$content" >"$path"
  fi
}

write_if_needed "$chat_toml" "[runtime]
environment = \"production\"
deployment_mode = \"server\"
app_code = \"chat\"

[server]
bind_address = \"${bind_address}\"
trust_forwarded_headers = true

[public_endpoints]
base_url = \"${base_url}\"
api_base_url = \"${api_base_url}\"
websocket_base_url = \"${websocket_base_url}\"
docs_base_url = \"${base_url}/docs\"

[paths]
config_directory = \"${config_dir}\"
config_file = \"${chat_toml}\"
data_directory = \"${data_dir}\"
log_directory = \"${log_dir}\"
cache_directory = \"${data_dir}/cache\"
runtime_directory = \"${run_dir}\"

[database]
engine = \"postgresql\"
host = \"127.0.0.1\"
port = 5432
database = \"sdkwork\"
schema = \"sdkwork\"
username = \"sdkwork\"
password_file = \"${password_file}\"
ssl_mode = \"require\"
max_connections = 20

[redis]
enabled = true
host = \"redis.example.com\"
port = 6379
database = 0
password_file = \"${redis_secret}\"
key_prefix = \"chat\"
tls = false
max_connections = 16
"

write_if_needed "$server_env" "SDKWORK_IM_DEPLOYMENT_MODE=server
SDKWORK_IM_CONFIG_FILE=${chat_toml}
SDKWORK_IM_DATA_DIR=${data_dir}
SDKWORK_IM_LOG_DIR=${log_dir}
SDKWORK_IM_RUN_DIR=${run_dir}
SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=${bind_address}
SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=${api_base_url}
SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=${websocket_base_url}
SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=http://127.0.0.1:3900
SDKWORK_IM_DATABASE_ENGINE=postgresql
SDKWORK_IM_DATABASE_HOST=127.0.0.1
SDKWORK_IM_DATABASE_PORT=5432
SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_prod
SDKWORK_CLAW_DATABASE_SCHEMA=sdkwork_ai_prod_chat_prod
SDKWORK_CLAW_DATABASE_USERNAME=sdkwork_ai_prod
SDKWORK_IM_DATABASE_PASSWORD_FILE=${password_file}
SDKWORK_IM_DATABASE_SSL_MODE=require
SDKWORK_IM_DATABASE_MAX_CONNECTIONS=20
SDKWORK_IM_BROWSER_ORIGINS=${browser_origins}
"

write_if_needed "$postgresql_yaml" "provider: postgresql

connection:
  host: 127.0.0.1
  port: 5432
  database: sdkwork
  username: sdkwork
  passwordFile: \"${password_file}\"
  sslmode: require
  applicationName: sdkwork-chat-server
  connectTimeoutSeconds: 10

schema:
  name: sdkwork
  provisioningMode: none
  migrationMode: apply
  expectedVersion: latest

pool:
  minConnections: 5
  maxConnections: 30
  idleTimeoutSeconds: 300
  maxLifetimeSeconds: 1800

# init-storage-server modes:
# - verify-only
# - bootstrap-schema
# - create-db-and-schema
"

if [[ ! -f "$password_file" || "$force_write" -eq 1 ]]; then
  printf '%s\n' "replace-me" >"$password_file"
fi

echo "Rendered sdkwork-im-server configuration for instance '${instance_name}'."
echo "chat.toml: ${chat_toml}"
echo "server.env: ${server_env}"
echo "postgresql.yaml: ${postgresql_yaml}"
