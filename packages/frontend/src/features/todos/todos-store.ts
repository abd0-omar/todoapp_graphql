import {
  type KissAction,
  UserException,
  createStore,
  type Store,
} from 'kiss-for-react'
import type { Todo } from './schema'

export type RowPending = { kind: 'toggle' | 'delete'; todoId: string }

export type TodosState = {
  items: Todo[]
  rowPending: RowPending | null
}

export const initialTodosState: TodosState = {
  items: [],
  rowPending: null,
}

function actionLabel(action: KissAction<TodosState>) {
  return action.constructor.name
}

export const todosStore: Store<TodosState> = createStore<TodosState>({
  initialState: initialTodosState,
  logStateChanges: false,
  logger: () => {},
  globalWrapError(error: unknown) {
    if (error instanceof UserException) return error
    const message =
      error instanceof Error ? error.message : String(error ?? 'Unknown error')
    return new UserException(message, { errorText: message }).noDialog
  },
  actionObserver(action, dispatchCount, ini) {
    if (!import.meta.env.DEV) return
    const phase = ini ? 'dispatch' : 'finished'
    // eslint-disable-next-line no-console
    console.log(`[todos-store] action ${phase}`, {
      name: actionLabel(action),
      dispatchCount,
    })
  },
  stateObserver(action, prevState, newState, error, dispatchCount) {
    if (!import.meta.env.DEV) return
    if (prevState === newState && error == null) return
    // eslint-disable-next-line no-console
    console.log('[todos-store] state', {
      action: actionLabel(action),
      dispatchCount,
      changed: prevState !== newState,
      error: error ?? undefined,
    })
  },
  errorObserver(error, action, _store) {
    // eslint-disable-next-line no-console
    console.error('[todos-store] error', actionLabel(action), error)
    return true
  },
})
