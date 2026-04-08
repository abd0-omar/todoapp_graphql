import { formatDistanceToNow } from 'date-fns'
import { ListTodo } from 'lucide-react'
import { cn } from '@/lib/utils'
import { Badge } from '@/components/ui/badge'
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Skeleton } from '@/components/ui/skeleton'
import { type Todo } from '../api/todos'

type TodoListProps = {
  todos: Todo[]
  renderActions?: (todo: Todo) => React.ReactNode
}

export function TodoList({ todos, renderActions }: TodoListProps) {
  return (
    <div className='space-y-4'>
      {todos.map((todo) => (
        <Card key={todo.id}>
          <CardHeader className='gap-3 sm:flex-row sm:items-start sm:justify-between'>
            <div className='space-y-2'>
              <div className='flex flex-wrap items-center gap-2'>
                <Badge variant={todo.isCompleted ? 'secondary' : 'outline'}>
                  {todo.isCompleted ? 'Completed' : 'Open'}
                </Badge>
                <span className='text-xs text-muted-foreground'>
                  Updated {formatTodoDate(todo.updatedAt)}
                </span>
              </div>
              <CardTitle className={cn(todo.isCompleted && 'line-through')}>
                {todo.title}
              </CardTitle>
              <CardDescription className='text-sm leading-6 whitespace-pre-wrap'>
                {todo.description || 'No description yet.'}
              </CardDescription>
            </div>
            {renderActions && (
              <CardAction className='w-full sm:w-auto'>
                {renderActions(todo)}
              </CardAction>
            )}
          </CardHeader>
          <CardContent className='flex flex-wrap items-center gap-2 text-xs text-muted-foreground'>
            <span>ID: {todo.id}</span>
            <span aria-hidden='true'>&#8226;</span>
            <span>Created {formatTodoDate(todo.createdAt)}</span>
          </CardContent>
        </Card>
      ))}
    </div>
  )
}

export function TodoListSkeleton() {
  return (
    <div className='space-y-4'>
      {Array.from({ length: 3 }, (_, index) => (
        <Card key={index}>
          <CardHeader className='space-y-3'>
            <Skeleton className='h-5 w-24' />
            <Skeleton className='h-6 w-2/3' />
            <Skeleton className='h-4 w-full' />
            <Skeleton className='h-4 w-5/6' />
          </CardHeader>
          <CardContent className='flex gap-2'>
            <Skeleton className='h-4 w-24' />
            <Skeleton className='h-4 w-36' />
          </CardContent>
        </Card>
      ))}
    </div>
  )
}

export function TodoEmptyState() {
  return (
    <Card>
      <CardHeader>
        <div className='flex items-center gap-2 text-muted-foreground'>
          <ListTodo className='size-4' />
          <span className='text-sm font-medium'>No todos yet</span>
        </div>
        <CardTitle>Your todo list is empty</CardTitle>
        <CardDescription>
          Create your first todo once the write actions are added to this page.
        </CardDescription>
      </CardHeader>
    </Card>
  )
}

function formatTodoDate(value: string) {
  const date = new Date(value)

  if (Number.isNaN(date.getTime())) {
    return 'just now'
  }

  return formatDistanceToNow(date, { addSuffix: true })
}
