import { useEffect } from 'react'
import { LoaderCircle } from 'lucide-react'
import { zodResolver } from '@hookform/resolvers/zod'
import { useForm } from 'react-hook-form'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Switch } from '@/components/ui/switch'
import { Textarea } from '@/components/ui/textarea'
import { todoInputSchema, type Todo, type TodoInput } from '../schema'

type TodoMutateDialogProps = {
  open: boolean
  currentTodo?: Todo
  onOpenChange: (open: boolean) => void
  onSubmit: (values: TodoInput) => void
  isPending?: boolean
}

export function TodoMutateDialog({
  open,
  currentTodo,
  onOpenChange,
  onSubmit,
  isPending = false,
}: TodoMutateDialogProps) {
  const isEdit = !!currentTodo
  const form = useForm<TodoInput>({
    resolver: zodResolver(todoInputSchema),
    defaultValues: getDefaultValues(currentTodo),
  })

  useEffect(() => {
    form.reset(getDefaultValues(currentTodo))
  }, [
    form,
    currentTodo?.id,
    currentTodo?.title,
    currentTodo?.description,
    currentTodo?.isCompleted,
    open,
  ])

  return (
    <Dialog
      open={open}
      onOpenChange={(nextOpen) => {
        if (isPending) return
        onOpenChange(nextOpen)
      }}
    >
      <DialogContent className='sm:max-w-lg'>
        <DialogHeader className='text-start'>
          <DialogTitle>{isEdit ? 'Edit todo' : 'Create todo'}</DialogTitle>
          <DialogDescription>
            {isEdit
              ? 'Update the todo details and save your changes.'
              : 'Add a new todo to the GraphQL-backed list.'}
          </DialogDescription>
        </DialogHeader>

        <Form {...form}>
          <form
            id='todo-form'
            onSubmit={form.handleSubmit(onSubmit)}
            className='space-y-4'
          >
            <FormField
              control={form.control}
              name='title'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Title</FormLabel>
                  <FormControl>
                    <Input
                      placeholder='Plan GraphQL integration'
                      autoComplete='off'
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name='description'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Description</FormLabel>
                  <FormControl>
                    <Textarea
                      placeholder='Capture the details you want to remember.'
                      className='min-h-28'
                      {...field}
                    />
                  </FormControl>
                  <FormDescription>
                    This maps directly to the GraphQL `description` field.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name='isCompleted'
              render={({ field }) => (
                <FormItem className='flex flex-row items-center justify-between rounded-lg border p-4'>
                  <div className='space-y-0.5'>
                    <FormLabel className='text-base'>Completed</FormLabel>
                    <FormDescription>
                      Toggle this on if the todo is already done.
                    </FormDescription>
                  </div>
                  <FormControl>
                    <Switch
                      checked={field.value}
                      onCheckedChange={field.onChange}
                    />
                  </FormControl>
                </FormItem>
              )}
            />
          </form>
        </Form>

        <DialogFooter>
          <Button
            type='button'
            variant='outline'
            onClick={() => onOpenChange(false)}
            disabled={isPending}
          >
            Cancel
          </Button>
          <Button type='submit' form='todo-form' disabled={isPending}>
            {isPending ? (
              <>
                <LoaderCircle className='animate-spin' />
                Saving...
              </>
            ) : isEdit ? (
              'Save changes'
            ) : (
              'Create todo'
            )}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

function getDefaultValues(todo?: Todo): TodoInput {
  return {
    title: todo?.title ?? '',
    description: todo?.description ?? '',
    isCompleted: todo?.isCompleted ?? false,
    tags: todo?.tags ?? [],
  }
}
