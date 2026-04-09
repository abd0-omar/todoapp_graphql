use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use todoapp_graphql_db::DbPool;
use todoapp_graphql_jwt_port::JwtAuthPort;
use todoapp_graphql_refresh_token_port::RefreshTokenStore;

use super::mutation::MutationRoot;
use super::query::QueryRoot;
use super::types::AuthTtls;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Build the GraphQL schema with pool, JWT, token TTLs, and Redis-backed refresh store.
pub fn build_schema(
    pool: DbPool,
    jwt: Arc<dyn JwtAuthPort>,
    auth_ttls: AuthTtls,
    refresh_tokens: Arc<dyn RefreshTokenStore>,
) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .data(jwt)
        .data(auth_ttls)
        .data(refresh_tokens)
        .finish()
}
