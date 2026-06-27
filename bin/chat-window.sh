#!/usr/bin/env bash
set -euo pipefail

base_url=""
tenant_id="100001"
conversation_id=""
user_id=""
session_id=""
device_id=""
bearer_token=""
login=""
password=""
label=""
message_prefix=""
release_flag=""

usage() {
  printf '%s\n' \
    "Usage: bin/chat-window.sh --conversation-id <id> --user-id <id> [--base-url <url>] [--tenant-id <id>] [--session-id <id>] [--device-id <id>] [--bearer-token <token>] [--login <id>] [--password <secret>] [--label <name>] [--message-prefix <prefix>] [--release]" \
    "Open one interactive chat terminal backed by bin/chat-cli.sh chat-session. Default seeded IM users prefer real login before shared-secret fallback."
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base-url)
      base_url="$2"
      shift 2
      ;;
    --tenant-id)
      tenant_id="$2"
      shift 2
      ;;
    --conversation-id)
      conversation_id="$2"
      shift 2
      ;;
    --user-id)
      user_id="$2"
      shift 2
      ;;
    --session-id)
      session_id="$2"
      shift 2
      ;;
    --device-id)
      device_id="$2"
      shift 2
      ;;
    --bearer-token)
      bearer_token="$2"
      shift 2
      ;;
    --login)
      login="$2"
      shift 2
      ;;
    --password)
      password="$2"
      shift 2
      ;;
    --label)
      label="$2"
      shift 2
      ;;
    --message-prefix)
      message_prefix="$2"
      shift 2
      ;;
    --release|-Release)
      release_flag="--release"
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

read_config_value() {
  local key="$1"
  local config_file="$script_dir/../configs/topology/self-hosted.split-services.development.env"
  [[ -f "$config_file" ]] || return 1
  while IFS= read -r line || [[ -n "$line" ]]; do
    line="${line%$'\r'}"
    [[ -z "$line" || "$line" == \#* ]] && continue
    local current_key="${line%%=*}"
    local current_value="${line#*=}"
    if [[ "$current_key" == "$key" ]]; then
      printf '%s\n' "$current_value"
      return 0
    fi
  done < "$config_file"
  return 1
}

resolve_base_url() {
  if [[ -n "$base_url" ]]; then
    return 0
  fi

  local http_url
  http_url="$(read_config_value SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL || true)"
  if [[ -n "$http_url" ]]; then
    base_url="${http_url%/}"
    return 0
  fi

  local bind_address
  bind_address="$(read_config_value SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND || true)"
  if [[ -z "$bind_address" ]]; then
    base_url="http://127.0.0.1:18079"
    return 0
  fi

  local port="${bind_address##*:}"
  local host="${bind_address%:*}"
  if [[ -z "$host" || "$host" == "0.0.0.0" || "$host" == "::" || "$host" == "[::]" ]]; then
    host="127.0.0.1"
  fi
  base_url="http://${host}:${port}"
}

capture_chat_cli() {
  local args=()
  if [[ -n "$release_flag" ]]; then
    args+=("$release_flag")
  fi
  args+=("$@")

  local output
  if ! output="$("${BASH:-bash}" "$script_dir/chat-cli.sh" "${args[@]}" 2>&1)"; then
    printf '%s\n' "$output" >&2
    return 1
  fi

  printf '%s' "$output"
}

normalize_json_text() {
  local raw_text="$1"
  local found_json="false"
  local line
  local normalized=""

  while IFS= read -r line || [[ -n "$line" ]]; do
    if [[ "$found_json" != "true" ]]; then
      if [[ "$line" == *"{"* ]]; then
        found_json="true"
      else
        continue
      fi
    fi
    normalized+="$line"
  done <<< "$raw_text"

  normalized="${normalized//$'\r'/}"
  normalized="${normalized//$'\n'/}"
  normalized="${normalized//$'\t'/}"
  normalized="${normalized// /}"
  printf '%s' "$normalized"
}

extract_json_string() {
  local json_text="$1"
  local key="$2"
  local compact
  compact="$(normalize_json_text "$json_text")"
  local marker="\"${key}\":\""
  if [[ "$compact" != *"$marker"* ]]; then
    return 0
  fi

  local remainder="${compact#*$marker}"
  printf '%s' "${remainder%%\"*}"
}

extract_login_user_id() {
  local json_text="$1"
  local compact
  compact="$(normalize_json_text "$json_text")"
  local user_marker='"user":{'
  if [[ "$compact" != *"$user_marker"* ]]; then
    return 0
  fi

  local user_section="${compact#*$user_marker}"
  local id_marker='"id":"'
  if [[ "$user_section" != *"$id_marker"* ]]; then
    return 0
  fi

  local remainder="${user_section#*$id_marker}"
  printf '%s' "${remainder%%\"*}"
}

resolve_seeded_im_password() {
  case "$1" in
    1|owner)
      printf '%s\n' "Owner#2026"
      ;;
    2|guest)
      printf '%s\n' "Guest#2026"
      ;;
    3|demo)
      printf '%s\n' "Demo#2026"
      ;;
    *)
      return 1
      ;;
  esac
}

resolve_im_login() {
  local requested_user_id="$1"
  local requested_login="$2"
  if [[ -n "$requested_login" ]]; then
    printf '%s\n' "$requested_login"
    return 0
  fi

  printf '%s\n' "$requested_user_id"
}

resolve_im_password() {
  local resolved_login="$1"
  local requested_password="$2"
  if [[ -n "$requested_password" ]]; then
    printf '%s\n' "$requested_password"
    return 0
  fi

  if resolve_seeded_im_password "$resolved_login"; then
    return 0
  fi

  printf '%s\n' "No password was provided for login '$resolved_login'. Supply --login/--password for non-seeded accounts." >&2
  return 1
}

login_im_user() {
  local requested_user_id="$1"
  local resolved_login="$2"
  local resolved_password="$3"
  local resolved_session_id="$4"
  local resolved_device_id="$5"

  local login_json
  login_json="$(
    capture_chat_cli \
      --base-url "$base_url" \
      --tenant-id "$tenant_id" \
      --user-id "$requested_user_id" \
      --session-id "$resolved_session_id" \
      --device-id "$resolved_device_id" \
      login \
      --login "$resolved_login" \
      --password "$resolved_password" \
      --client-kind im_user
  )"

  local access_token
  access_token="$(extract_json_string "$login_json" "accessToken")"
  if [[ -z "$access_token" ]]; then
    printf '%s\n' "login response did not include accessToken for '$resolved_login'" >&2
    return 1
  fi

  local resolved_user_id
  resolved_user_id="$(extract_login_user_id "$login_json")"
  if [[ -z "$resolved_user_id" ]]; then
    resolved_user_id="$requested_user_id"
  fi

  printf '%s\t%s\n' "$resolved_user_id" "$access_token"
}

if [[ -z "$conversation_id" || -z "$user_id" ]]; then
  usage >&2
  exit 1
fi

if [[ -z "$label" ]]; then
  label="$user_id"
fi
if [[ -z "$session_id" ]]; then
  session_id="s_${user_id}"
fi
if [[ -z "$device_id" ]]; then
  device_id="d_${user_id}"
fi
if [[ -z "$message_prefix" ]]; then
  message_prefix="[$label] "
fi

resolve_base_url

auth_user_id="$user_id"
auth_mode="implicit-cli-default"

if [[ -n "$bearer_token" ]]; then
  auth_mode="provided-bearer"
else
  resolved_login="$(resolve_im_login "$user_id" "$login")"
  seeded_password=""
  if seeded_password="$(resolve_seeded_im_password "$resolved_login" 2>/dev/null)"; then
    :
  else
    seeded_password=""
  fi

  if [[ -n "$login" || -n "$password" || -n "$seeded_password" ]]; then
    resolved_password="$(resolve_im_password "$resolved_login" "$password")"
    IFS=$'\t' read -r auth_user_id bearer_token < <(
      login_im_user "$user_id" "$resolved_login" "$resolved_password" "$session_id" "$device_id"
    )
    auth_mode="real-login"
  fi
fi

echo "Opening chat session: conversation=$conversation_id user=$auth_user_id label=$label baseUrl=$base_url authMode=$auth_mode"
echo "Type /quit to exit."

args=()
if [[ -n "$release_flag" ]]; then
  args+=("$release_flag")
fi
args+=(
  --base-url "$base_url"
  --tenant-id "$tenant_id"
  --user-id "$auth_user_id"
  --session-id "$session_id"
  --device-id "$device_id"
  chat-session
  --conversation-id "$conversation_id"
  --label "$label"
)

if [[ -n "$bearer_token" ]]; then
  args+=(--bearer-token "$bearer_token")
fi

if [[ -n "$message_prefix" ]]; then
  args+=(--message-prefix "$message_prefix")
fi

"$script_dir/chat-cli.sh" "${args[@]}"
