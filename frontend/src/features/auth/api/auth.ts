import { graphqlRequest } from '@/lib/graphql-client'

export type AuthUserPayload = {
  id: string
  email: string
}

export type AuthPayload = {
  accessToken: string
  refreshToken: string
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
          refreshToken
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
          refreshToken
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

type RefreshTokenVariables = {
  input: {
    refreshToken: string
  }
}

type RefreshTokenResponse = {
  refreshToken: AuthPayload
}

export async function refreshSession(refreshToken: string) {
  const data = await graphqlRequest<RefreshTokenResponse, RefreshTokenVariables>({
    query: `
      mutation RefreshToken($input: RefreshTokenInput!) {
        refreshToken(input: $input) {
          accessToken
          refreshToken
          user {
            id
            email
          }
        }
      }
    `,
    variables: { input: { refreshToken } },
    auth: false,
  })
  return data.refreshToken
}

type LogoutVariables = {
  input: {
    refreshToken: string
  }
}

type LogoutResponse = {
  logout: boolean
}

export async function logout(refreshToken: string) {
  const data = await graphqlRequest<LogoutResponse, LogoutVariables>({
    query: `
      mutation Logout($input: LogoutInput!) {
        logout(input: $input)
      }
    `,
    variables: { input: { refreshToken } },
    auth: false,
  })
  return data.logout
}
