# First create a todo to read
POST {{base_url}}/graphql
Content-Type: application/json
{
  "query": "mutation CreateTodo($input: TodoInput!) { createTodo(input: $input) { id title description isCompleted } }",
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

# Get all todos
POST {{base_url}}/graphql
Content-Type: application/json
{
  "query": "query { todos { id title description isCompleted createdAt updatedAt } }"
}

HTTP 200
[Asserts]
header "Content-Type" contains "application/graphql-response+json"
jsonpath "$.data.todos" count >= 1

# Get the specific todo by ID
POST {{base_url}}/graphql
Content-Type: application/json
{
  "query": "query GetTodo($id: UUID!) { todo(id: $id) { id title description isCompleted } }",
  "variables": {
    "id": "{{todo_id}}"
  }
}

HTTP 200
[Asserts]
header "Content-Type" contains "application/graphql-response+json"
jsonpath "$.data.todo.id" == "{{todo_id}}"
jsonpath "$.data.todo.title" == "{{todo_title}}"
jsonpath "$.data.todo.description" == "{{todo_description}}"
jsonpath "$.data.todo.isCompleted" == {{todo_is_completed}}

# Get a nonexistent todo (should return null, not error)
POST {{base_url}}/graphql
Content-Type: application/json
{
  "query": "query GetTodo($id: UUID!) { todo(id: $id) { id title } }",
  "variables": {
    "id": "00000000-0000-0000-0000-000000000000"
  }
}

HTTP 200
[Asserts]
jsonpath "$.data.todo" == null
