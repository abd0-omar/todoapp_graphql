#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

default_state_dir="/tmp/todoapp_graphql-dev-proxy-${USER:-$(id -u)}"
: "${DEV_PROXY_STATE_DIR:=$default_state_dir}"
: "${DEV_PROXY_PORT:=80}"
: "${DEV_FRONTEND_PORT:=5173}"
: "${APP_SERVER__PORT:=3000}"

frontend_host="todoapp.localhost"
api_host="api.todoapp.localhost"

resolve_haproxy_bin() {
  if [ -n "${HAPROXY_BIN:-}" ]; then
    if [ -x "$HAPROXY_BIN" ]; then
      printf '%s\n' "$HAPROXY_BIN"
      return
    fi
    echo "Warning: HAPROXY_BIN is set but not executable: $HAPROXY_BIN" >&2
    echo "Warning: falling back to auto-detection." >&2
  fi

  for candidate in \
    "${HOME}/programs/haproxy/sbin/haproxy" \
    "${HOME}/programs/haproxy/haproxy"
  do
    if [ -x "$candidate" ]; then
      printf '%s\n' "$candidate"
      return
    fi
  done

  # Support versioned local installs like ~/programs/haproxy-3.3.6/haproxy.
  for candidate in "${HOME}"/programs/haproxy-*/haproxy "${HOME}"/programs/haproxy-*/sbin/haproxy; do
    [ -e "$candidate" ] || continue
    if [ -x "$candidate" ]; then
      printf '%s\n' "$candidate"
      return
    fi
  done

  for candidate in /usr/local/sbin/haproxy /usr/sbin/haproxy /sbin/haproxy; do
    if [ -x "$candidate" ]; then
      printf '%s\n' "$candidate"
      return
    fi
  done

  if command -v haproxy >/dev/null 2>&1; then
    command -v haproxy
    return
  fi

  echo "Could not find haproxy binary. Set HAPROXY_BIN=/absolute/path/to/haproxy." >&2
  exit 1
}

can_bind_without_sudo() {
  haproxy_bin=$1

  if [ "$(id -u)" -eq 0 ]; then
    return 0
  fi

  if [ "$DEV_PROXY_PORT" -ge 1024 ]; then
    return 0
  fi

  if command -v getcap >/dev/null 2>&1 && getcap "$haproxy_bin" 2>/dev/null | grep -q "cap_net_bind_service"; then
    return 0
  fi

  return 1
}

haproxy_bin=$(resolve_haproxy_bin)
if ! can_bind_without_sudo "$haproxy_bin"; then
  echo "Cannot bind 127.0.0.1:${DEV_PROXY_PORT} without elevated privileges." >&2
  echo "Run one-time setup to allow no-sudo startup:" >&2
  echo "  make proxy-setup-no-sudo" >&2
  echo "Or use an unprivileged port for this run:" >&2
  echo "  DEV_PROXY_PORT=1355 make start" >&2
  exit 1
fi

mkdir -p "$DEV_PROXY_STATE_DIR"
config_path="${DEV_PROXY_STATE_DIR}/haproxy.cfg"

"${SCRIPT_DIR}/generate-haproxy-config.sh" "$config_path"
"$haproxy_bin" -c -f "$config_path" >/dev/null

echo "[dev-proxy] http://${frontend_host} -> 127.0.0.1:${DEV_FRONTEND_PORT}"
echo "[dev-proxy] http://${api_host} -> 127.0.0.1:${APP_SERVER__PORT}"
echo "[dev-proxy] using ${haproxy_bin} on 127.0.0.1:${DEV_PROXY_PORT}"

exec "$haproxy_bin" -db -f "$config_path"
