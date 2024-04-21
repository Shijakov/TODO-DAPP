import { FC, useState } from 'react'

import { ContractIds } from '@/deployments/deployments'
import { useInkathon, useRegisteredContract } from '@scio-labs/use-inkathon'
import toast from 'react-hot-toast'

import { encodeDate, encodeDayOfWeek } from '@/app/helpers/date-converter'
import { useAppState } from '@/app/state/app-state'
import { useTodo } from '@/app/state/todo-state'
import { Button } from '@/components/ui/button'
import { contractTxWithToast } from '@/utils/contract-tx-with-toast'

import TodoForm from './todo-form'

type TodoFormValues = {
  title: string
  description: string
}

const TodoAdd: FC = () => {
  const [addState, setAddState] = useState(false)
  const { appState } = useAppState()

  const { addNote, isLoading } = useTodo()

  const handleSave = async (title: string, description: string) => {
    if (!addNote) {
      return
    }

    await addNote(title.toString(), description.toString())
  }

  const handleCancel = () => {
    setAddState(false)
  }

  const handleOpen = () => {
    if (!appState.selectedDate && !appState.selectedDayOfWeek) {
      return
    }

    setAddState(true)
  }

  if (!addState) {
    return (
      <div className="content-center">
        <Button
          disabled={
            (appState.selectedDate == undefined && appState.selectedDayOfWeek == undefined) ||
            isLoading
          }
          onClick={handleOpen}
        >
          Add Note
        </Button>
      </div>
    )
  } else {
    return (
      <TodoForm handleSave={handleSave} disabled={isLoading}>
        <Button disabled={isLoading} type="submit">
          Save
        </Button>
        <Button disabled={isLoading} type="button" onClick={handleCancel}>
          Cancel
        </Button>
      </TodoForm>
    )
  }
}

export default TodoAdd
