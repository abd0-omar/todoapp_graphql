# First create a todo to update
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

# Update the todo
PUT {{base_url}}/todos/{{todo_id}}
Content-Type: application/json
{
  "title": "{{updated_todo_title}}",
  "description": "{{updated_todo_description}}",
  "is_completed": {{updated_todo_is_completed}}
}

HTTP 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.id" == "{{todo_id}}"
jsonpath "$.title" == "{{updated_todo_title}}"
jsonpath "$.description" == "{{updated_todo_description}}"
jsonpath "$.is_completed" == {{updated_todo_is_completed}}

# Verify the update
GET {{base_url}}/todos/{{todo_id}}

HTTP 200
[Asserts]
jsonpath "$.title" == "{{updated_todo_title}}"
jsonpath "$.description" == "{{updated_todo_description}}"
jsonpath "$.is_completed" == {{updated_todo_is_completed}}

# Update a nonexistent todo
PUT {{base_url}}/todos/00000000-0000-0000-0000-000000000000
Content-Type: application/json
{
  "title": "{{updated_todo_title}}",
  "description": "{{updated_todo_description}}",
  "is_completed": {{updated_todo_is_completed}}
}

HTTP 404

# Update with invalid data (empty title)
PUT {{base_url}}/todos/{{todo_id}}
Content-Type: application/json
{
  "title": "",
  "description": "some description",
  "is_completed": false
}

HTTP 422
