import { createFileRoute } from '@tanstack/react-router'
import { Todos } from '@/features/todos'

export const Route = createFileRoute('/_authenticated/todos/')({
  component: Todos,
})
