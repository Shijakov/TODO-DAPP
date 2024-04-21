import DayOfWeek from '../model/DayOfWeek'

export const decodeDate = (date: string[]) => {
  const [year, month, day] = [
    Number(date[0].split(',').join('')),
    Number(date[1]) - 1,
    Number(date[2]),
  ]
  return new Date(year, month, day)
}

export const encodeDate = (date: Date) => {
  return [date.getFullYear(), date.getMonth() + 1, date.getDate()]
}

export const encodeDayOfWeek = (dayOfWeek: DayOfWeek) => {
  return DayOfWeek[dayOfWeek].toLowerCase()
}

export const encodeAsDateString = (date: Date) => {
  const year = date.getFullYear()
  const month = date.getMonth() + 1
  const day = date.getDate()
  return `${year}-${month < 10 ? `0${month}` : month}-${day < 10 ? `0${day}` : day}`
}

export const getFormattedDateName = (date: Date) => {
  return date.toDateString()
}
