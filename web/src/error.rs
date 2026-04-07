use std::fmt::{Debug, Display};

/// Error type for the web layer, primarily used for logging.
/// GraphQL errors are handled through async-graphql's error system.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Errors that can occur as a result of a data layer operation.
    #[error("Database error")]
    Database(#[from] todoapp_graphql_db::Error),
    /// Any other error.
    #[error("Error: {0}")]
    Other(#[from] anyhow::Error),
}

/// Helper function to log internal errors.
pub fn log_internal_error<E>(e: E)
where
    E: Debug + Display,
{
    tracing::error!(err.msg = %e, err.details = ?e, "Internal server error");
}
