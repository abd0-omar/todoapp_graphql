import { AlertTriangle } from 'lucide-react'
import { ConfirmDialog } from '@/components/confirm-dialog'
import { type Todo } from '../schema'

type TodoDeleteDialogProps = {
  open: boolean
  currentTodo?: Todo
  onOpenChange: (open: boolean) => void
  onConfirm: () => void
  isPending?: boolean
}

export function TodoDeleteDialog({
  open,
  currentTodo,
  onOpenChange,
  onConfirm,
  isPending = false,
}: TodoDeleteDialogProps) {
  if (!currentTodo) {
    return null
  }

  return (
    <ConfirmDialog
      open={open}
      onOpenChange={onOpenChange}
      handleConfirm={onConfirm}
      isLoading={isPending}
      title={
        <span className='text-destructive'>
          <AlertTriangle
            className='me-1 inline-block stroke-destructive'
            size={18}
          />{' '}
          Delete todo
        </span>
      }
      desc={
        <div className='space-y-4'>
          <p>
            You are about to delete <strong>{currentTodo.title}</strong>. This
            removes the todo from the GraphQL API and cannot be undone.
          </p>
          <p className='text-muted-foreground'>
            ID: <span className='font-mono'>{currentTodo.id}</span>
          </p>
        </div>
      }
      confirmText={isPending ? 'Deleting...' : 'Delete'}
      destructive
    />
  )
}
