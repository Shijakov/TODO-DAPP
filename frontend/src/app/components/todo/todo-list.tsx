import { FC } from 'react'

import Note from '@/app/model/Note'

import Todo from './todo'

type TodoListProps = {
  notes: Note[]
}

const TodoList: FC<TodoListProps> = ({ notes }) => {
  return (
    <div className="flex w-5/6 flex-wrap">
      {notes.map((note) => (
        <Todo
          id={note.id}
          title={note.title}
          description={note.description}
          completed={note.completed}
          isRepeating={note.isRepeating}
          key={note.id}
        />
      ))}
    </div>
  )
}

export default TodoList
