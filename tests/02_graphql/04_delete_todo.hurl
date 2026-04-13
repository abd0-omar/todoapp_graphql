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

# First create a todo to delete
POST {{base_url}}/graphql
Content-Type: application/json
Authorization: Bearer {{auth_token}}
{
  "query": "mutation CreateTodo($input: TodoInput!) { createTodo(input: $input) { id } }",
  "variables": {
    "input": {
      "title": "{{todo_title}}",
      "description": "{{todo_description}}",
      "isCompleted": {{todo_is_completed}}
    }
  }
}

HTTP 200
[Captures]
todo_id: jsonpath "$.data.createTodo.id"

# Verify todo exists
POST {{base_url}}/graphql
Content-Type: application/json
Authorization: Bearer {{auth_token}}
{
  "query": "query GetTodo($id: UUID!) { todo(id: $id) { id } }",
  "variables": {
    "id": "{{todo_id}}"
  }
}

HTTP 200
[Asserts]
jsonpath "$.data.todo.id" == "{{todo_id}}"

# Delete the todo
POST {{base_url}}/graphql
Content-Type: application/json
Authorization: Bearer {{auth_token}}
{
  "query": "mutation DeleteTodo($id: UUID!) { deleteTodo(id: $id) }",
  "variables": {
    "id": "{{todo_id}}"
  }
}

HTTP 200
[Asserts]
jsonpath "$.data.deleteTodo" == true

# Verify todo is deleted
POST {{base_url}}/graphql
Content-Type: application/json
Authorization: Bearer {{auth_token}}
{
  "query": "query GetTodo($id: UUID!) { todo(id: $id) { id } }",
  "variables": {
    "id": "{{todo_id}}"
  }
}

HTTP 200
[Asserts]
jsonpath "$.data.todo" == null

# Delete a nonexistent todo (returns false)
POST {{base_url}}/graphql
Content-Type: application/json
Authorization: Bearer {{auth_token}}
{
  "query": "mutation DeleteTodo($id: UUID!) { deleteTodo(id: $id) }",
  "variables": {
    "id": "00000000-0000-0000-0000-000000000000"
  }
}

HTTP 200
[Asserts]
jsonpath "$.data.deleteTodo" == false
