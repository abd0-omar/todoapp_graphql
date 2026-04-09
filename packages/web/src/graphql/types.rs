use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// GraphQL representation of a Todo item.
#[derive(SimpleObject)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<todoapp_graphql_db::entities::todos::Todo> for Todo {
    fn from(todo: todoapp_graphql_db::entities::todos::Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            description: todo.description,
            tags: todo.tags,
            is_completed: todo.is_completed,
            created_at: todo.created_at,
            updated_at: todo.updated_at,
        }
    }
}

/// Input type for creating or updating a Todo.
#[derive(InputObject)]
pub struct TodoInput {
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub tags: Option<Vec<String>>,
}

impl From<TodoInput> for todoapp_graphql_db::entities::todos::TodoChangeset {
    fn from(input: TodoInput) -> Self {
        Self {
            title: input.title,
            description: input.description,
            is_completed: input.is_completed,
            tags: input.tags,
        }
    }
}

/// Register a new account.
#[derive(InputObject)]
pub struct SignUpInput {
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

/// Public user fields returned after sign-up or login.
#[derive(SimpleObject, Clone)]
pub struct GqlUser {
    pub id: Uuid,
    pub email: String,
}

/// Access and refresh token TTLs in seconds (GraphQL context).
#[derive(Clone, Copy)]
pub struct AuthTtls {
    pub access_token_secs: u64,
    pub refresh_token_secs: u64,
}

/// JWT and user returned from auth mutations.
#[derive(SimpleObject)]
pub struct AuthPayload {
    #[graphql(name = "accessToken")]
    pub access_token: String,
    #[graphql(name = "refreshToken")]
    pub refresh_token: String,
    pub user: GqlUser,
}

/// Refresh an access token using an opaque refresh token from Redis.
#[derive(InputObject)]
pub struct RefreshTokenInput {
    #[graphql(name = "refreshToken")]
    pub refresh_token: String,
}

/// Revoke a refresh token in Redis (logout).
#[derive(InputObject)]
pub struct LogoutInput {
    #[graphql(name = "refreshToken")]
    pub refresh_token: String,
}
