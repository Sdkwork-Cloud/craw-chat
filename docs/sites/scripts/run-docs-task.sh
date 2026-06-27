#!/usr/bin/env sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
entry="$script_dir/run-docs-task.mjs"

if [ -n "${npm_node_execpath:-}" ]; then
  exec "$npm_node_execpath" "$entry" "$@"
fi

if [ -n "${NODE:-}" ]; then
  exec "$NODE" "$entry" "$@"
fi

exec node "$entry" "$@"
