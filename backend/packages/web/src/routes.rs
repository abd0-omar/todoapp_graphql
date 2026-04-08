use crate::state::AppState;
use async_graphql_axum::GraphQL;
use axum::routing::post_service;
use axum::Router;

/// Initializes the application's routes.
///
/// This function sets up the GraphQL endpoint at `/graphql`.
pub fn init_routes(app_state: AppState) -> Router {
    Router::new().route(
        "/graphql",
        post_service(GraphQL::new(app_state.graphql_schema.clone())),
    )
}
