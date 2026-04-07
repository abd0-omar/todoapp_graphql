use crate::graphql::build_schema;
use crate::state::AppState;
use async_graphql_axum::GraphQL;
use axum::routing::post_service;
use axum::Router;

/// Initializes the application's routes.
///
/// This function sets up the GraphQL endpoint at `/graphql`.
pub fn init_routes(app_state: AppState) -> Router {
    let schema = build_schema(app_state.db_pool);
    Router::new().route("/graphql", post_service(GraphQL::new(schema)))
}
