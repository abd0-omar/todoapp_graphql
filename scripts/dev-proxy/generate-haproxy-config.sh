#!/bin/sh
set -eu

if [ "${1:-}" = "" ]; then
  echo "usage: $0 <output-path>" >&2
  exit 1
fi

output_path=$1

: "${DEV_PROXY_PORT:=80}"
: "${DEV_FRONTEND_PORT:=5173}"
: "${APP_SERVER__PORT:=3000}"

frontend_host="todoapp.localhost"
api_host="api.todoapp.localhost"

cat >"$output_path" <<EOF
global
    log stdout format raw local0
    maxconn 2048

defaults
    mode http
    log global
    option httplog
    option dontlognull
    timeout connect 5s
    timeout client 60s
    timeout server 60s
    timeout tunnel 1h

frontend todoapp_frontend
    bind 127.0.0.1:${DEV_PROXY_PORT}
    acl host_frontend hdr(host),lower,field(1,:) -i ${frontend_host}
    acl host_api hdr(host),lower,field(1,:) -i ${api_host}
    use_backend frontend_backend if host_frontend
    use_backend api_backend if host_api
    default_backend frontend_backend

backend frontend_backend
    server frontend 127.0.0.1:${DEV_FRONTEND_PORT} check

backend api_backend
    server api 127.0.0.1:${APP_SERVER__PORT} check
EOF
