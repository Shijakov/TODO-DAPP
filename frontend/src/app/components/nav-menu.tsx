import { FC } from 'react'

import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Select } from '@/components/ui/select'

import { encodeAsDateString } from '../helpers/date-converter'
import DayOfWeek from '../model/DayOfWeek'
import { useAppState } from '../state/app-state'
import { useTodo } from '../state/todo-state'

const NavMenu: FC = () => {
  const {
    appState: { selectedDate, selectedDayOfWeek, isDate },
    setDate,
    setDayOfWeek,
  } = useAppState()
  const { today, isLoading } = useTodo()

  const onDateChange = (event: any) => {
    setDate(new Date(event.target.value))
  }

  const onDayOfWeekChange = (event: any) => {
    const dayOfWeek = event.target.value as DayOfWeek
    console.log(dayOfWeek)
    setDayOfWeek(dayOfWeek)
  }

  if (!selectedDate && !selectedDayOfWeek) {
    return (
      <div className="flex flex-row">
        <Input type="date" className="m-3 min-w-[9rem]" disabled />
        <Button translate="no" className="m-3 min-w-[9rem]" disabled>
          Repeating notes
        </Button>
      </div>
    )
  }

  if (isDate) {
    return (
      <div className="flex flex-row">
        <Input
          type="date"
          className="m-3 min-w-[9rem]"
          defaultValue={encodeAsDateString(selectedDate!)}
          onChange={onDateChange}
          disabled={isLoading}
        />
        <Button
          translate="no"
          className="m-3 min-w-[9rem]"
          onClick={() => setDayOfWeek(today!.getDay())}
          disabled={isLoading}
        >
          Repeating notes
        </Button>
      </div>
    )
  } else {
    return (
      <span className="flex flex-row">
        <Button
          translate="no"
          className="m-3 min-w-[9rem]"
          onClick={() => setDate(today!)}
          disabled={isLoading}
        >
          Daily notes
        </Button>

        <span className="m-3">
          <Select
            onChange={onDayOfWeekChange}
            defaultValue={selectedDayOfWeek!}
            disabled={isLoading}
          >
            {Object.values(DayOfWeek)
              .filter((v) => !isNaN(Number(v)))
              .map((v) => Number(v))
              .map((day) => (
                <option key={day} value={day}>
                  {DayOfWeek[day]}
                </option>
              ))}
          </Select>
        </span>
      </span>
    )
  }
}

export default NavMenu
