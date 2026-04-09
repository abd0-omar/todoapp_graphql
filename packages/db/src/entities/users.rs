use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use todoapp_graphql_db_queries::queries::users as user_queries;
use todoapp_graphql_db_queries::tokio_postgres;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

/// Row loaded for login (includes password hash).
#[derive(Debug, Clone)]
pub struct UserCredentials {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate, Clone)]
pub struct UserChangeset {
    #[validate(email(message = "invalid email"))]
    pub email: String,
    #[validate(length(min = 1))]
    pub password_hash: String,
}

fn user_from_create(row: user_queries::CreateBorrowed<'_>) -> User {
    User {
        id: row.id,
        email: row.email.to_owned(),
        created_at: row.created_at,
    }
}

fn creds_from_load(row: user_queries::LoadByEmailBorrowed<'_>) -> UserCredentials {
    UserCredentials {
        id: row.id,
        email: row.email.to_owned(),
        password_hash: row.password_hash.to_owned(),
        created_at: row.created_at,
    }
}

fn user_from_load_by_id(row: user_queries::LoadByIdBorrowed<'_>) -> User {
    User {
        id: row.id,
        email: row.email.to_owned(),
        created_at: row.created_at,
    }
}

fn map_unique_violation(e: tokio_postgres::Error) -> crate::Error {
    if let Some(db) = e.as_db_error() {
        if db.constraint() == Some("users_email_key") {
            return crate::Error::DuplicateEmail;
        }
    }
    crate::Error::DbError(e)
}

pub async fn create(user: UserChangeset, db_pool: &crate::DbPool) -> Result<User, crate::Error> {
    let client = db_pool.get().await?;
    user.validate()?;
    user_queries::create()
        .bind(&client, &user.email, &user.password_hash)
        .map(user_from_create)
        .one()
        .await
        .map_err(map_unique_violation)
}

pub async fn load_by_email(
    email: &str,
    db_pool: &crate::DbPool,
) -> Result<UserCredentials, crate::Error> {
    let client = db_pool.get().await?;
    match user_queries::load_by_email()
        .bind(&client, &email)
        .map(creds_from_load)
        .opt()
        .await?
    {
        Some(u) => Ok(u),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn load_by_id(id: Uuid, db_pool: &crate::DbPool) -> Result<User, crate::Error> {
    let client = db_pool.get().await?;
    match user_queries::load_by_id()
        .bind(&client, &id)
        .map(user_from_load_by_id)
        .opt()
        .await?
    {
        Some(u) => Ok(u),
        None => Err(crate::Error::NoRecordFound),
    }
}
