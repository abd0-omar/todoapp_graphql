import { useState } from 'react'
import { AlertCircle, Plus } from 'lucide-react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { ConfigDrawer } from '@/components/config-drawer'
import { Header } from '@/components/layout/header'
import { Main } from '@/components/layout/main'
import { ProfileDropdown } from '@/components/profile-dropdown'
import { Search } from '@/components/search'
import { ThemeSwitch } from '@/components/theme-switch'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { createTodo, deleteTodo, getTodos, todoKeys, updateTodo } from './api/todos'
import { TodoDeleteDialog } from './components/todo-delete-dialog'
import {
  TodoEmptyState,
  TodoList,
  TodoListSkeleton,
} from './components/todo-list'
import { TodoMutateDialog } from './components/todo-mutate-dialog'
import { type Todo, type TodoInput } from './schema'

export function Todos() {
  const queryClient = useQueryClient()
  const [isDialogOpen, setDialogOpen] = useState(false)
  const [currentTodo, setCurrentTodo] = useState<Todo | undefined>()
  const [todoToDelete, setTodoToDelete] = useState<Todo | undefined>()
  const todosQuery = useQuery({
    queryKey: todoKeys.all,
    queryFn: ({ signal }) => getTodos(signal),
  })

  const replaceTodoInCache = (updatedTodo: Todo) => {
    queryClient.setQueryData<Todo[]>(todoKeys.all, (currentTodos = []) =>
      currentTodos.map((todo) =>
        todo.id === updatedTodo.id ? updatedTodo : todo
      )
    )
  }

  const removeTodoFromCache = (todoId: string) => {
    queryClient.setQueryData<Todo[]>(todoKeys.all, (currentTodos = []) =>
      currentTodos.filter((todo) => todo.id !== todoId)
    )
  }

  const closeDialog = () => {
    setDialogOpen(false)
    setCurrentTodo(undefined)
  }

  const closeDeleteDialog = () => {
    setTodoToDelete(undefined)
  }

  const createTodoMutation = useMutation({
    mutationFn: createTodo,
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: todoKeys.all })
      toast.success('Todo created.')
      closeDialog()
    },
  })

  const updateTodoMutation = useMutation({
    mutationFn: ({ id, input }: { id: string; input: TodoInput }) =>
      updateTodo(id, input),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: todoKeys.all })
      toast.success('Todo updated.')
      closeDialog()
    },
  })

  const toggleTodoMutation = useMutation({
    mutationFn: ({ todo, isCompleted }: { todo: Todo; isCompleted: boolean }) =>
      updateTodo(todo.id, {
        title: todo.title,
        description: todo.description,
        isCompleted,
      }),
    onSuccess: (updatedTodo, variables) => {
      replaceTodoInCache(updatedTodo)
      toast.success(
        variables.isCompleted ? 'Todo marked complete.' : 'Todo marked open.'
      )
    },
  })

  const deleteTodoMutation = useMutation({
    mutationFn: async (todo: Todo) => {
      await deleteTodo(todo.id)
      return todo
    },
    onSuccess: (deletedTodo) => {
      removeTodoFromCache(deletedTodo.id)
      toast.success('Todo deleted.')
      closeDeleteDialog()
    },
  })

  const isMutating =
    createTodoMutation.isPending || updateTodoMutation.isPending
  const totalTodos = todosQuery.data?.length ?? 0

  const openCreateDialog = () => {
    setCurrentTodo(undefined)
    setDialogOpen(true)
  }

  const openEditDialog = (todo: Todo) => {
    setCurrentTodo(todo)
    setDialogOpen(true)
  }

  const handleDialogSubmit = (values: TodoInput) => {
    if (currentTodo) {
      updateTodoMutation.mutate({ id: currentTodo.id, input: values })
      return
    }

    createTodoMutation.mutate(values)
  }

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
          <div className='flex flex-wrap items-center gap-2'>
            <Badge variant='secondary'>{totalTodos} total</Badge>
            <Button onClick={openCreateDialog}>
              <Plus />
              New todo
            </Button>
          </div>
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
              <TodoEmptyState
                action={
                  <Button onClick={openCreateDialog}>
                    <Plus />
                    Create your first todo
                  </Button>
                }
              />
            ) : null}

            {todosQuery.isSuccess && todosQuery.data.length > 0 ? (
              <TodoList
                todos={todosQuery.data}
                renderActions={(todo) => (
                  <div className='flex flex-wrap justify-end gap-2'>
                    <Button
                      size='sm'
                      variant={todo.isCompleted ? 'outline' : 'default'}
                      disabled={
                        toggleTodoMutation.isPending ||
                        deleteTodoMutation.isPending
                      }
                      onClick={() =>
                        toggleTodoMutation.mutate({
                          todo,
                          isCompleted: !todo.isCompleted,
                        })
                      }
                    >
                      {toggleTodoMutation.isPending &&
                      toggleTodoMutation.variables?.todo.id === todo.id
                        ? 'Saving...'
                        : todo.isCompleted
                          ? 'Mark open'
                          : 'Complete'}
                    </Button>
                    <Button
                      variant='outline'
                      size='sm'
                      disabled={
                        toggleTodoMutation.isPending ||
                        deleteTodoMutation.isPending
                      }
                      onClick={() => openEditDialog(todo)}
                    >
                      Edit
                    </Button>
                    <Button
                      variant='destructive'
                      size='sm'
                      disabled={
                        toggleTodoMutation.isPending ||
                        deleteTodoMutation.isPending
                      }
                      onClick={() => setTodoToDelete(todo)}
                    >
                      Delete
                    </Button>
                  </div>
                )}
              />
            ) : null}
          </CardContent>
        </Card>
      </Main>

      <TodoMutateDialog
        open={isDialogOpen}
        currentTodo={currentTodo}
        onOpenChange={(open) => {
          if (!open) {
            closeDialog()
            return
          }

          setDialogOpen(true)
        }}
        onSubmit={handleDialogSubmit}
        isPending={isMutating}
      />

      <TodoDeleteDialog
        open={!!todoToDelete}
        currentTodo={todoToDelete}
        onOpenChange={(open) => {
          if (!open) {
            closeDeleteDialog()
          }
        }}
        onConfirm={() => {
          if (!todoToDelete) return
          deleteTodoMutation.mutate(todoToDelete)
        }}
        isPending={deleteTodoMutation.isPending}
      />
    </>
  )
}
