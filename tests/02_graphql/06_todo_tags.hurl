# Create a todo, set tags, read them back, then clear with an empty list
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

POST {{base_url}}/graphql
Content-Type: application/json
{
  "query": "mutation SetTodoTags($id: UUID!, $tags: [String!]!) { setTodoTags(id: $id, tags: $tags) { id tags } }",
  "variables": {
    "id": "{{todo_id}}",
    "tags": ["work", "urgent"]
  }
}

HTTP 200
[Asserts]
header "Content-Type" contains "application/graphql-response+json"
jsonpath "$.data.setTodoTags.id" == "{{todo_id}}"
jsonpath "$.data.setTodoTags.tags[0]" == "work"
jsonpath "$.data.setTodoTags.tags[1]" == "urgent"

POST {{base_url}}/graphql
Content-Type: application/json
{
  "query": "query GetTodo($id: UUID!) { todo(id: $id) { id tags } }",
  "variables": {
    "id": "{{todo_id}}"
  }
}

HTTP 200
[Asserts]
jsonpath "$.data.todo.id" == "{{todo_id}}"
jsonpath "$.data.todo.tags[0]" == "work"
jsonpath "$.data.todo.tags[1]" == "urgent"

POST {{base_url}}/graphql
Content-Type: application/json
{
  "query": "mutation SetTodoTags($id: UUID!, $tags: [String!]!) { setTodoTags(id: $id, tags: $tags) { id tags } }",
  "variables": {
    "id": "{{todo_id}}",
    "tags": []
  }
}

HTTP 200
[Asserts]
jsonpath "$.data.setTodoTags.id" == "{{todo_id}}"
jsonpath "$.data.setTodoTags.tags" count == 0

POST {{base_url}}/graphql
Content-Type: application/json
{
  "query": "mutation SetTodoTags($id: UUID!, $tags: [String!]!) { setTodoTags(id: $id, tags: $tags) { id } }",
  "variables": {
    "id": "00000000-0000-0000-0000-000000000000",
    "tags": ["noop"]
  }
}

HTTP 200
[Asserts]
jsonpath "$.data.setTodoTags" == null
