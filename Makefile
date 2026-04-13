POSTGRES_CONTAINER_NAME := todoapp_graphql_postgres
POSTGRES_USER := todoapp_graphql
POSTGRES_PASSWORD := todoapp_graphql
POSTGRES_DB := todoapp_graphql
POSTGRES_PORT := 5432
DATABASE_URL := postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@localhost:$(POSTGRES_PORT)/$(POSTGRES_DB)
# for Figment config
APP_DATABASE__URL := $(DATABASE_URL)
APP_SERVER__IP ?= 127.0.0.1
APP_SERVER__PORT ?= 3000

REDIS_IMAGE := redis:latest
REDIS_CONTAINER_NAME := todoapp_graphql_redis
REDIS_PORT := 6379
REDIS_URL := redis://127.0.0.1:$(REDIS_PORT)
# for Figment config
APP_REDIS__URL := $(REDIS_URL)

APP_JWT__SECRET := L/5mEwflOD6/iqTxs4AwQ6ntsxjWK7xRDCUCiUuaoD0A3XK7CiQh6x6YEsgqFpjT

DEV_PROXY_PORT := 80
DEV_FRONTEND_PORT := 5173
# Leave empty to auto-detect; override explicitly when needed.
HAPROXY_BIN ?=

export POSTGRES_CONTAINER_NAME POSTGRES_USER POSTGRES_PASSWORD POSTGRES_DB POSTGRES_PORT DATABASE_URL APP_DATABASE__URL APP_SERVER__IP APP_SERVER__PORT
export REDIS_IMAGE REDIS_CONTAINER_NAME REDIS_PORT REDIS_URL APP_REDIS__URL
export APP_JWT__SECRET
export DEV_PROXY_PORT DEV_FRONTEND_PORT HAPROXY_BIN

ATLAS_MIGRATIONS_DIR_TEMP := migrations
ATLAS_SCHEMA := packages/db/atlas/schema.sql
ATLAS_DEV_URL ?= docker://postgres/latest

.PHONY: start hurl dal proxy-setup-no-sudo proxy-hosts-clean

start:
	mprocs

hurl:
	./tests/run.sh

proxy-setup-no-sudo:
	./scripts/dev-proxy/setup-no-sudo.sh

proxy-hosts-clean:
	./scripts/dev-proxy/hosts.sh clean

# Data access layer: Atlas diff -> root migrations/, then copy to Refinery db/migrations/ as V{N}__{NAME}.sql and refresh db/migrations/atlas.sum.
# Usage: make dal NAME=note  -> e.g. V3__note.sql  (NAME is also passed to atlas migrate diff)
# Note: GNU Make treats $next__ as variable "next__"; dest must not use $$next__ — use printf below.
dal:
	@test -n "$(NAME)" || (echo 'usage: make dal NAME=short_description' >&2; false)
	@mkdir -p "$(ATLAS_MIGRATIONS_DIR_TEMP)"
	atlas migrate diff "$(NAME)" \
		--dir "file://$(CURDIR)/$(ATLAS_MIGRATIONS_DIR_TEMP)" \
		--to "file://$(CURDIR)/$(ATLAS_SCHEMA)" \
		--dev-url "$(ATLAS_DEV_URL)"
	@newest=$$(ls -t $(ATLAS_MIGRATIONS_DIR_TEMP)/*.sql 2>/dev/null | head -1); \
	test -n "$$newest" || (echo "dal: no .sql file in $(ATLAS_MIGRATIONS_DIR_TEMP)/" >&2; exit 1); \
	max=0; \
	for f in $(CURDIR)/packages/db/migrations/V[0-9]*.sql; do \
		test -f "$$f" || continue; \
		n=$$(basename "$$f" | sed -n 's/^V\([0-9][0-9]*\).*/\1/p'); \
		test -n "$$n" || continue; \
		if test "$$n" -gt "$$max" 2>/dev/null; then max=$$n; fi; \
	done; \
	next=$$((max + 1)); \
	dest=`printf '%s/V%s__%s.sql' "$(CURDIR)/packages/db/migrations" "$$next" "$(NAME)"`; \
	cp "$$newest" "$$dest"; \
	echo "dal: wrote $$dest"; \
	atlas migrate hash --dir "file://$(CURDIR)/packages/db/migrations"
