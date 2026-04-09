import { useEffect, useState } from 'react'
import {
  StoreProvider,
  useClearExceptionFor,
  useDispatchAndWait,
  useExceptionFor,
  useIsFailed,
  useIsWaiting,
  useSelect,
} from 'kiss-for-react'
import { AlertCircle, Plus } from 'lucide-react'
import { toast } from 'sonner'
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
import { ConfigDrawer } from '@/components/config-drawer'
import { Header } from '@/components/layout/header'
import { Main } from '@/components/layout/main'
import { ProfileDropdown } from '@/components/profile-dropdown'
import { Search } from '@/components/search'
import { ThemeSwitch } from '@/components/theme-switch'
import { handleServerError } from '@/lib/handle-server-error'
import {
  CreateTodoAction,
  DeleteTodoAction,
  LoadTodosAction,
  ToggleTodoAction,
  UpdateTodoAction,
} from './todos-actions'
import { todosStore, type TodosState } from './todos-store'
import { TodoDeleteDialog } from './components/todo-delete-dialog'
import {
  TodoEmptyState,
  TodoList,
  TodoListSkeleton,
} from './components/todo-list'
import { TodoMutateDialog } from './components/todo-mutate-dialog'
import { type Todo, type TodoInput } from './schema'

function TodoTotalBadge() {
  const totalTodos = useSelect((s: TodosState) => s.items.length)
  return <Badge variant='secondary'>{totalTodos} total</Badge>
}

function TodosContent() {
  const dispatchAndWait = useDispatchAndWait()
  const clearLoadException = useClearExceptionFor()

  const [isDialogOpen, setDialogOpen] = useState(false)
  const [currentTodo, setCurrentTodo] = useState<Todo | undefined>()
  const [todoToDelete, setTodoToDelete] = useState<Todo | undefined>()

  const items = useSelect((s: TodosState) => s.items)
  const rowPending = useSelect((s: TodosState) => s.rowPending)

  const loadWaiting = useIsWaiting(LoadTodosAction)
  const loadFailed = useIsFailed(LoadTodosAction)
  const loadException = useExceptionFor(LoadTodosAction)

  const createWaiting = useIsWaiting(CreateTodoAction)
  const updateWaiting = useIsWaiting(UpdateTodoAction)
  const deleteWaiting = useIsWaiting(DeleteTodoAction)

  useEffect(() => {
    todosStore.dispatch(new LoadTodosAction())
  }, [])

  const closeDialog = () => {
    setDialogOpen(false)
    setCurrentTodo(undefined)
  }

  const closeDeleteDialog = () => {
    setTodoToDelete(undefined)
  }

  const openCreateDialog = () => {
    setCurrentTodo(undefined)
    setDialogOpen(true)
  }

  const openEditDialog = (todo: Todo) => {
    setCurrentTodo(todo)
    setDialogOpen(true)
  }

  const handleDialogSubmit = async (values: TodoInput) => {
    if (currentTodo) {
      const status = await dispatchAndWait(
        new UpdateTodoAction(currentTodo.id, values)
      )
      if (status.isCompletedOk) {
        toast.success('Todo updated.')
        closeDialog()
      } else {
        handleServerError(status.wrappedError ?? status.originalError)
      }
      return
    }

    const status = await dispatchAndWait(new CreateTodoAction(values))
    if (status.isCompletedOk) {
      toast.success('Todo created.')
      closeDialog()
    } else {
      handleServerError(status.wrappedError ?? status.originalError)
    }
  }

  const isTogglePendingForTodo = (todoId: string) =>
    rowPending?.kind === 'toggle' && rowPending.todoId === todoId

  const isDeletePendingForTodo = (todoId: string) =>
    rowPending?.kind === 'delete' && rowPending.todoId === todoId

  const isRowActionPending = (todoId: string) =>
    isTogglePendingForTodo(todoId) || isDeletePendingForTodo(todoId)

  const isMutating = createWaiting || updateWaiting

  const loadErrorMessage =
    loadException?.message ??
    loadException?.errorText ??
    'The GraphQL API request failed.'

  const handleRetryLoad = () => {
    clearLoadException(LoadTodosAction)
    todosStore.dispatch(new LoadTodosAction())
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
            <TodoTotalBadge />
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
            {loadWaiting ? <TodoListSkeleton /> : null}

            {loadFailed ? (
              <Alert variant='destructive'>
                <AlertCircle />
                <AlertTitle>Unable to load todos</AlertTitle>
                <AlertDescription className='flex flex-col gap-2'>
                  <span>{loadErrorMessage}</span>
                  <Button
                    type='button'
                    variant='outline'
                    size='sm'
                    className='w-fit'
                    onClick={handleRetryLoad}
                  >
                    Retry
                  </Button>
                </AlertDescription>
              </Alert>
            ) : null}

            {!loadWaiting && !loadFailed && items.length === 0 ? (
              <TodoEmptyState
                action={
                  <Button onClick={openCreateDialog}>
                    <Plus />
                    Create your first todo
                  </Button>
                }
              />
            ) : null}

            {!loadWaiting && !loadFailed && items.length > 0 ? (
              <TodoList
                todos={items}
                renderActions={(todo) => (
                  <div className='flex flex-wrap justify-end gap-2'>
                    <Button
                      size='sm'
                      variant={todo.isCompleted ? 'outline' : 'default'}
                      disabled={isRowActionPending(todo.id)}
                      onClick={async () => {
                        const next = !todo.isCompleted
                        const status = await dispatchAndWait(
                          new ToggleTodoAction(todo, next)
                        )
                        if (status.isCompletedOk) {
                          toast.success(
                            next
                              ? 'Todo marked complete.'
                              : 'Todo marked open.'
                          )
                        } else {
                          handleServerError(
                            status.wrappedError ?? status.originalError
                          )
                        }
                      }}
                    >
                      {isTogglePendingForTodo(todo.id)
                        ? 'Saving...'
                        : todo.isCompleted
                          ? 'Mark open'
                          : 'Complete'}
                    </Button>
                    <Button
                      variant='outline'
                      size='sm'
                      disabled={isRowActionPending(todo.id)}
                      onClick={() => openEditDialog(todo)}
                    >
                      Edit
                    </Button>
                    <Button
                      variant='destructive'
                      size='sm'
                      disabled={isRowActionPending(todo.id)}
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
        onConfirm={async () => {
          if (!todoToDelete) return
          const status = await dispatchAndWait(
            new DeleteTodoAction(todoToDelete)
          )
          if (status.isCompletedOk) {
            toast.success('Todo deleted.')
            closeDeleteDialog()
          } else {
            handleServerError(status.wrappedError ?? status.originalError)
          }
        }}
        isPending={deleteWaiting}
      />
    </>
  )
}

export function Todos() {
  return (
    <StoreProvider store={todosStore}>
      <TodosContent />
    </StoreProvider>
  )
}
