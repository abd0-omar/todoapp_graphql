import { z } from 'zod'
import { GraphQLRequestError, graphqlRequest } from '@/lib/graphql-client'
import {
  todoInputSchema,
  todoSchema,
  type Todo,
  type TodoInput,
} from '../schema'

type TodosQueryResponse = {
  todos: Todo[]
}

type CreateTodoMutationResponse = {
  createTodo: Todo
}

type CreateTodoMutationVariables = {
  input: TodoInput
}

type UpdateTodoMutationResponse = {
  updateTodo: Todo | null
}

type UpdateTodoMutationVariables = {
  id: string
  input: TodoInput
}

type DeleteTodoMutationResponse = {
  deleteTodo: boolean
}

type DeleteTodoMutationVariables = {
  id: string
}

const todosQueryResponseSchema = z.object({
  todos: z.array(todoSchema),
})

const createTodoMutationResponseSchema = z.object({
  createTodo: todoSchema,
})

const updateTodoMutationResponseSchema = z.object({
  updateTodo: todoSchema.nullable(),
})

const deleteTodoMutationResponseSchema = z.object({
  deleteTodo: z.boolean(),
})

export const TODO_FIELDS = `
  id
  title
  description
  tags
  isCompleted
  createdAt
  updatedAt
`

export async function getTodos(signal?: AbortSignal) {
  const data = await graphqlRequest<TodosQueryResponse>({
    query: `
      query GetTodos {
        todos {
          ${TODO_FIELDS}
        }
      }
    `,
    signal,
  })

  return todosQueryResponseSchema.parse(data).todos
}

export async function createTodo(input: TodoInput) {
  const parsedInput = todoInputSchema.parse(input)
  const data = await graphqlRequest<
    CreateTodoMutationResponse,
    CreateTodoMutationVariables
  >({
    query: `
      mutation CreateTodo($input: TodoInput!) {
        createTodo(input: $input) {
          ${TODO_FIELDS}
        }
      }
    `,
    variables: {
      input: parsedInput,
    },
  })

  return createTodoMutationResponseSchema.parse(data).createTodo
}

export async function updateTodo(id: string, input: TodoInput) {
  const parsedInput = todoInputSchema.parse(input)
  const data = await graphqlRequest<
    UpdateTodoMutationResponse,
    UpdateTodoMutationVariables
  >({
    query: `
      mutation UpdateTodo($id: UUID!, $input: TodoInput!) {
        updateTodo(id: $id, input: $input) {
          ${TODO_FIELDS}
        }
      }
    `,
    variables: {
      id,
      input: parsedInput,
    },
  })

  const updatedTodo = updateTodoMutationResponseSchema.parse(data).updateTodo

  if (!updatedTodo) {
    throw new GraphQLRequestError('Todo not found.', { status: 404 })
  }

  return updatedTodo
}

export async function deleteTodo(id: string) {
  const data = await graphqlRequest<
    DeleteTodoMutationResponse,
    DeleteTodoMutationVariables
  >({
    query: `
      mutation DeleteTodo($id: UUID!) {
        deleteTodo(id: $id)
      }
    `,
    variables: {
      id,
    },
  })

  const deleted = deleteTodoMutationResponseSchema.parse(data).deleteTodo

  if (!deleted) {
    throw new GraphQLRequestError('Todo not found.', { status: 404 })
  }

  return deleted
}

export type { Todo, TodoInput } from '../schema'
