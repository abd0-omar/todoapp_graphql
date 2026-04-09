use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::{Extension, State};
use axum::middleware::from_fn_with_state;
use axum::response::Html;
use axum::routing::{get, post};
use axum::Router;

use crate::middlewares::{jwt_auth_middleware, AuthContext};
use crate::state::SharedAppState;

async fn graphiql_handler() -> Html<String> {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .title("Todo GraphQL")
            .finish(),
    )
}

async fn graphql_handler(
    State(state): State<SharedAppState>,
    Extension(auth): Extension<AuthContext>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner().data(auth).data(state.db_pool.clone());

    state.graphql_schema.execute(req).await.into()
}

/// Initializes the application's routes.
///
/// This function sets up GraphiQL at `/graphiql` and the GraphQL endpoint at `/graphql`.
pub fn init_routes(state: SharedAppState) -> Router {
    Router::new()
        .route("/graphiql", get(graphiql_handler))
        .route("/graphql", post(graphql_handler))
        .layer(from_fn_with_state(state.clone(), jwt_auth_middleware))
        .with_state(state)
}
