import { AxiosError } from 'axios'

type GraphQLError = {
  message: string
  path?: Array<string | number>
  extensions?: Record<string, unknown>
}

type GraphQLResponse<TData> = {
  data?: TData
  errors?: GraphQLError[]
}

type GraphQLRequestOptions<TVariables> = {
  query: string
  variables?: TVariables
  signal?: AbortSignal
}

type GraphQLRequestErrorOptions = {
  status?: number
  errors?: GraphQLError[]
  response?: GraphQLResponse<unknown>
}

export class GraphQLRequestError extends Error {
  status?: number
  errors?: GraphQLError[]
  response?: GraphQLResponse<unknown>

  constructor(message: string, options: GraphQLRequestErrorOptions = {}) {
    super(message)
    this.name = 'GraphQLRequestError'
    this.status = options.status
    this.errors = options.errors
    this.response = options.response
  }
}

function getGraphQLApiUrl() {
  const apiUrl = import.meta.env.VITE_GRAPHQL_API_URL

  if (!apiUrl) {
    throw new GraphQLRequestError(
      'Missing VITE_GRAPHQL_API_URL. Set it in your frontend environment to reach the GraphQL API.'
    )
  }

  return apiUrl
}

function getErrorMessage<TData>(
  response: Response,
  payload?: GraphQLResponse<TData>
) {
  return (
    payload?.errors?.[0]?.message ||
    response.statusText ||
    `Request failed with status ${response.status}`
  )
}

async function readGraphQLResponse<TData>(response: Response) {
  const contentType = response.headers.get('content-type') ?? ''

  if (
    contentType.includes('application/json') ||
    contentType.includes('application/graphql-response+json')
  ) {
    return (await response.json()) as GraphQLResponse<TData>
  }

  const text = await response.text()
  throw new GraphQLRequestError(
    text || getErrorMessage(response),
    { status: response.status }
  )
}

export async function graphqlRequest<
  TData,
  TVariables extends Record<string, unknown> | undefined = undefined,
>({ query, variables, signal }: GraphQLRequestOptions<TVariables>) {
  try {
    const response = await fetch(getGraphQLApiUrl(), {
      method: 'POST',
      headers: {
        Accept: 'application/graphql-response+json, application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ query, variables }),
      signal,
    })

    const payload = await readGraphQLResponse<TData>(response)

    if (!response.ok || payload.errors?.length) {
      throw new GraphQLRequestError(getErrorMessage(response, payload), {
        status: response.status,
        errors: payload.errors,
        response: payload as GraphQLResponse<unknown>,
      })
    }

    if (!payload.data) {
      throw new GraphQLRequestError(
        'GraphQL response did not include any data.',
        {
          status: response.status,
          response: payload as GraphQLResponse<unknown>,
        }
      )
    }

    return payload.data
  } catch (error) {
    if (error instanceof GraphQLRequestError) {
      throw error
    }

    if (error instanceof DOMException && error.name === 'AbortError') {
      throw error
    }

    if (error instanceof Error) {
      throw new GraphQLRequestError(error.message)
    }

    throw new GraphQLRequestError('Unable to reach the GraphQL API.')
  }
}

export function getErrorStatus(error: unknown) {
  if (error instanceof AxiosError) {
    return error.response?.status
  }

  if (error instanceof GraphQLRequestError) {
    return error.status
  }

  if (error && typeof error === 'object' && 'status' in error) {
    const { status } = error

    if (typeof status === 'number') {
      return status
    }
  }

  return undefined
}
