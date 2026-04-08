use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use async_graphql::{Context, Error, Object, Result};
use secrecy::{ExposeSecret, SecretString};
use std::sync::Arc;
use todoapp_graphql_db::entities::users::UserChangeset;
use todoapp_graphql_db::entities::{todos, users as user_entity};
use todoapp_graphql_db::DbPool;
use todoapp_graphql_jwt_port::JwtAuthPort;
use tracing::info;
use uuid::Uuid;

use super::auth::require_user;
use super::types::{AuthPayload, GqlUser, LoginInput, SignUpInput, Todo, TodoInput};

pub struct MutationRoot;

fn ensure_password_strength(password: &str) -> Result<()> {
    if password.len() < 8 {
        return Err(Error::new("password must be at least 8 characters"));
    }
    Ok(())
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|_| Error::new("could not hash password"))
}

fn verify_password(password: &str, phc: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(phc) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

#[Object]
impl MutationRoot {
    /// Register a new user and return a JWT access token.
    async fn sign_up(&self, ctx: &Context<'_>, input: SignUpInput) -> Result<AuthPayload> {
        let password: SecretString = input.password.into();
        ensure_password_strength(password.expose_secret())?;
        let password_owned = password.expose_secret().to_string();
        let pool = ctx.data::<DbPool>()?;
        let jwt = ctx.data::<Arc<dyn JwtAuthPort>>()?.clone();
        let ttl = *ctx.data::<u64>()?;
        let hash = tokio::task::spawn_blocking(move || hash_password(&password_owned))
            .await
            .map_err(|e| Error::new(format!("password hashing failed: {e}")))?
            ?;
        let changeset = UserChangeset {
            email: input.email.trim().to_lowercase(),
            password_hash: hash,
        };
        let user = match user_entity::create(changeset, pool).await {
            Ok(u) => u,
            Err(todoapp_graphql_db::Error::DuplicateEmail) => {
                return Err(Error::new("email already registered"));
            }
            Err(e) => return Err(e.into()),
        };
        let access_token = jwt
            .sign_access_token(&user.id.to_string(), ttl)
            .map_err(|e| Error::new(format!("{e}")))?;
        Ok(AuthPayload {
            access_token,
            user: GqlUser {
                id: user.id,
                email: user.email,
            },
        })
    }

    /// Log in with email and password.
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<AuthPayload> {
        let password: SecretString = input.password.into();
        ensure_password_strength(password.expose_secret())?;
        let pool = ctx.data::<DbPool>()?;
        let jwt = ctx.data::<Arc<dyn JwtAuthPort>>()?.clone();
        let ttl = *ctx.data::<u64>()?;
        let email = input.email.trim().to_lowercase();
        let creds = match user_entity::load_by_email(&email, pool).await {
            Ok(c) => c,
            Err(todoapp_graphql_db::Error::NoRecordFound) => {
                return Err(Error::new("invalid email or password"));
            }
            Err(e) => return Err(e.into()),
        };
        let password_owned = password.expose_secret().to_string();
        let password_hash = creds.password_hash.clone();
        let verified = tokio::task::spawn_blocking(move || {
            verify_password(&password_owned, &password_hash)
        })
        .await
        .map_err(|e| Error::new(format!("password verification failed: {e}")))?;
        if !verified {
            return Err(Error::new("invalid email or password"));
        }
        let access_token = jwt
            .sign_access_token(&creds.id.to_string(), ttl)
            .map_err(|e| Error::new(format!("{e}")))?;
        Ok(AuthPayload {
            access_token,
            user: GqlUser {
                id: creds.id,
                email: creds.email,
            },
        })
    }

    /// Create a new todo.
    async fn create_todo(&self, ctx: &Context<'_>, input: TodoInput) -> Result<Todo> {
        require_user(ctx)?;
        let pool = ctx.data::<DbPool>()?;
        let todo = todos::create(input.into(), pool).await?;
        info!("created todo {:?}", todo);
        Ok(Todo::from(todo))
    }

    /// Update an existing todo. Returns None if the todo doesn't exist.
    async fn update_todo(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: TodoInput,
    ) -> Result<Option<Todo>> {
        require_user(ctx)?;
        let pool = ctx.data::<DbPool>()?;
        match todos::update(id, input.into(), pool).await {
            Ok(todo) => {
                info!("updated todo {:?}", todo);
                Ok(Some(Todo::from(todo)))
            }
            Err(todoapp_graphql_db::Error::NoRecordFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Delete a todo. Returns true if the todo was deleted, false if it didn't exist.
    async fn delete_todo(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        require_user(ctx)?;
        let pool = ctx.data::<DbPool>()?;
        match todos::delete(id, pool).await {
            Ok(()) => {
                info!("deleted todo {}", id);
                Ok(true)
            }
            Err(todoapp_graphql_db::Error::NoRecordFound) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    /// Set tags on a todo. Returns None if the todo does not exist.
    async fn set_todo_tags(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        tags: Vec<String>,
    ) -> Result<Option<Todo>> {
        require_user(ctx)?;
        let pool = ctx.data::<DbPool>()?;
        match todos::update_tags(id, tags, pool).await {
            Ok(todo) => {
                info!("updated tags on todo {:?}", todo);
                Ok(Some(Todo::from(todo)))
            }
            Err(todoapp_graphql_db::Error::NoRecordFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
