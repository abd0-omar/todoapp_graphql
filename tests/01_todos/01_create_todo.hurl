# Create a todo
POST {{base_url}}/todos
Content-Type: application/json
{
  "title": "{{todo_title}}",
  "description": "{{todo_description}}",
  "is_completed": {{todo_is_completed}}
}

HTTP 201
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.id" exists
jsonpath "$.title" == "{{todo_title}}"
jsonpath "$.description" == "{{todo_description}}"
jsonpath "$.is_completed" == {{todo_is_completed}}
jsonpath "$.created_at" exists
jsonpath "$.updated_at" exists

[Captures]
todo_id: jsonpath "$.id"

# Create another todo for list tests
POST {{base_url}}/todos
Content-Type: application/json
{
  "title": "Second todo",
  "description": "Second description",
  "is_completed": false
}

HTTP 201
[Asserts]
jsonpath "$.id" exists
jsonpath "$.title" == "Second todo"
