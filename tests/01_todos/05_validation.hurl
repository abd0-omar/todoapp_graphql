# Create with empty title (invalid)
POST {{base_url}}/todos
Content-Type: application/json
{
  "title": "",
  "description": "{{todo_description}}",
  "is_completed": {{todo_is_completed}}
}

HTTP 422
