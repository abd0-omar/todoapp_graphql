use std::fmt::{self, Debug, Display};

use rootcause::Report;

/// Error type for the web layer, primarily used for logging.
/// GraphQL errors are handled through async-graphql's error system.
#[derive(Debug)]
pub enum Error {
    /// Errors that can occur as a result of a data layer operation.
    Database(todoapp_graphql_db::Error),
    /// Any other error.
    Other(Report),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Database(e) => write!(f, "Database error: {e}"),
            Error::Other(e) => write!(f, "Error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Database(e) => Some(e),
            Error::Other(_) => None,
        }
    }
}

impl From<todoapp_graphql_db::Error> for Error {
    fn from(value: todoapp_graphql_db::Error) -> Self {
        Error::Database(value)
    }
}

impl From<Report> for Error {
    fn from(value: Report) -> Self {
        Error::Other(value)
    }
}

/// Helper function to log internal errors.
pub fn log_internal_error<E>(e: E)
where
    E: Debug + Display,
{
    tracing::error!(err.msg = %e, err.details = ?e, "Internal server error");
}
