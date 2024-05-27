import { useState } from 'react'
import {
  PlusCircledIcon,
  CaretSortIcon,
  CheckIcon,
} from '@radix-ui/react-icons'
import { ColumnTest } from '@quary/proto/quary/service/v1/project_file'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogTrigger,
} from '@/components/ui/dialog'
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from '@/components/ui/command'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { cn } from '@/lib/utils'

// TODO: get available tests from rust/proto
const POSSIBLE_TESTS = [
  {
    type: 'unique',
    description: 'Asserts the column contains only unique values',
  },
  {
    type: 'not_null',
    description: 'Asserts the column does not contain any null values',
  },
  {
    type: 'gt',
    description: 'Asserts the column is greater than the specified value',
    details: [
      {
        name: 'Value',
        value: 'value',
        placeholder: 'Enter a value for comparison',
        inputType: 'text',
      },
    ],
  },
  {
    type: 'gte',
    description:
      'Asserts the column is greater than or equal to the specified value',
    details: [
      {
        name: 'Value',
        value: 'value',
        placeholder: 'Enter a value for comparison',
        inputType: 'text',
      },
    ],
  },
  {
    type: 'lt',
    description: 'Asserts the column is less than the specified value',
    details: [
      {
        name: 'Value',
        value: 'value',
        placeholder: 'Enter a value for comparison',
        inputType: 'text',
      },
    ],
  },
  {
    type: 'lte',
    description:
      'Asserts the column is less than or equal to the specified value',
    details: [
      {
        name: 'Value',
        value: 'value',
        placeholder: 'Enter a value for comparison',
        inputType: 'text',
      },
    ],
  },
  {
    type: 'relationship',
    description: 'Defines a relationship between the column and another',
    details: [
      {
        name: 'Model',
        value: 'model',
        placeholder: 'Enter the model name',
        inputType: 'text',
      },
      {
        name: 'Column',
        value: 'column',
        placeholder: 'Enter the column name',
        inputType: 'text',
      },
    ],
  },
  {
    type: 'accepted_values',
    description: 'Asserts the column is in a particular set of accepted values',
    details: [
      {
        name: 'Values',
        value: 'values',
        placeholder: 'Comma seperated values i.e. cat,dog,bird',
        inputType: 'text',
      },
    ],
  },
]

export const AddTestDialog: React.FC<{
  addColumnTest: (columnTest: ColumnTest) => void
}> = ({ addColumnTest }) => {
  const [selectedTest, setSelectedTest] = useState(POSSIBLE_TESTS[0])
  const [testDetails, setTestDetails] = useState({})
  const [testSelectOpen, setTestSelectOpen] = useState<boolean>(false)

  const handleTestDetailsChange = (inputName: string, value: string) => {
    setTestDetails({ ...testDetails, [inputName]: value })
  }

  return (
    <Dialog>
      <DialogTrigger>
        <Badge variant="secondary">
          <div className="flex items-center gap-1">
            <PlusCircledIcon className="h-3 w-3" />
            Add
          </div>
        </Badge>
      </DialogTrigger>
      <DialogContent>
        <Label>Test type</Label>
        <Popover open={testSelectOpen} onOpenChange={setTestSelectOpen} modal>
          <PopoverTrigger asChild>
            <Button
              variant="outline"
              role="combobox"
              className="w-[300px] justify-between"
            >
              {selectedTest.type
                ? POSSIBLE_TESTS.find((test) => test.type === selectedTest.type)
                    ?.type
                : 'Select a test'}
              <CaretSortIcon className="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
          </PopoverTrigger>
          <PopoverContent className="w-[300px] p-0">
            <Command>
              <CommandInput placeholder="Search tests..." className="h-9" />
              <CommandEmpty>No tests found.</CommandEmpty>
              <CommandGroup className="max-h-[300px] overflow-y-scroll">
                {POSSIBLE_TESTS.map((test) => (
                  <CommandItem
                    key={test.type}
                    value={test.type}
                    onSelect={(value) => {
                      setTestDetails({})
                      const selected = POSSIBLE_TESTS.find(
                        ({ type }) => value === type,
                      )
                      if (selected) {
                        setSelectedTest(selected)
                      }
                      setTestSelectOpen(false)
                    }}
                  >
                    <div>
                      {test.type}
                      <>
                        <p className="text-muted-foreground flex text-xs">
                          {test.description}
                        </p>
                      </>
                    </div>

                    <CheckIcon
                      className={cn(
                        'ml-auto h-4 w-4',
                        selectedTest.type === test.type
                          ? 'opacity-100'
                          : 'opacity-0',
                      )}
                    />
                  </CommandItem>
                ))}
              </CommandGroup>
            </Command>
          </PopoverContent>
        </Popover>
        {selectedTest.details &&
          selectedTest.details.map((input) => (
            <>
              <Label>{input.name}</Label>
              <Input
                placeholder={input.placeholder}
                type={input.inputType}
                onChange={(e) =>
                  handleTestDetailsChange(input.value, e.target.value)
                }
              />
            </>
          ))}
        <DialogFooter>
          <DialogClose asChild>
            <Button
              onClick={() => {
                addColumnTest(
                  ColumnTest.create({
                    type: selectedTest.type,
                    info: testDetails,
                  }),
                )
              }}
            >
              Add Test
            </Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
