# First create a todo to read
POST {{base_url}}/todos
Content-Type: application/json
{
  "title": "{{todo_title}}",
  "description": "{{todo_description}}",
  "is_completed": {{todo_is_completed}}
}

HTTP 201
[Captures]
todo_id: jsonpath "$.id"

# Get all todos
GET {{base_url}}/todos

HTTP 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$" count >= 1

# Get the specific todo
GET {{base_url}}/todos/{{todo_id}}

HTTP 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.id" == "{{todo_id}}"
jsonpath "$.title" == "{{todo_title}}"
jsonpath "$.description" == "{{todo_description}}"
jsonpath "$.is_completed" == {{todo_is_completed}}

# Get a nonexistent todo
GET {{base_url}}/todos/00000000-0000-0000-0000-000000000000

HTTP 404
