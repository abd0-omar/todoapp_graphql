# Create with empty title (invalid)
POST {{base_url}}/graphql
Content-Type: application/json
{
  "query": "mutation CreateTodo($input: TodoInput!) { createTodo(input: $input) { id title } }",
  "variables": {
    "input": {
      "title": "",
      "description": "{{todo_description}}",
      "isCompleted": {{todo_is_completed}}
    }
  }
}

HTTP 200
[Asserts]
jsonpath "$.errors" exists
jsonpath "$.errors" count >= 1
