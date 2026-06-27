#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash tools/smoke/local_stack_smoke.sh [--base-url <url>]

Run a minimal local-stack smoke check against the self-hosted.split-services.development deployment profile.
EOF
}

DEFAULT_BASE_URL="http://127.0.0.1:18079"
DEFAULT_HEALTH_URL="http://127.0.0.1:18079/healthz"
base_url="$DEFAULT_BASE_URL"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base-url)
      if [[ $# -lt 2 ]]; then
        echo "--base-url requires a value" >&2
        exit 1
      fi
      base_url="$2"
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

CONTENT_TYPE_HEADER="Content-Type: application/json"

have_curl() {
  command -v curl >/dev/null 2>&1
}

have_wget() {
  command -v wget >/dev/null 2>&1
}

base64url() {
  local raw="$1"
  if have_openssl; then
    printf '%s' "$raw" | openssl base64 -A | tr '+/' '-_' | tr -d '='
    return
  fi

  printf '%s' "$raw" | base64 | tr -d '\n' | tr '+/' '-_' | tr -d '='
}

have_openssl() {
  command -v openssl >/dev/null 2>&1
}

local_jwt() {
  local claims="$1"
  local header=""
  local payload=""
  header="$(base64url '{"alg":"none","typ":"JWT"}')"
  payload="$(base64url "$claims")"
  printf '%s.%s.local\n' "$header" "$payload"
}

AUTH_TOKEN="$(local_jwt '{"tenant_id":"100001","login_scope":"TENANT","user_id":"1","session_id":"s_demo","app_id":"sdkwork-im","auth_level":"password","subject_type":"user"}')"
ACCESS_TOKEN="$(local_jwt '{"tenant_id":"100001","login_scope":"TENANT","user_id":"1","session_id":"s_demo","app_id":"sdkwork-im","environment":"dev","deployment_mode":"saas","auth_level":"password","actor_id":"1","actor_kind":"user","device_id":"d_demo","data_scope":["tenant"],"permission_scope":["chat.write"],"subject_type":"user"}')"
DUAL_TOKEN_HEADERS=(
  "Authorization: Bearer ${AUTH_TOKEN}"
  "Access-Token: ${ACCESS_TOKEN}"
)

curl_dual_token_args() {
  local args=()
  local header
  for header in "${DUAL_TOKEN_HEADERS[@]}"; do
    args+=("-H" "$header")
  done
  printf '%s\n' "${args[@]}"
}

wget_dual_token_args() {
  local args=()
  local header
  for header in "${DUAL_TOKEN_HEADERS[@]}"; do
    args+=("--header=$header")
  done
  printf '%s\n' "${args[@]}"
}

http_get() {
  local url="$1"

  if have_curl; then
    mapfile -t dual_token_args < <(curl_dual_token_args)
    curl --fail --silent --show-error "${dual_token_args[@]}" "$url"
    return
  fi

  if have_wget; then
    mapfile -t dual_token_args < <(wget_dual_token_args)
    wget -q -O - "${dual_token_args[@]}" "$url"
    return
  fi

  echo "Neither curl nor wget is available for smoke verification." >&2
  exit 1
}

http_post() {
  local url="$1"
  local body="$2"

  if have_curl; then
    mapfile -t dual_token_args < <(curl_dual_token_args)
    curl --fail --silent --show-error \
      -X POST \
      "${dual_token_args[@]}" \
      -H "$CONTENT_TYPE_HEADER" \
      -d "$body" \
      "$url"
    return
  fi

  if have_wget; then
    mapfile -t dual_token_args < <(wget_dual_token_args)
    wget -q -O - \
      --method=POST \
      "${dual_token_args[@]}" \
      --header="$CONTENT_TYPE_HEADER" \
      --body-data="$body" \
      "$url"
    return
  fi

  echo "Neither curl nor wget is available for smoke verification." >&2
  exit 1
}

wait_healthy() {
  local url="$1"
  local health_url="${url}/healthz"

  if [[ "$url" == "$DEFAULT_BASE_URL" ]]; then
    health_url="$DEFAULT_HEALTH_URL"
  fi

  for _ in $(seq 1 20); do
    if have_curl; then
      if curl --fail --silent --show-error "$health_url" >/dev/null 2>&1; then
        return
      fi
    elif have_wget; then
      if wget -q -O /dev/null "$health_url" >/dev/null 2>&1; then
        return
      fi
    else
      echo "Neither curl nor wget is available for smoke verification." >&2
      exit 1
    fi

    sleep 2
  done

  echo "Timed out waiting for ${base_url}/healthz" >&2
  exit 1
}

normalize_json() {
  tr -d '\r\n\t '
}

wait_healthy "$base_url"

conversation_id="c_smoke_$(date +%s)_$$"

create_body="$(cat <<EOF
{"conversationId":"${conversation_id}","conversationType":"group"}
EOF
)"
http_post "${base_url}/im/v3/api/chat/conversations" "$create_body" >/dev/null

message_body="$(cat <<'EOF'
{"clientMsgId":"smoke_client","summary":"smoke","text":"smoke"}
EOF
)"
http_post "${base_url}/im/v3/api/chat/conversations/${conversation_id}/messages" "$message_body" >/dev/null

summary_response="$(http_get "${base_url}/im/v3/api/chat/conversations/${conversation_id}")"
summary_compact="$(printf '%s' "$summary_response" | normalize_json)"
if [[ "$summary_compact" != *'"lastSummary":"smoke"'* ]]; then
  echo "Unexpected conversation summary payload" >&2
  exit 1
fi

echo "local stack smoke check passed."
