use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use todoapp_graphql_jwt_port::VerifiedToken;

use crate::state::SharedAppState;

const BEARER: &str = "Bearer ";

/// Per-request auth for GraphQL (set by [`jwt_auth_middleware`]).
#[derive(Clone, Debug)]
pub enum AuthContext {
    Anonymous,
    Authenticated(VerifiedToken),
}

pub async fn jwt_auth_middleware(
    State(state): State<SharedAppState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let ctx = match req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
    {
        None => AuthContext::Anonymous,
        Some(value) => {
            if !value.starts_with(BEARER) {
                return (StatusCode::UNAUTHORIZED, "invalid Authorization scheme").into_response();
            }
            let token = value[BEARER.len()..].trim();
            if token.is_empty() {
                return (StatusCode::UNAUTHORIZED, "missing bearer token").into_response();
            }
            match state.jwt.verify_access_token(token) {
                Ok(v) => AuthContext::Authenticated(v),
                Err(_) => {
                    return (StatusCode::UNAUTHORIZED, "invalid or expired token").into_response();
                }
            }
        }
    };

    req.extensions_mut().insert(ctx);
    next.run(req).await
}
