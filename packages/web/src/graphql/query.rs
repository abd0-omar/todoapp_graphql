use async_graphql::{Context, Object, Result};
use todoapp_graphql_db::{entities::todos, DbPool};
use uuid::Uuid;

use super::types::Todo;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get all todos.
    async fn todos(&self, ctx: &Context<'_>) -> Result<Vec<Todo>> {
        let pool = ctx.data::<DbPool>()?;
        let todos = todos::load_all(pool).await?;
        Ok(todos.into_iter().map(Todo::from).collect())
    }

    /// Get a single todo by ID.
    async fn todo(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Todo>> {
        let pool = ctx.data::<DbPool>()?;
        match todos::load(id, pool).await {
            Ok(todo) => Ok(Some(Todo::from(todo))),
            Err(todoapp_graphql_db::Error::NoRecordFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
