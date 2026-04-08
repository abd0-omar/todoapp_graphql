use async_graphql::{Context, Error, Result};
use todoapp_graphql_jwt_port::VerifiedToken;

use crate::middlewares::AuthContext;

/// Returns the authenticated subject or a GraphQL error.
pub fn require_user(ctx: &Context<'_>) -> Result<VerifiedToken> {
    match ctx.data::<AuthContext>() {
        Ok(AuthContext::Authenticated(v)) => Ok(v.clone()),
        Ok(AuthContext::Anonymous) | Err(_) => Err(Error::new("authentication required")),
    }
}
