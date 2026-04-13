#!/bin/bash

set -euo pipefail

APP_SERVER__IP="${APP_SERVER__IP:-127.0.0.1}"
APP_SERVER__PORT="${APP_SERVER__PORT:-3000}"
DEFAULT_BASE_URL="http://${APP_SERVER__IP}:${APP_SERVER__PORT}"
BASE_URL="${BASE_URL:-$DEFAULT_BASE_URL}"

# Run all hurl test files. Each file captures its own auth token.
for file in $(find tests -type f -name '*.hurl' | sort -V); do
    hurl --test \
      --variables-file tests/variables.env \
      --variable "base_url=$BASE_URL" \
      --error-format long \
      "$file"
done
