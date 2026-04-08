use async_graphql::{Context, Object, Result};
use todoapp_graphql_db::{entities::todos, DbPool};
use tracing::info;
use uuid::Uuid;

use super::auth::require_user;
use super::types::{Todo, TodoInput};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
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
