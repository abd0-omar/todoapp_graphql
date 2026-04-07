use crate::controllers::todo;
use crate::state::AppState;
use axum::routing::{delete, get, post, put};
use axum::Router;

use std::sync::Arc;

/// Initializes the application's routes.
///
/// This function maps paths (e.g. "/greet") and HTTP methods (e.g. "GET") to functions in [`crate::controllers`] as well as includes middlewares defined in [`crate::middlewares`] into the routing layer (see [`axum::Router`]).
pub fn init_routes(app_state: AppState) -> Router {
    let shared_app_state = Arc::new(app_state);
    Router::new()
        .route("/todos", get(todo::read_all))
        .route("/todos", post(todo::create))
        .route("/todos/{id}", get(todo::read_one))
        .route("/todos/{id}", put(todo::update))
        .route("/todos/{id}", delete(todo::delete))
        .with_state(shared_app_state)
}
