use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use todoapp_graphql_db_queries::queries::todos as todo_queries;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Debug, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate, Clone)]
pub struct TodoChangeset {
    #[validate(length(min = 1, message = "title cannot be empty"))]
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    /// On create: `None` yields an empty tag list. On update: `None` leaves tags unchanged.
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

fn todo_from_load_all(todo: todo_queries::LoadAllBorrowed<'_>) -> Todo {
    Todo {
        id: todo.id,
        title: todo.title.to_owned(),
        description: todo.description.to_owned(),
        tags: todo.tags.map(|v| v.into()).collect(),
        is_completed: todo.is_completed,
        created_at: todo.created_at,
        updated_at: todo.updated_at,
    }
}

fn todo_from_load(todo: todo_queries::LoadBorrowed<'_>) -> Todo {
    Todo {
        id: todo.id,
        title: todo.title.to_owned(),
        description: todo.description.to_owned(),
        tags: todo.tags.map(|v| v.into()).collect(),
        is_completed: todo.is_completed,
        created_at: todo.created_at,
        updated_at: todo.updated_at,
    }
}

fn todo_from_create(todo: todo_queries::CreateBorrowed<'_>) -> Todo {
    Todo {
        id: todo.id,
        title: todo.title.to_owned(),
        description: todo.description.to_owned(),
        tags: todo.tags.map(|v| v.into()).collect(),
        is_completed: todo.is_completed,
        created_at: todo.created_at,
        updated_at: todo.updated_at,
    }
}

fn todo_from_update(todo: todo_queries::UpdateBorrowed<'_>) -> Todo {
    Todo {
        id: todo.id,
        title: todo.title.to_owned(),
        description: todo.description.to_owned(),
        tags: todo.tags.map(|v| v.into()).collect(),
        is_completed: todo.is_completed,
        created_at: todo.created_at,
        updated_at: todo.updated_at,
    }
}

fn todo_from_update_including_tags(todo: todo_queries::UpdateIncludingTagsBorrowed<'_>) -> Todo {
    Todo {
        id: todo.id,
        title: todo.title.to_owned(),
        description: todo.description.to_owned(),
        tags: todo.tags.map(|v| v.into()).collect(),
        is_completed: todo.is_completed,
        created_at: todo.created_at,
        updated_at: todo.updated_at,
    }
}

fn todo_from_update_tags(todo: todo_queries::UpdateTagsBorrowed<'_>) -> Todo {
    Todo {
        id: todo.id,
        title: todo.title.to_owned(),
        description: todo.description.to_owned(),
        tags: todo.tags.map(|v| v.into()).collect(),
        is_completed: todo.is_completed,
        created_at: todo.created_at,
        updated_at: todo.updated_at,
    }
}

pub async fn load_all(db_pool: &crate::DbPool) -> Result<Vec<Todo>, crate::Error> {
    let client = db_pool.get().await?;
    Ok(todo_queries::load_all()
        .bind(&client)
        .map(todo_from_load_all)
        .all()
        .await?)
}

pub async fn load(id: Uuid, db_pool: &crate::DbPool) -> Result<Todo, crate::Error> {
    let client = db_pool.get().await?;
    match todo_queries::load()
        .bind(&client, &id)
        .map(todo_from_load)
        .opt()
        .await?
    {
        Some(todo) => Ok(todo),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn create(todo: TodoChangeset, db_pool: &crate::DbPool) -> Result<Todo, crate::Error> {
    let client = db_pool.get().await?;
    todo.validate()?;
    let tags = todo.tags.as_ref().cloned().unwrap_or_default();
    Ok(todo_queries::create()
        .bind(
            &client,
            &todo.title,
            &todo.description,
            &todo.is_completed,
            &tags,
        )
        .map(todo_from_create)
        .one()
        .await?)
}

pub async fn update(
    id: Uuid,
    todo: TodoChangeset,
    db_pool: &crate::DbPool,
) -> Result<Todo, crate::Error> {
    let client = db_pool.get().await?;
    todo.validate()?;
    let opt = match &todo.tags {
        None => {
            todo_queries::update()
                .bind(
                    &client,
                    &todo.title,
                    &todo.description,
                    &todo.is_completed,
                    &id,
                )
                .map(todo_from_update)
                .opt()
                .await?
        }
        Some(tags) => {
            todo_queries::update_including_tags()
                .bind(
                    &client,
                    &todo.title,
                    &todo.description,
                    &todo.is_completed,
                    tags,
                    &id,
                )
                .map(todo_from_update_including_tags)
                .opt()
                .await?
        }
    };
    match opt {
        Some(todo) => Ok(todo),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn update_tags(
    id: Uuid,
    tags: Vec<String>,
    db_pool: &crate::DbPool,
) -> Result<Todo, crate::Error> {
    let client = db_pool.get().await?;
    match todo_queries::update_tags()
        .bind(&client, &tags, &id)
        .map(todo_from_update_tags)
        .opt()
        .await?
    {
        Some(todo) => Ok(todo),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn delete(id: Uuid, db_pool: &crate::DbPool) -> Result<(), crate::Error> {
    let client = db_pool.get().await?;
    match todo_queries::delete().bind(&client, &id).opt().await? {
        Some(_) => Ok(()),
        None => Err(crate::Error::NoRecordFound),
    }
}
