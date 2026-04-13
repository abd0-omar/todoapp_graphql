//! The todoapp_graphql-db crate contains all code related to database access: entities, migrations, functions for validating and reading and writing data.

use refinery::config::Config as RefineryConfig;
use rootcause::prelude::*;
use std::fmt;
use todoapp_graphql_config::DatabaseConfig;
use todoapp_graphql_db_queries::deadpool_postgres::{
    Config as PoolConfig, CreatePoolError, Pool, PoolError, Runtime,
};
use todoapp_graphql_db_queries::tokio_postgres::{self, NoTls};

mod embedded {
    use refinery::embed_migrations;

    embed_migrations!("migrations");
}

pub type DbClient = todoapp_graphql_db_queries::deadpool_postgres::Client;
pub type DbTransaction<'a> = todoapp_graphql_db_queries::deadpool_postgres::Transaction<'a>;

/// Application database pool plus the originating connection URL.
#[derive(Clone)]
pub struct DbPool {
    pool: Pool,
    database_url: String,
}

impl DbPool {
    pub async fn get(&self) -> std::result::Result<DbClient, PoolError> {
        self.pool.get().await
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}

/// Entity definitions and related functions
pub mod entities;

/// Starts a new database transaction.
///
/// Example:
/// ```
/// let tx = transaction(&app_state.db_pool).await?;
/// tasks::create(task_data, &mut *tx)?;
/// users::create(user_data, &mut *tx)?;
///
/// match tx.commit().await {
///     Ok(_) => Ok((StatusCode::CREATED, Json(results))),
///     Err(e) => Err((internal_error(e), "".into())),
/// }
/// ```
pub async fn client(db_pool: &DbPool) -> Result<DbClient, Report<Error>> {
    db_pool
        .get()
        .await
        .map_err(|e| report!(Error::PoolError(e)).attach("Failed to get database client from pool"))
}

/// Errors that can occur as a result of a data layer operation.
#[derive(Debug)]
pub enum Error {
    /// General database error, e.g. communicating with the database failed
    DbError(tokio_postgres::Error),
    /// database connection failed
    PoolError(PoolError),
    /// Database URL could not be turned into a refinery config (migrations).
    RefineryConfig(refinery::Error),
    /// Embedded SQL migrations failed to apply.
    Migration(refinery::Error),
    /// Deadpool could not build the connection pool from configuration.
    PoolCreate(CreatePoolError),
    /// No record was found, e.g. when loading a record by ID. This variant is different from
    /// `Error::DbError(tokio_postgres::Error)` in that the latter indicates a bug, and
    /// `Error::NoRecordFound` does not. It merely represents an expected "not found" result.
    NoRecordFound,
    DuplicateEmail,
    /// An invalid changeset was passed to a writing operation such as creating or updating a record.
    ValidationError(validator::ValidationErrors),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DbError(_) => write!(f, "database query failed"),
            Error::PoolError(_) => write!(f, "database connection failed"),
            Error::RefineryConfig(_) => write!(f, "invalid database URL for migrations"),
            Error::Migration(_) => write!(f, "database migration failed"),
            Error::PoolCreate(_) => write!(f, "failed to create database pool"),
            Error::NoRecordFound => write!(f, "no record found"),
            Error::DuplicateEmail => write!(f, "email already registered"),
            Error::ValidationError(_) => write!(f, "validation failed"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DbError(e) => Some(e),
            Error::PoolError(e) => Some(e),
            Error::RefineryConfig(e) | Error::Migration(e) => Some(e),
            Error::PoolCreate(e) => Some(e),
            Error::ValidationError(e) => Some(e),
            Error::NoRecordFound | Error::DuplicateEmail => None,
        }
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(value: tokio_postgres::Error) -> Self {
        Error::DbError(value)
    }
}

impl From<PoolError> for Error {
    fn from(value: PoolError) -> Self {
        Error::PoolError(value)
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(value: validator::ValidationErrors) -> Self {
        Error::ValidationError(value)
    }
}

async fn migrate_database(config: &DatabaseConfig) -> Result<(), Report<Error>> {
    let mut refinery_config: RefineryConfig = config.url.parse().map_err(|e| {
        report!(Error::RefineryConfig(e))
            .attach("Failed to build refinery config from database URL")
    })?;

    embedded::migrations::runner()
        .run_async(&mut refinery_config)
        .await
        .map_err(|e| report!(Error::Migration(e)).attach("Failed to run startup migrations"))?;

    Ok(())
}

/// Creates a connection pool to the database specified in the passed [`todoapp_graphql-config::DatabaseConfig`]
pub async fn connect_pool(config: DatabaseConfig) -> Result<DbPool, Report<Error>> {
    migrate_database(&config).await?;

    let mut pool_config = PoolConfig::new();
    pool_config.url = Some(config.url.clone());

    let pool = pool_config
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|e| report!(Error::PoolCreate(e)).attach("Failed to create database pool"))?;

    let _ = pool
        .get()
        .await
        .map_err(|e| report!(Error::PoolError(e)).attach("Failed to connect to database"))?;

    Ok(DbPool {
        pool,
        database_url: config.url,
    })
}
