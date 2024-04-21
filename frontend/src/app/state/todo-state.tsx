'use client'

import React, { useState, createContext, useContext, FC, useEffect } from 'react'

import { ContractIds } from '@/deployments/deployments'
import { AccountId } from '@polkadot/types/interfaces'
import {
  contractQuery,
  decodeOutput,
  useInkathon,
  useRegisteredContract,
} from '@scio-labs/use-inkathon'
import toast from 'react-hot-toast'

import { contractTxWithToast } from '@/utils/contract-tx-with-toast'

import { decodeDate, encodeDate, encodeDayOfWeek } from '../helpers/date-converter'
import Note from '../model/Note'
import { useAppState } from './app-state'

// create the context object for delivering your state across your app.
const AppContext = createContext({
  isInitialized: false as boolean | undefined,
  today: new Date() as Date | undefined,
  notes: [] as Note[] as Note[] | undefined,
  contractAddress: undefined as string | AccountId | undefined,
  isLoading: false as boolean | undefined,
  fetchNotes: (async () => {}) as (() => Promise<void>) | undefined,
  initialize: (async () => {}) as (() => Promise<void>) | undefined,
  addNote: (async (title: string, description: string) => {}) as
    | ((title: string, description: string) => Promise<void>)
    | undefined,
  editNote: (async (
    noteId: number,
    title: string,
    description: string,
    isRepeating: boolean,
  ) => {}) as
    | ((noteId: number, title: string, description: string, isRepeating: boolean) => Promise<void>)
    | undefined,
  deleteNote: (async (noteId: number, isRepeating: boolean) => {}) as
    | ((noteId: number, isRepeating: boolean) => Promise<void>)
    | undefined,
  completeNote: (async (noteId: number) => {}) as ((noteId: number) => Promise<void>) | undefined,
})

// custom component to provide the state to your app
export const TodoState: FC<{ children: React.ReactNode }> = (props) => {
  const { api, activeAccount, activeSigner } = useInkathon()
  const { contract, address: contractAddress } = useRegisteredContract(ContractIds.Todo)

  const [isInitialized, setIsInitialized] = useState(false)
  const [today, setToday] = useState(new Date())
  const [notes, setNotes] = useState<Note[]>([])
  const [isLoading, setIsLoading] = useState(false)

  const {
    appState: { isDate, selectedDate, selectedDayOfWeek },
  } = useAppState()

  if (!contract || !api || !activeAccount || !activeSigner) {
    return (
      <AppContext.Provider
        value={{
          isInitialized: undefined,
          today: undefined,
          notes: undefined,
          contractAddress: undefined,
          isLoading: undefined,
          initialize: undefined,
          fetchNotes: undefined,
          addNote: undefined,
          editNote: undefined,
          deleteNote: undefined,
          completeNote: undefined,
        }}
      >
        {props.children}
      </AppContext.Provider>
    )
  }

  const readQuery = async (message: string, args: any[]) => {
    setIsLoading(true)
    try {
      const result = await contractQuery(
        api,
        activeAccount?.address,
        contract,
        message,
        undefined,
        args,
      )
      const { output, isError, decodedOutput } = decodeOutput(result, contract, message)

      if (isError) {
        throw new Error(decodedOutput)
      }

      console.log(output)

      return output
    } catch (e) {
      console.error(e)
      toast.error(`Error while executing ${message}. Try again…`)
      throw e
    } finally {
      setIsLoading(false)
    }
  }

  const editQuery = async (message: string, args: any[]) => {
    console.log(message, args)

    setIsLoading(true)
    try {
      await contractTxWithToast(api, activeAccount?.address, contract, message, {}, args)
    } catch (e) {
      console.error(e)
      throw e
    } finally {
      setIsLoading(false)
    }
  }

  const initialize = async () => {
    try {
      const today = await readQuery('today', [])
      const decodedToday = decodeDate(today)
      setToday(decodedToday)
      setIsInitialized(true)
    } catch (e) {
      console.log(e)
      throw e
    }
  }

  const fetchNotes = async () => {
    console.log(isDate, selectedDate, selectedDayOfWeek)
    const message = isDate ? 'get_notes' : 'get_repeating_notes'
    const args = isDate ? [encodeDate(selectedDate!)] : [encodeDayOfWeek(selectedDayOfWeek!)]

    try {
      const resultNotes = await readQuery(message, args)
      setNotes(resultNotes.Ok)
    } catch (_) {
      setNotes([])
    }
  }

  const addNote = async (title: string, description: string) => {
    const message = isDate ? 'add_note' : 'add_repeating_note'
    const args = [
      isDate ? encodeDate(selectedDate!) : encodeDayOfWeek(selectedDayOfWeek!),
      title,
      description,
    ]

    try {
      await editQuery(message, args)
      await fetchNotes()
    } catch (e) {
      console.error(e)
    }
  }

  const editNote = async (
    noteId: number,
    title: string,
    description: string,
    isRepeating: boolean,
  ) => {
    const dayOfWeek = isDate ? selectedDate!.getDay() : selectedDayOfWeek

    const message = !isRepeating ? 'edit_note' : 'edit_repeating_note'
    const args = [
      !isRepeating ? encodeDate(selectedDate!) : encodeDayOfWeek(dayOfWeek!),
      noteId,
      title,
      description,
    ]

    try {
      await editQuery(message, args)
      await fetchNotes()
    } catch (e) {
      console.error(e)
    }
  }

  const deleteNote = async (noteId: number, isRepeating: boolean) => {
    const dayOfWeek = isDate ? selectedDate!.getDay() : selectedDayOfWeek

    const message = !isRepeating ? 'delete_note' : 'delete_repeating_note'
    const args = [!isRepeating ? encodeDate(selectedDate!) : encodeDayOfWeek(dayOfWeek!), noteId]

    try {
      await editQuery(message, args)
      await fetchNotes()
    } catch (e) {
      console.error(e)
    }
  }

  const completeNote = async (noteId: number) => {
    const message = 'complete_note'
    const args = [encodeDate(selectedDate!), noteId]

    setIsLoading(true)
    try {
      await editQuery(message, args)
      await fetchNotes()
    } catch (e) {
      console.error(e)
    }
  }

  return (
    <AppContext.Provider
      value={{
        isInitialized,
        today,
        notes,
        contractAddress,
        isLoading,
        initialize,
        fetchNotes,
        addNote,
        editNote,
        deleteNote,
        completeNote,
      }}
    >
      {props.children}
    </AppContext.Provider>
  )
}

// custom hook for retrieving the provided state
export const useTodo = () => useContext(AppContext)
