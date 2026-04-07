use crate::entities::todos::Todo;
use fake::{faker::boolean::en::Boolean, faker::lorem::en::Sentence, faker::name::en::Name, Dummy};
use sqlx::postgres::PgPool;
use validator::Validate;

#[derive(Debug, Clone, Dummy, Validate)]
pub struct TodoChangeset {
    #[dummy(faker = "Name()")]
    #[validate(length(min = 1))]
    pub title: String,
    #[dummy(faker = "Sentence(1..10)")]
    pub description: String,
    #[dummy(faker = "Boolean(50)")]
    pub is_completed: bool,
}

pub async fn create(todo: TodoChangeset, db: &PgPool) -> Result<Todo, anyhow::Error> {
    let record = sqlx::query!(
        r#"INSERT INTO todos (title, description, is_completed) VALUES ($1, $2, $3) RETURNING id, created_at as "created_at!", updated_at as "updated_at!""#,
        todo.title,
        todo.description,
        todo.is_completed,
    )
    .fetch_one(db)
    .await?;

    Ok(Todo {
        id: record.id,
        title: todo.title,
        description: todo.description,
        is_completed: todo.is_completed,
        created_at: record.created_at,
        updated_at: record.updated_at,
    })
}
