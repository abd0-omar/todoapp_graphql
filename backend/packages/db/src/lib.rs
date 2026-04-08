//! The todoapp_graphql-db crate contains all code related to database access: entities, migrations, functions for validating and reading and writing data.

use anyhow::{Context, Result};
use refinery::config::Config as RefineryConfig;
use thiserror::Error;
use todoapp_graphql_config::DatabaseConfig;
use todoapp_graphql_db_queries::deadpool_postgres::{
    Config as PoolConfig, Pool, PoolError, Runtime,
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
    pub async fn get(&self) -> Result<DbClient, PoolError> {
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
pub async fn client(db_pool: &DbPool) -> Result<DbClient, anyhow::Error> {
    db_pool
        .get()
        .await
        .context("Failed to get database client from pool")
}

/// Errors that can occur as a result of a data layer operation.
#[derive(Error, Debug)]
pub enum Error {
    /// General database error, e.g. communicating with the database failed
    #[error("database query failed")]
    DbError(#[from] tokio_postgres::Error),
    #[error("database connection failed")]
    PoolError(#[from] PoolError),
    /// No record was found, e.g. when loading a record by ID. This variant is different from
    /// `Error::DbError(tokio_postgres::Error)` in that the latter indicates a bug, and
    /// `Error::NoRecordFound` does not. It merely represents an expected "not found" result.
    #[error("no record found")]
    NoRecordFound,
    #[error("validation failed")]
    /// An invalid changeset was passed to a writing operation such as creating or updating a record.
    ValidationError(#[from] validator::ValidationErrors),
}

async fn migrate_database(config: &DatabaseConfig) -> Result<()> {
    let mut refinery_config: RefineryConfig = config
        .url
        .parse()
        .context("Failed to build refinery config from database URL")?;

    embedded::migrations::runner()
        .run_async(&mut refinery_config)
        .await
        .context("Failed to run startup migrations")?;

    Ok(())
}

/// Creates a connection pool to the database specified in the passed [`todoapp_graphql-config::DatabaseConfig`]
pub async fn connect_pool(config: DatabaseConfig) -> Result<DbPool, anyhow::Error> {
    migrate_database(&config).await?;

    let mut pool_config = PoolConfig::new();
    pool_config.url = Some(config.url.clone());

    let pool = pool_config
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .context("Failed to create database pool")?;

    let _ = pool.get().await.context("Failed to connect to database")?;

    Ok(DbPool {
        pool,
        database_url: config.url,
    })
}
