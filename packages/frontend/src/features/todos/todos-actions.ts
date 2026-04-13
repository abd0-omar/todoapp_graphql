import { KissAction } from 'kiss-for-react'
import { createTodo, deleteTodo, getTodos, updateTodo } from './api/todos'
import type { Todo, TodoInput } from './schema'
import type { TodosState } from './todos-store'

export abstract class TodoBaseAction extends KissAction<TodosState> {}

/** Sync: mark a row as busy (toggle/delete). */
export class SetRowPendingAction extends TodoBaseAction {
  constructor(readonly payload: NonNullable<TodosState['rowPending']>) {
    super()
  }

  reduce(): TodosState {
    return { ...this.state, rowPending: this.payload }
  }
}

/** Sync: clear row pending (e.g. after a failed async op). */
export class ClearRowPendingAction extends TodoBaseAction {
  reduce(): TodosState {
    return { ...this.state, rowPending: null }
  }
}

export class LoadTodosAction extends TodoBaseAction {
  async reduce() {
    const todos = await getTodos()
    return (state: TodosState): TodosState => ({
      ...state,
      items: todos,
      rowPending: null,
    })
  }
}

export class CreateTodoAction extends TodoBaseAction {
  constructor(readonly input: TodoInput) {
    super()
  }

  async reduce() {
    const created = await createTodo(this.input)
    return (state: TodosState): TodosState => ({
      ...state,
      items: [...state.items, created],
    })
  }
}

export class UpdateTodoAction extends TodoBaseAction {
  constructor(
    readonly id: string,
    readonly input: TodoInput
  ) {
    super()
  }

  async reduce() {
    const updated = await updateTodo(this.id, this.input)
    return (state: TodosState): TodosState => ({
      ...state,
      items: state.items.map((t) => (t.id === updated.id ? updated : t)),
    })
  }
}

export class ToggleTodoAction extends TodoBaseAction {
  constructor(
    readonly todo: Todo,
    readonly isCompleted: boolean
  ) {
    super()
  }

  async reduce() {
    this.dispatchSync(
      new SetRowPendingAction({ kind: 'toggle', todoId: this.todo.id })
    )
    const updated = await updateTodo(this.todo.id, {
      title: this.todo.title,
      description: this.todo.description,
      isCompleted: this.isCompleted,
      tags: this.todo.tags,
    })
    return (state: TodosState): TodosState => ({
      ...state,
      items: state.items.map((t) => (t.id === updated.id ? updated : t)),
      rowPending: null,
    })
  }

  after() {
    if (this.store.state.rowPending !== null) {
      this.dispatchSync(new ClearRowPendingAction())
    }
  }
}

export class DeleteTodoAction extends TodoBaseAction {
  constructor(readonly todo: Todo) {
    super()
  }

  async reduce() {
    this.dispatchSync(
      new SetRowPendingAction({ kind: 'delete', todoId: this.todo.id })
    )
    await deleteTodo(this.todo.id)
    return (state: TodosState): TodosState => ({
      ...state,
      items: state.items.filter((t) => t.id !== this.todo.id),
      rowPending: null,
    })
  }

  after() {
    if (this.store.state.rowPending !== null) {
      this.dispatchSync(new ClearRowPendingAction())
    }
  }
}
