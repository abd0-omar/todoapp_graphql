#!/bin/sh
set -eu

SCRIPT_PATH=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)/$(basename -- "$0")

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

if [ "$(id -u)" -ne 0 ]; then
  resolved_bin=$(resolve_haproxy_bin)
  export HAPROXY_BIN="$resolved_bin"
  exec sudo --preserve-env=HAPROXY_BIN "$SCRIPT_PATH"
fi

if ! command -v setcap >/dev/null 2>&1; then
  echo "setcap is required but not available. Install libcap tools first." >&2
  exit 1
fi

haproxy_bin=$(resolve_haproxy_bin)
setcap "cap_net_bind_service=+ep" "$haproxy_bin"

echo "[dev-proxy] configured CAP_NET_BIND_SERVICE on: $haproxy_bin"
if command -v getcap >/dev/null 2>&1; then
  getcap "$haproxy_bin" || true
fi

echo "[dev-proxy] done. Daily 'make start' should no longer require sudo for port 80."
