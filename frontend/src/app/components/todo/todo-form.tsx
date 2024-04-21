import { FC, PropsWithChildren } from 'react'

import { bool } from '@polkadot/types'
import { useForm } from 'react-hook-form'

import { Card, CardContent, CardFooter } from '@/components/ui/card'
import { Form, FormControl, FormItem } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { TextArea } from '@/components/ui/textarea'

type TodoFormValues = {
  title: string
  description: string
}

type TodoFormProps = {
  handleSave: (title: string, description: string) => Promise<void>
  disabled?: boolean
  titleValue?: string
  descriptionValue?: string
  repeatingBorder?: boolean
}

const TodoForm: FC<TodoFormProps & PropsWithChildren> = ({
  handleSave,
  disabled,
  children,
  titleValue,
  descriptionValue,
  repeatingBorder,
}) => {
  const form = useForm<TodoFormValues>()

  const { register, reset, handleSubmit } = form

  const onSubmit = ({ title, description }: TodoFormValues) => {
    handleSave(title, description).then(() => reset())
  }

  return (
    <Form {...form}>
      <Card className={`relative w-80 ${repeatingBorder && 'border-sky-500'}`}>
        <form onSubmit={handleSubmit(onSubmit)}>
          <CardContent>
            <FormItem className="mb-6 mt-7">
              <FormControl>
                <Input
                  defaultValue={titleValue}
                  disabled={disabled}
                  placeholder="Title"
                  {...register('title')}
                />
              </FormControl>
            </FormItem>
            <FormItem>
              <FormControl>
                <TextArea
                  defaultValue={descriptionValue}
                  disabled={disabled}
                  placeholder="Description"
                  rows={10}
                  {...register('description')}
                />
              </FormControl>
            </FormItem>
          </CardContent>
          <CardFooter className="flex justify-between">{children}</CardFooter>
        </form>
      </Card>
    </Form>
  )
}

export default TodoForm
