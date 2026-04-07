# First create a todo to delete
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

# Verify todo exists
GET {{base_url}}/todos/{{todo_id}}

HTTP 200

# Delete the todo
DELETE {{base_url}}/todos/{{todo_id}}

HTTP 204

# Verify todo is deleted
GET {{base_url}}/todos/{{todo_id}}

HTTP 404

# Delete a nonexistent todo
DELETE {{base_url}}/todos/00000000-0000-0000-0000-000000000000

HTTP 404
