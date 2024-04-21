import { FC, useState } from 'react'

import { ContractIds } from '@/deployments/deployments'
import { useInkathon, useRegisteredContract } from '@scio-labs/use-inkathon'
import toast from 'react-hot-toast'

import { encodeDate } from '@/app/helpers/date-converter'
import Note from '@/app/model/Note'
import { useAppState } from '@/app/state/app-state'
import { useTodo } from '@/app/state/todo-state'
import { Button } from '@/components/ui/button'
import CheckMarker from '@/components/ui/check-marker'
import { contractTxWithToast } from '@/utils/contract-tx-with-toast'

import TodoForm from './todo-form'

type TodoShowProps = {
  handleEdit: () => void
}

const TodoShow: FC<Note & TodoShowProps> = ({
  id,
  title,
  description,
  completed,
  handleEdit,
  isRepeating,
}) => {
  const { appState } = useAppState()

  const { completeNote, isLoading } = useTodo()

  const onComplete = async () => {
    if (!completeNote) {
      return
    }

    await completeNote(id)
  }

  return (
    <TodoForm
      repeatingBorder={isRepeating}
      handleSave={async (_, __) => {}}
      titleValue={title}
      descriptionValue={description}
      disabled
    >
      {appState.isDate && !completed && (
        <Button disabled={isLoading} type="button" onClick={onComplete}>
          Complete
        </Button>
      )}
      <Button disabled={isLoading} type="button" onClick={handleEdit}>
        Edit
      </Button>
      <CheckMarker completed={completed} />
    </TodoForm>
  )
}

export default TodoShow
