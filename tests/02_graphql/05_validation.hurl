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

# Create with empty title (invalid)
POST {{base_url}}/graphql
Content-Type: application/json
Authorization: Bearer {{auth_token}}
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
