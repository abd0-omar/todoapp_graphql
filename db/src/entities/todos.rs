use chrono::{DateTime, Utc};
#[cfg(feature = "test-helpers")]
use fake::Dummy;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Postgres;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Debug, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate, Clone)]
#[cfg_attr(feature = "test-helpers", derive(Serialize, Dummy))]
pub struct TodoChangeset {
    #[validate(length(min = 1, message = "title cannot be empty"))]
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

pub async fn load_all(
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Vec<Todo>, crate::Error> {
    let todos = sqlx::query_as!(
        Todo,
        "SELECT id, title, description as \"description!\", is_completed as \"is_completed!\", created_at as \"created_at!\", updated_at as \"updated_at!\" FROM todos"
    )
    .fetch_all(executor)
    .await?;
    Ok(todos)
}

pub async fn load(
    id: Uuid,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Todo, crate::Error> {
    match sqlx::query_as!(
        Todo,
        r#"SELECT id, title, description as "description!", is_completed as "is_completed!", created_at as "created_at!", updated_at as "updated_at!" FROM todos WHERE id = $1"#,
        id
    )
    .fetch_optional(executor)
    .await
    .map_err(crate::Error::DbError)?
    {
        Some(todo) => Ok(todo),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn create(
    todo: TodoChangeset,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Todo, crate::Error> {
    todo.validate()?;

    let record = sqlx::query!(
        r#"INSERT INTO todos (title, description, is_completed) VALUES ($1, $2, $3) RETURNING id, created_at as "created_at!", updated_at as "updated_at!""#,
        todo.title,
        todo.description,
        todo.is_completed,
    )
    .fetch_one(executor)
    .await
    .map_err(crate::Error::DbError)?;

    Ok(Todo {
        id: record.id,
        title: todo.title,
        description: todo.description,
        is_completed: todo.is_completed,
        created_at: record.created_at,
        updated_at: record.updated_at,
    })
}

pub async fn update(
    id: Uuid,
    todo: TodoChangeset,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Todo, crate::Error> {
    todo.validate()?;

    match sqlx::query!(
        r#"UPDATE todos SET title = $1, description = $2, is_completed = $3, updated_at = CURRENT_TIMESTAMP WHERE id = $4 RETURNING id, created_at as "created_at!", updated_at as "updated_at!""#,
        todo.title,
        todo.description,
        todo.is_completed,
        id
    )
    .fetch_optional(executor)
    .await
    .map_err(crate::Error::DbError)?
    {
        Some(record) => Ok(Todo {
            id: record.id,
            title: todo.title,
            description: todo.description,
            is_completed: todo.is_completed,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn delete(
    id: Uuid,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<(), crate::Error> {
    match sqlx::query!("DELETE FROM todos WHERE id = $1 RETURNING id", id)
        .fetch_optional(executor)
        .await
        .map_err(crate::Error::DbError)?
    {
        Some(_) => Ok(()),
        None => Err(crate::Error::NoRecordFound),
    }
}
