import { z } from 'zod'

export function normalizeTags(tags: string[]) {
  return [...new Set(tags.map((tag) => tag.trim()).filter(Boolean))]
}

export const todoSchema = z.object({
  id: z.uuid(),
  title: z.string(),
  description: z.string(),
  tags: z.array(z.string()),
  isCompleted: z.boolean(),
  createdAt: z.string().min(1),
  updatedAt: z.string().min(1),
})

export const todoInputSchema = z.object({
  title: z.string().trim().min(1, 'Title is required.'),
  description: z.string().trim(),
  isCompleted: z.boolean(),
  tags: z.array(z.string()).transform(normalizeTags),
})

export type Todo = z.infer<typeof todoSchema>
export type TodoInput = z.infer<typeof todoInputSchema>
