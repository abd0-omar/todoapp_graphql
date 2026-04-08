use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use todoapp_graphql_db::DbPool;
use todoapp_graphql_jwt_port::JwtAuthPort;

use super::mutation::MutationRoot;
use super::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Build the GraphQL schema with pool, JWT service, and access-token TTL for auth mutations.
pub fn build_schema(
    pool: DbPool,
    jwt: Arc<dyn JwtAuthPort>,
    jwt_access_token_ttl_secs: u64,
) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .data(jwt)
        .data(jwt_access_token_ttl_secs)
        .finish()
}
