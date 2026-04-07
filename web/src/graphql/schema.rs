use async_graphql::{EmptySubscription, Schema};
use todoapp_graphql_db::DbPool;

use super::mutation::MutationRoot;
use super::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Build the GraphQL schema with the database pool in context.
pub fn build_schema(pool: DbPool) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish()
}
