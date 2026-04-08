# Task 1 (Todo app) 

## Description

Create a simple todo app using the following technologies:

- Rust
- Axum
- PostgreSQL
- GraphQL

## Requirements

- The app should be able to create, read, update and delete todos.
- Postgres database should be running in a docker container.

# Task 2 (type-checked database queries)

## Description

Create database CRUD queries for the todos table using: [clorinde](https://github.com/halcyonnouveau/clorinde)
and [deadpool](https://github.com/deadpool-rs/deadpool) for postgres database connection pooling.

## Requirements

- The queries should be type-checked at compile time.

# Task 3 (database migrations)

## Description

Create embedded database migrations using: [refinery](https://github.com/rust-db/refinery)

## Requirements

- Migrations should be applied and validated at runtime.

# Task 4 (Schema and migrations generation)

## Description

Generate migrations from the schema using: [atlas](https://github.com/arigaio/atlas)

## Requirements

- Add a new `tags` field to the todos table schema.
- Use `atlas` to generate sql migrations from the schema.
- Add new database query for the `tags` field.
- Expose `tags` field in the GraphQL schema.

# Task 5 (Run server and postgres database container in one command)

## Description

Use [mprocs](https://github.com/pvolok/mprocs).

## Requirements

- the server should be run in cargo watch mode.
- mprocs should be able to run/start/attach the docker container.
    - run, if the container doesn't exist yet.
    - start, if it exists but is stopped.
    - attach, if it's already running.
- the server and postgres database container should be started in one command.