import { FC, useState } from 'react'

import toast from 'react-hot-toast'

import Note from '@/app/model/Note'
import { useTodo } from '@/app/state/todo-state'
import { Button } from '@/components/ui/button'
import CheckMarker from '@/components/ui/check-marker'

import TodoForm from './todo-form'

type TodoEditProps = {
  handleCancel: () => void
}

const TodoEdit: FC<Note & TodoEditProps> = ({
  id,
  isRepeating,
  title,
  description,
  completed,
  handleCancel,
}) => {
  const { editNote, deleteNote, isLoading } = useTodo()

  const handleSave = async (title: string, description: string) => {
    if (!editNote) {
      toast.error('Wallet not connected. Try again…')
      return
    }

    await editNote(id, title.toString(), description.toString(), isRepeating)
  }

  const handleDelete = async () => {
    if (!deleteNote) {
      toast.error('Wallet not connected. Try again…')
      return
    }

    await deleteNote(id, isRepeating)
  }

  return (
    <TodoForm
      repeatingBorder={isRepeating}
      handleSave={handleSave}
      titleValue={title}
      descriptionValue={description}
      disabled={isLoading}
    >
      <Button disabled={isLoading} type="submit">
        Save
      </Button>
      <Button disabled={isLoading} type="button" onClick={handleCancel}>
        Cancel
      </Button>
      <Button disabled={isLoading} type="button" onClick={handleDelete}>
        Delete
      </Button>
      <CheckMarker completed={completed} />
    </TodoForm>
  )
}

export default TodoEdit
