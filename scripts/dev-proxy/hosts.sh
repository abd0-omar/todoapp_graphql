#!/bin/sh
set -eu

SCRIPT_PATH=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)/$(basename -- "$0")

: "${DEV_HOSTS_FILE:=/etc/hosts}"
: "${DEV_PROXY_MARKER_START:=# todoapp-dev-proxy-start}"
: "${DEV_PROXY_MARKER_END:=# todoapp-dev-proxy-end}"

usage() {
  echo "usage: $0 <sync|clean>" >&2
}

ensure_root() {
  if [ "$(id -u)" -eq 0 ] || [ -w "$DEV_HOSTS_FILE" ]; then
    return
  fi

  exec sudo --preserve-env=DEV_HOSTS_FILE,DEV_PROXY_MARKER_START,DEV_PROXY_MARKER_END "$SCRIPT_PATH" "$@"
}

strip_managed_block() {
  awk -v start="$DEV_PROXY_MARKER_START" -v end="$DEV_PROXY_MARKER_END" '
    $0 == start { skip = 1; next }
    $0 == end { skip = 0; next }
    !skip { print }
  ' "$DEV_HOSTS_FILE"
}

trim_trailing_blanks() {
  awk '
    {
      lines[NR] = $0
      if ($0 !~ /^[[:space:]]*$/) {
        last = NR
      }
    }
    END {
      for (i = 1; i <= last; i++) {
        print lines[i]
      }
    }
  '
}

collect_hosts() {
  {
    printf '%s\n' "todoapp.localhost"
    printf '%s\n' "api.todoapp.localhost"
  } | awk 'NF && !seen[$0]++'
}

clean_hosts() {
  tmp_file=$(mktemp)
  strip_managed_block | trim_trailing_blanks >"$tmp_file"
  printf '\n' >>"$tmp_file"
  cat "$tmp_file" >"$DEV_HOSTS_FILE"
  rm -f "$tmp_file"
}

sync_hosts() {
  tmp_dir=$(mktemp -d)
  tmp_file="${tmp_dir}/hosts"
  tmp_out="${tmp_dir}/out"

  strip_managed_block | trim_trailing_blanks >"$tmp_file"
  {
    if [ -s "$tmp_file" ]; then
      cat "$tmp_file"
      printf '\n'
    fi
    printf '%s\n' "$DEV_PROXY_MARKER_START"
    collect_hosts | while IFS= read -r host; do
      printf '127.0.0.1 %s\n' "$host"
    done
    printf '%s\n' "$DEV_PROXY_MARKER_END"
    printf '\n'
  } >"$tmp_out"

  cat "$tmp_out" >"$DEV_HOSTS_FILE"
  rm -rf "$tmp_dir"
}

if [ ! -f "$DEV_HOSTS_FILE" ]; then
  echo "hosts file not found: $DEV_HOSTS_FILE" >&2
  exit 1
fi

command_name=${1:-}
case "$command_name" in
  sync)
    ensure_root "$@"
    sync_hosts
    ;;
  clean)
    ensure_root "$@"
    clean_hosts
    ;;
  *)
    usage
    exit 1
    ;;
esac
