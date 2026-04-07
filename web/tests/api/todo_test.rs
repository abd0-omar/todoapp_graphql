use axum::{
    body::Body,
    http::{self, Method},
};
use fake::{Fake, Faker};
use googletest::prelude::*;
use hyper::StatusCode;
use serde_json::{json, Value};
use todoapp_graphql_db::entities;
use todoapp_graphql_macros::db_test;
use todoapp_graphql_web::test_helpers::{BodyExt, DbTestContext, RouterExt};

fn graphql_request(query: &str, variables: Option<Value>) -> String {
    let body = match variables {
        Some(vars) => json!({ "query": query, "variables": vars }),
        None => json!({ "query": query }),
    };
    body.to_string()
}

#[db_test]
async fn test_create_invalid(context: &DbTestContext) {
    let query = r#"
        mutation CreateTodo($input: TodoInput!) {
            createTodo(input: $input) {
                id
                title
            }
        }
    "#;
    let variables = json!({
        "input": {
            "title": "",
            "description": "",
            "isCompleted": false
        }
    });

    let response = context
        .app
        .request("/graphql")
        .method(Method::POST)
        .body(Body::from(graphql_request(query, Some(variables))))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let body: Value = response.into_body().into_json().await;
    assert!(body["errors"].is_array());
    assert!(!body["errors"].as_array().unwrap().is_empty());
}

#[db_test]
async fn test_create_success(context: &DbTestContext) {
    let changeset: entities::todos::TodoChangeset = Faker.fake();
    let query = r#"
        mutation CreateTodo($input: TodoInput!) {
            createTodo(input: $input) {
                id
                title
                description
                isCompleted
            }
        }
    "#;
    let variables = json!({
        "input": {
            "title": changeset.title,
            "description": changeset.description,
            "isCompleted": changeset.is_completed
        }
    });

    let response = context
        .app
        .request("/graphql")
        .method(Method::POST)
        .body(Body::from(graphql_request(query, Some(variables))))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let body: Value = response.into_body().into_json().await;
    assert!(body["errors"].is_null());
    assert_that!(
        body["data"]["createTodo"]["title"].as_str().unwrap(),
        eq(&changeset.title)
    );

    let todos = entities::todos::load_all(&context.db_pool).await.unwrap();
    assert_that!(todos, len(eq(1)));
}

#[db_test]
async fn test_read_all(context: &DbTestContext) {
    let changeset: entities::todos::TodoChangeset = Faker.fake();
    entities::todos::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let query = r#"
        query {
            todos {
                id
                title
                description
                isCompleted
            }
        }
    "#;

    let response = context
        .app
        .request("/graphql")
        .method(Method::POST)
        .body(Body::from(graphql_request(query, None)))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let body: Value = response.into_body().into_json().await;
    assert!(body["errors"].is_null());
    let todos = body["data"]["todos"].as_array().unwrap();
    assert_that!(todos, len(eq(1)));
    assert_that!(todos[0]["title"].as_str().unwrap(), eq(&changeset.title));
}

#[db_test]
async fn test_read_one_nonexistent(context: &DbTestContext) {
    let query = r#"
        query GetTodo($id: UUID!) {
            todo(id: $id) {
                id
                title
            }
        }
    "#;
    let variables = json!({
        "id": uuid::Uuid::new_v4().to_string()
    });

    let response = context
        .app
        .request("/graphql")
        .method(Method::POST)
        .body(Body::from(graphql_request(query, Some(variables))))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let body: Value = response.into_body().into_json().await;
    assert!(body["errors"].is_null());
    assert!(body["data"]["todo"].is_null());
}

#[db_test]
async fn test_read_one_success(context: &DbTestContext) {
    let todo_changeset: entities::todos::TodoChangeset = Faker.fake();
    let todo = entities::todos::create(todo_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let query = r#"
        query GetTodo($id: UUID!) {
            todo(id: $id) {
                id
                title
                description
                isCompleted
            }
        }
    "#;
    let variables = json!({
        "id": todo.id.to_string()
    });

    let response = context
        .app
        .request("/graphql")
        .method(Method::POST)
        .body(Body::from(graphql_request(query, Some(variables))))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let body: Value = response.into_body().into_json().await;
    assert!(body["errors"].is_null());
    assert_that!(
        body["data"]["todo"]["title"].as_str().unwrap(),
        eq(&todo_changeset.title)
    );
}

#[db_test]
async fn test_update_invalid(context: &DbTestContext) {
    let todo_changeset: entities::todos::TodoChangeset = Faker.fake();
    let todo = entities::todos::create(todo_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let query = r#"
        mutation UpdateTodo($id: UUID!, $input: TodoInput!) {
            updateTodo(id: $id, input: $input) {
                id
                title
            }
        }
    "#;
    let variables = json!({
        "id": todo.id.to_string(),
        "input": {
            "title": "",
            "description": "",
            "isCompleted": false
        }
    });

    let response = context
        .app
        .request("/graphql")
        .method(Method::POST)
        .body(Body::from(graphql_request(query, Some(variables))))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let body: Value = response.into_body().into_json().await;
    assert!(body["errors"].is_array());

    let todo_after = entities::todos::load(todo.id, &context.db_pool)
        .await
        .unwrap();
    assert_that!(todo_after.title, eq(&todo.title));
}

#[db_test]
async fn test_update_nonexistent(context: &DbTestContext) {
    let query = r#"
        mutation UpdateTodo($id: UUID!, $input: TodoInput!) {
            updateTodo(id: $id, input: $input) {
                id
                title
            }
        }
    "#;
    let variables = json!({
        "id": uuid::Uuid::new_v4().to_string(),
        "input": {
            "title": "Some title",
            "description": "Some description",
            "isCompleted": false
        }
    });

    let response = context
        .app
        .request("/graphql")
        .method(Method::POST)
        .body(Body::from(graphql_request(query, Some(variables))))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let body: Value = response.into_body().into_json().await;
    assert!(body["errors"].is_null());
    assert!(body["data"]["updateTodo"].is_null());
}

#[db_test]
async fn test_update_success(context: &DbTestContext) {
    let todo_changeset: entities::todos::TodoChangeset = Faker.fake();
    let todo = entities::todos::create(todo_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let new_changeset: entities::todos::TodoChangeset = Faker.fake();
    let query = r#"
        mutation UpdateTodo($id: UUID!, $input: TodoInput!) {
            updateTodo(id: $id, input: $input) {
                id
                title
                description
                isCompleted
            }
        }
    "#;
    let variables = json!({
        "id": todo.id.to_string(),
        "input": {
            "title": new_changeset.title,
            "description": new_changeset.description,
            "isCompleted": new_changeset.is_completed
        }
    });

    let response = context
        .app
        .request("/graphql")
        .method(Method::POST)
        .body(Body::from(graphql_request(query, Some(variables))))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let body: Value = response.into_body().into_json().await;
    assert!(body["errors"].is_null());
    assert_that!(
        body["data"]["updateTodo"]["title"].as_str().unwrap(),
        eq(&new_changeset.title)
    );

    let todo_after = entities::todos::load(todo.id, &context.db_pool)
        .await
        .unwrap();
    assert_that!(todo_after.title, eq(&new_changeset.title));
}

#[db_test]
async fn test_delete_nonexistent(context: &DbTestContext) {
    let query = r#"
        mutation DeleteTodo($id: UUID!) {
            deleteTodo(id: $id)
        }
    "#;
    let variables = json!({
        "id": uuid::Uuid::new_v4().to_string()
    });

    let response = context
        .app
        .request("/graphql")
        .method(Method::POST)
        .body(Body::from(graphql_request(query, Some(variables))))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let body: Value = response.into_body().into_json().await;
    assert!(body["errors"].is_null());
    assert_that!(body["data"]["deleteTodo"].as_bool().unwrap(), eq(false));
}

#[db_test]
async fn test_delete_success(context: &DbTestContext) {
    let todo_changeset: entities::todos::TodoChangeset = Faker.fake();
    let todo = entities::todos::create(todo_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let query = r#"
        mutation DeleteTodo($id: UUID!) {
            deleteTodo(id: $id)
        }
    "#;
    let variables = json!({
        "id": todo.id.to_string()
    });

    let response = context
        .app
        .request("/graphql")
        .method(Method::POST)
        .body(Body::from(graphql_request(query, Some(variables))))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let body: Value = response.into_body().into_json().await;
    assert!(body["errors"].is_null());
    assert_that!(body["data"]["deleteTodo"].as_bool().unwrap(), eq(true));

    let result = entities::todos::load(todo.id, &context.db_pool).await;
    assert_that!(result, err(anything()));
}
