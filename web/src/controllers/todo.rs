use crate::{error::Error, state::SharedAppState};
use axum::{extract::Path, extract::State, http::StatusCode, Json};
use todoapp_graphql_db::entities;
use tracing::info;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn create(
    State(app_state): State<SharedAppState>,
    Json(todo): Json<entities::todos::TodoChangeset>,
) -> Result<(StatusCode, Json<entities::todos::Todo>), Error> {
    let todo = entities::todos::create(todo, &app_state.db_pool).await?;
    info!("created todo {:?}", todo);
    Ok((StatusCode::CREATED, Json(todo)))
}

#[axum::debug_handler]
pub async fn read_all(
    State(app_state): State<SharedAppState>,
) -> Result<Json<Vec<entities::todos::Todo>>, Error> {
    let todos = entities::todos::load_all(&app_state.db_pool).await?;
    info!("responding with {:?}", todos);
    Ok(Json(todos))
}

#[axum::debug_handler]
pub async fn read_one(
    State(app_state): State<SharedAppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<entities::todos::Todo>, Error> {
    let todo = entities::todos::load(id, &app_state.db_pool).await?;
    info!("responding with {:?}", todo);
    Ok(Json(todo))
}

#[axum::debug_handler]
pub async fn update(
    State(app_state): State<SharedAppState>,
    Path(id): Path<Uuid>,
    Json(todo): Json<entities::todos::TodoChangeset>,
) -> Result<Json<entities::todos::Todo>, Error> {
    let todo = entities::todos::update(id, todo, &app_state.db_pool).await?;
    info!("updated todo {:?}", todo);
    Ok(Json(todo))
}

#[axum::debug_handler]
pub async fn delete(
    State(app_state): State<SharedAppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, Error> {
    entities::todos::delete(id, &app_state.db_pool).await?;
    info!("deleted todo {}", id);
    Ok(StatusCode::NO_CONTENT)
}
