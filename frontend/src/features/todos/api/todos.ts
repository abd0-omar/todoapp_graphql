import { graphqlRequest } from '@/lib/graphql-client'

export type Todo = {
  id: string
  title: string
  description: string
  tags: string[]
  isCompleted: boolean
  createdAt: string
  updatedAt: string
}

type TodosQueryResponse = {
  todos: Todo[]
}

export const todoKeys = {
  all: ['todos'] as const,
}

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

  return data.todos
}
