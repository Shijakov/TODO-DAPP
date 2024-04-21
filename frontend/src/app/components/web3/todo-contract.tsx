'use client'

import { FC, useEffect } from 'react'

import { getFormattedDateName } from '@/app/helpers/date-converter'
import { useAppState } from '@/app/state/app-state'
import { useTodo } from '@/app/state/todo-state'
import { Spinner } from '@/components/ui/spinner'

import TodoAdd from '../todo/todo-add'
import TodoList from '../todo/todo-list'

const TodoContract: FC = () => {
  const { initialize, isInitialized, today, notes, fetchNotes } = useTodo()
  const {
    setDate,
    appState: { selectedDate, selectedDayOfWeek },
  } = useAppState()

  useEffect(() => {
    if (isInitialized && today) {
      setDate(today)
    }
  }, [isInitialized, today])

  useEffect(() => {
    if (fetchNotes) {
      fetchNotes().then((_) => console.log('FETCHED NOTES'))
    }
  }, [selectedDate, selectedDayOfWeek])

  useEffect(() => {
    if (initialize) {
      initialize()
        .then((_) => console.log('INITIALIZED'))
        .catch((_) => console.log('ERROR IN INITIALIZATION'))
    }
  }, [isInitialized])

  if (!isInitialized) {
    return <Spinner />
  } else {
    return (
      <div className="flex w-full flex-col items-center">
        <div className="mb-3">{today ? `Today: ${getFormattedDateName(today)}` : 'ERROR'}</div>
        <TodoAdd />
        <TodoList notes={notes!} />
      </div>
    )
  }
}

export default TodoContract
