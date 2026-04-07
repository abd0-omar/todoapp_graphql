use axum::{
    body::Body,
    http::{self, Method},
};
use fake::{Fake, Faker};
use googletest::prelude::*;
use hyper::StatusCode;
use serde_json::json;
use todoapp_graphql_db::entities;
use todoapp_graphql_macros::db_test;
use todoapp_graphql_web::test_helpers::{BodyExt, DbTestContext, RouterExt};
use uuid::Uuid;

#[db_test]
async fn test_create_invalid(context: &DbTestContext) {
    let payload = json!(entities::todos::TodoChangeset {
        title: String::from(""),
        description: String::from(""),
        is_completed: false,
    });

    let response = context
        .app
        .request("/todos")
        .method(Method::POST)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::UNPROCESSABLE_ENTITY));
}

#[db_test]
async fn test_create_success(context: &DbTestContext) {
    let changeset: entities::todos::TodoChangeset = Faker.fake();
    let payload = json!(changeset);

    let response = context
        .app
        .request("/todos")
        .method(Method::POST)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::CREATED));

    let todos = entities::todos::load_all(&context.db_pool).await.unwrap();
    assert_that!(todos, len(eq(1)));
    assert_that!(todos.first().unwrap().title, eq(&changeset.title));
}

#[db_test]
async fn test_read_all(context: &DbTestContext) {
    let changeset: entities::todos::TodoChangeset = Faker.fake();
    entities::todos::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let response = context.app.request("/todos").send().await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let todos: Vec<entities::todos::Todo> = response
        .into_body()
        .into_json::<Vec<entities::todos::Todo>>()
        .await;
    assert_that!(todos, len(eq(1)));
    assert_that!(todos.first().unwrap().title, eq(&changeset.title));
}

#[db_test]
async fn test_read_one_nonexistent(context: &DbTestContext) {
    let response = context
        .app
        .request(&format!("/todos/{}", Uuid::new_v4()))
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::NOT_FOUND));
}

#[db_test]
async fn test_read_one_success(context: &DbTestContext) {
    let todo_changeset: entities::todos::TodoChangeset = Faker.fake();
    let todo = entities::todos::create(todo_changeset.clone(), &context.db_pool)
        .await
        .unwrap();
    let todo_id = todo.id;

    let response = context
        .app
        .request(&format!("/todos/{}", todo_id))
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let todo: entities::todos::Todo = response
        .into_body()
        .into_json::<entities::todos::Todo>()
        .await;
    assert_that!(todo.id, eq(todo_id));
    assert_that!(todo.title, eq(&todo_changeset.title));
}

#[db_test]
async fn test_update_invalid(context: &DbTestContext) {
    let todo_changeset: entities::todos::TodoChangeset = Faker.fake();
    let todo = entities::todos::create(todo_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let payload = json!(entities::todos::TodoChangeset {
        title: String::from(""),
        description: String::from(""),
        is_completed: false,
    });

    let response = context
        .app
        .request(&format!("/todos/{}", todo.id))
        .method(Method::PUT)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::UNPROCESSABLE_ENTITY));

    let todo_after = entities::todos::load(todo.id, &context.db_pool)
        .await
        .unwrap();
    assert_that!(todo_after.title, eq(&todo.title));
}

#[db_test]
async fn test_update_nonexistent(context: &DbTestContext) {
    let todo_changeset: entities::todos::TodoChangeset = Faker.fake();
    let payload = json!(todo_changeset);

    let response = context
        .app
        .request(&format!("/todos/{}", Uuid::new_v4()))
        .method(Method::PUT)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::NOT_FOUND));
}

#[db_test]
async fn test_update_success(context: &DbTestContext) {
    let todo_changeset: entities::todos::TodoChangeset = Faker.fake();
    let todo = entities::todos::create(todo_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let todo_changeset: entities::todos::TodoChangeset = Faker.fake();
    let payload = json!(todo_changeset);

    let response = context
        .app
        .request(&format!("/todos/{}", todo.id))
        .method(Method::PUT)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let todo: entities::todos::Todo = response
        .into_body()
        .into_json::<entities::todos::Todo>()
        .await;
    assert_that!(todo.title, eq(&todo_changeset.title.clone()));

    let todo = entities::todos::load(todo.id, &context.db_pool)
        .await
        .unwrap();
    assert_that!(todo.title, eq(&todo_changeset.title));
}

#[db_test]
async fn test_delete_nonexistent(context: &DbTestContext) {
    let response = context
        .app
        .request(&format!("/todos/{}", Uuid::new_v4()))
        .method(Method::DELETE)
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::NOT_FOUND));
}

#[db_test]
async fn test_delete_success(context: &DbTestContext) {
    let todo_changeset: entities::todos::TodoChangeset = Faker.fake();
    let todo = entities::todos::create(todo_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let response = context
        .app
        .request(&format!("/todos/{}", todo.id))
        .method(Method::DELETE)
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::NO_CONTENT));

    let result = entities::todos::load(todo.id, &context.db_pool).await;
    assert_that!(result, err(anything()));
}
