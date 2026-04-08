import { graphqlRequest } from '@/lib/graphql-client'

export type AuthUserPayload = {
  id: string
  email: string
}

export type AuthPayload = {
  accessToken: string
  user: AuthUserPayload
}

type LoginVariables = {
  input: {
    email: string
    password: string
  }
}

type LoginResponse = {
  login: AuthPayload
}

type SignUpVariables = {
  input: {
    email: string
    password: string
  }
}

type SignUpResponse = {
  signUp: AuthPayload
}

export async function login(input: LoginVariables['input']) {
  const data = await graphqlRequest<LoginResponse, LoginVariables>({
    query: `
      mutation Login($input: LoginInput!) {
        login(input: $input) {
          accessToken
          user {
            id
            email
          }
        }
      }
    `,
    variables: { input },
    auth: false,
  })
  return data.login
}

export async function signUp(input: SignUpVariables['input']) {
  const data = await graphqlRequest<SignUpResponse, SignUpVariables>({
    query: `
      mutation SignUp($input: SignUpInput!) {
        signUp(input: $input) {
          accessToken
          user {
            id
            email
          }
        }
      }
    `,
    variables: { input },
    auth: false,
  })
  return data.signUp
}
