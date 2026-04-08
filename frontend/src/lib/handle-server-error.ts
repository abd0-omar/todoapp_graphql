import { AxiosError } from 'axios'
import { toast } from 'sonner'
import { GraphQLRequestError, getErrorStatus } from './graphql-client'

export function handleServerError(error: unknown) {
  // eslint-disable-next-line no-console
  console.log(error)

  let errMsg = 'Something went wrong!'
  const status = getErrorStatus(error)

  if (status === 204) {
    errMsg = 'Content not found.'
  }

  if (error instanceof AxiosError) {
    errMsg = error.response?.data.title ?? error.message ?? errMsg
  }

  if (error instanceof GraphQLRequestError) {
    errMsg = error.message
  }

  if (error instanceof Error && errMsg === 'Something went wrong!') {
    errMsg = error.message || errMsg
  }

  toast.error(errMsg)
}
