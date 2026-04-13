# Capture auth token for this file
POST {{base_url}}/graphql
Content-Type: application/json
{
  "query": "mutation SignUp($input: SignUpInput!) { signUp(input: $input) { accessToken } }",
  "variables": {
    "input": {
      "email": "{{newUuid}}@example.com",
      "password": "TestPassword123!@#"
    }
  }
}

HTTP 200
[Asserts]
jsonpath "$.data.signUp.accessToken" exists
[Captures]
auth_token: jsonpath "$.data.signUp.accessToken"

# Create a todo via GraphQL mutation
POST {{base_url}}/graphql
Content-Type: application/json
Authorization: Bearer {{auth_token}}
{
  "query": "mutation CreateTodo($input: TodoInput!) { createTodo(input: $input) { id title description isCompleted createdAt updatedAt } }",
  "variables": {
    "input": {
      "title": "{{todo_title}}",
      "description": "{{todo_description}}",
      "isCompleted": {{todo_is_completed}}
    }
  }
}

HTTP 200
[Asserts]
header "Content-Type" contains "application/graphql-response+json"
jsonpath "$.data.createTodo.id" exists
jsonpath "$.data.createTodo.title" == "{{todo_title}}"
jsonpath "$.data.createTodo.description" == "{{todo_description}}"
jsonpath "$.data.createTodo.isCompleted" == {{todo_is_completed}}
jsonpath "$.data.createTodo.createdAt" exists
jsonpath "$.data.createTodo.updatedAt" exists

[Captures]
todo_id: jsonpath "$.data.createTodo.id"

# Create another todo for list tests
POST {{base_url}}/graphql
Content-Type: application/json
Authorization: Bearer {{auth_token}}
{
  "query": "mutation CreateTodo($input: TodoInput!) { createTodo(input: $input) { id title description isCompleted } }",
  "variables": {
    "input": {
      "title": "Second GraphQL todo",
      "description": "Second description",
      "isCompleted": false
    }
  }
}

HTTP 200
[Asserts]
jsonpath "$.data.createTodo.id" exists
jsonpath "$.data.createTodo.title" == "Second GraphQL todo"
