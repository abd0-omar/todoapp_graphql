# First create a todo to update
POST {{base_url}}/graphql
Content-Type: application/json
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

# Update the todo
POST {{base_url}}/graphql
Content-Type: application/json
{
  "query": "mutation UpdateTodo($id: UUID!, $input: TodoInput!) { updateTodo(id: $id, input: $input) { id title description isCompleted } }",
  "variables": {
    "id": "{{todo_id}}",
    "input": {
      "title": "{{updated_todo_title}}",
      "description": "{{updated_todo_description}}",
      "isCompleted": {{updated_todo_is_completed}}
    }
  }
}

HTTP 200
[Asserts]
header "Content-Type" contains "application/graphql-response+json"
jsonpath "$.data.updateTodo.id" == "{{todo_id}}"
jsonpath "$.data.updateTodo.title" == "{{updated_todo_title}}"
jsonpath "$.data.updateTodo.description" == "{{updated_todo_description}}"
jsonpath "$.data.updateTodo.isCompleted" == {{updated_todo_is_completed}}

# Verify the update via query
POST {{base_url}}/graphql
Content-Type: application/json
{
  "query": "query GetTodo($id: UUID!) { todo(id: $id) { title description isCompleted } }",
  "variables": {
    "id": "{{todo_id}}"
  }
}

HTTP 200
[Asserts]
jsonpath "$.data.todo.title" == "{{updated_todo_title}}"
jsonpath "$.data.todo.description" == "{{updated_todo_description}}"
jsonpath "$.data.todo.isCompleted" == {{updated_todo_is_completed}}

# Update a nonexistent todo (should return null, not error)
POST {{base_url}}/graphql
Content-Type: application/json
{
  "query": "mutation UpdateTodo($id: UUID!, $input: TodoInput!) { updateTodo(id: $id, input: $input) { id title } }",
  "variables": {
    "id": "00000000-0000-0000-0000-000000000000",
    "input": {
      "title": "{{updated_todo_title}}",
      "description": "{{updated_todo_description}}",
      "isCompleted": {{updated_todo_is_completed}}
    }
  }
}

HTTP 200
[Asserts]
jsonpath "$.data.updateTodo" == null
