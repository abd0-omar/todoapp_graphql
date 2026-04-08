import { AlertCircle } from 'lucide-react'
import { useQuery } from '@tanstack/react-query'
import { ConfigDrawer } from '@/components/config-drawer'
import { Header } from '@/components/layout/header'
import { Main } from '@/components/layout/main'
import { ProfileDropdown } from '@/components/profile-dropdown'
import { Search } from '@/components/search'
import { ThemeSwitch } from '@/components/theme-switch'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { getTodos, todoKeys } from './api/todos'
import {
  TodoEmptyState,
  TodoList,
  TodoListSkeleton,
} from './components/todo-list'

export function Todos() {
  const todosQuery = useQuery({
    queryKey: todoKeys.all,
    queryFn: ({ signal }) => getTodos(signal),
  })

  const totalTodos = todosQuery.data?.length ?? 0

  return (
    <>
      <Header fixed>
        <Search />
        <div className='ms-auto flex items-center space-x-4'>
          <ThemeSwitch />
          <ConfigDrawer />
          <ProfileDropdown />
        </div>
      </Header>

      <Main className='flex flex-1 flex-col gap-4 sm:gap-6'>
        <div className='flex flex-wrap items-end justify-between gap-3'>
          <div>
            <h2 className='text-2xl font-bold tracking-tight'>Todos</h2>
            <p className='text-muted-foreground'>
              Live todo items loaded from the Rust GraphQL API.
            </p>
          </div>
          <Badge variant='secondary'>{totalTodos} total</Badge>
        </div>

        <Card>
          <CardHeader>
            <CardTitle>All todos</CardTitle>
            <CardDescription>
              The list below reflects the current `todos` query response.
            </CardDescription>
          </CardHeader>
          <CardContent className='space-y-4'>
            {todosQuery.isPending ? <TodoListSkeleton /> : null}

            {todosQuery.isError ? (
              <Alert variant='destructive'>
                <AlertCircle />
                <AlertTitle>Unable to load todos</AlertTitle>
                <AlertDescription>
                  {todosQuery.error instanceof Error
                    ? todosQuery.error.message
                    : 'The GraphQL API request failed.'}
                </AlertDescription>
              </Alert>
            ) : null}

            {todosQuery.isSuccess && todosQuery.data.length === 0 ? (
              <TodoEmptyState />
            ) : null}

            {todosQuery.isSuccess && todosQuery.data.length > 0 ? (
              <TodoList todos={todosQuery.data} />
            ) : null}
          </CardContent>
        </Card>
      </Main>
    </>
  )
}
