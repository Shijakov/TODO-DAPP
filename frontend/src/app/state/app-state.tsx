'use client'

import React, { useState, createContext, useContext, FC } from 'react'

import DayOfWeek from '../model/DayOfWeek'

type InitialState = {
  isDate: boolean
  selectedDate?: Date
  selectedDayOfWeek?: DayOfWeek
  today?: Date
}

// The initial state, you can setup any properties initilal values here.
const initialState: InitialState = {
  isDate: false,
  selectedDate: undefined,
  selectedDayOfWeek: undefined,
  today: undefined,
}

// create the context object for delivering your state across your app.
const AppContext = createContext({
  appState: {} as InitialState,
  setDate: (date: Date) => {},
  setDayOfWeek: (day: DayOfWeek) => {},
  setToday: (date: Date) => {},
})

// custom component to provide the state to your app
export const AppState: FC<{ children: React.ReactNode }> = (props) => {
  // declare the GlobalState
  const [appState, setAppState] = useState(initialState)

  const setDate = (date: Date) => {
    setAppState({
      isDate: true,
      selectedDate: date,
      selectedDayOfWeek: undefined,
    })
  }

  const setDayOfWeek = (day: DayOfWeek) => {
    setAppState({
      isDate: false,
      selectedDate: undefined,
      selectedDayOfWeek: day,
    })
  }

  const setToday = (date: Date) => {
    setAppState((prev) => ({ ...prev, today: date }))
  }

  return (
    <AppContext.Provider value={{ appState, setDate, setDayOfWeek, setToday }}>
      {props.children}
    </AppContext.Provider>
  )
}

// custom hook for retrieving the provided state
export const useAppState = () => useContext(AppContext)
