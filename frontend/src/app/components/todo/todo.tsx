import { FC, useState } from 'react'

import Note from '@/app/model/Note'

import TodoEdit from './todo-edit'
import TodoShow from './todo-show'

const Todo: FC<Note> = ({ id, title, description, completed, isRepeating }) => {
  const [editState, setEditState] = useState(false)

  let todo = <></>

  if (!editState) {
    todo = (
      <TodoShow
        id={id}
        title={title}
        description={description}
        completed={completed}
        isRepeating={isRepeating}
        handleEdit={() => setEditState(true)}
      />
    )
  } else {
    todo = (
      <TodoEdit
        id={id}
        title={title}
        description={description}
        completed={completed}
        isRepeating={isRepeating}
        handleCancel={() => setEditState(false)}
      />
    )
  }

  return <div className="m-5">{todo}</div>
}

export default Todo
