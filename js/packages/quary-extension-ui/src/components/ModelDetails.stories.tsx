import { Meta, StoryObj } from '@storybook/react'
import { ModelDetails } from './ModelDetails'

const meta: Meta<typeof ModelDetails> = {
  component: ModelDetails,
}

export default meta

type Story = StoryObj<typeof ModelDetails>

const argTypes = {
  addColumn: { action: 'addColumn' },
  addColumnTest: { action: 'addColumnTest' },
  removeColumnTest: { action: 'removeColumnTest' },
  addDescription: { action: 'addDescription' },
}

const args = {
  table: {
    tableType: {
      $case: 'present',
      present: {
        rows: [
          {
            row: {
              $case: 'presentInSqlAndDefinitions',
              presentInSqlAndDefinitions: {
                title: 'Present in SQL and definitions',
                tests: [],
                description: {
                  description: {
                    $case: 'present',
                    present: 'This is just a present description',
                  },
                },
              },
            },
          },
          {
            row: {
              $case: 'presentInSqlAndDefinitions',
              presentInSqlAndDefinitions: {
                title: 'Present in SQL and definitions',
                tests: [],
                description: {
                  description: {
                    $case: 'presentAndInferredIdentical',
                    presentAndInferredIdentical:
                      "This is just a present description that's also inferred",
                  },
                },
              },
            },
          },
          {
            row: {
              $case: 'presentInSqlAndDefinitions',
              presentInSqlAndDefinitions: {
                title: 'Present in SQL and definitions',
                tests: [],
                description: {
                  description: {
                    $case: 'presentWithDifferentInference',
                    presentWithDifferentInference: {
                      present: "This is the description that's present",
                      inferred: "This is the description that's inferred",
                    },
                  },
                },
              },
            },
          },
          {
            row: {
              $case: 'presentInSqlAndDefinitions',
              presentInSqlAndDefinitions: {
                title: 'Present in SQL and definitions',
                tests: [],
                description: {
                  description: {
                    $case: 'inferred',
                    inferred: 'This is just an inferred description',
                  },
                },
              },
            },
          },
          {
            row: {
              $case: 'presentInSqlAndDefinitions',
              presentInSqlAndDefinitions: {
                title: 'Present in SQL and definitions',
                tests: [],
                description: {
                  description: {
                    $case: 'notPresent',
                    notPresent: {},
                  },
                },
              },
            },
          },
          {
            row: {
              $case: 'presentInDefinitionsButNotRecognisableInSql',
              presentInDefinitionsButNotRecognisableInSql: {
                title: 'Present in definition but not recognisable in SQL',
                tests: [],
                description: {
                  description: {
                    $case: 'notPresent',
                    notPresent: {},
                  },
                },
              },
            },
          },
          {
            row: {
              $case: 'presentInDefinitionsButNotRecognisableInSql',
              presentInDefinitionsButNotRecognisableInSql: {
                title: 'Present in definition but not recognisable in SQL',
                tests: [],
                description: {
                  description: {
                    $case: 'present',
                    present: 'This is just a present description',
                  },
                },
              },
            },
          },
          {
            row: {
              $case: 'presentInDefinitionsButNotRecognisableInSql',
              presentInDefinitionsButNotRecognisableInSql: {
                title: 'Present in definition but not recognisable in SQL',
                tests: [
                  {
                    test: {
                      $case: 'presentAndNotInferred',
                      presentAndNotInferred: {
                        text: 'Present and not Inferred',
                        columnTest: {
                          type: 'not_null',
                        },
                      },
                    },
                  },
                  {
                    test: {
                      $case: 'presentAndNotInferred',
                      presentAndNotInferred: {
                        text: 'Present and not Inferred',
                        columnTest: {
                          type: 'not_null',
                        },
                      },
                    },
                  },
                  {
                    test: {
                      $case: 'presentAndNotInferred',
                      presentAndNotInferred: {
                        text: 'Present and not Inferred',
                        columnTest: {
                          type: 'not_null',
                        },
                      },
                    },
                  },
                  {
                    test: {
                      $case: 'presentAndNotInferred',
                      presentAndNotInferred: {
                        text: 'Present and not Inferred',
                        columnTest: {
                          type: 'not_null',
                        },
                      },
                    },
                  },
                  {
                    test: {
                      $case: 'presentAndInferred',
                      presentAndInferred: {
                        text: 'Present and Inferred',
                        columnTest: {
                          type: 'not_null',
                        },
                      },
                    },
                  },
                  {
                    test: {
                      $case: 'notPresentButInferred',
                      notPresentButInferred: {
                        text: 'Not present but inferred',
                        columnTest: {
                          type: 'not_null',
                        },
                      },
                    },
                  },
                ],
                description: {
                  description: {
                    $case: 'present',
                    present: 'This is just a present description',
                  },
                },
              },
            },
          },
          {
            row: {
              $case: 'missingInDefinitions',
              missingInDefinitions: {
                title:
                  'Present in definition but not recognisable in SQL with super long title that should wrap.',
                tests: [
                  {
                    test: {
                      $case: 'presentAndNotInferred',
                      presentAndNotInferred: {
                        text: 'Present and not inferred',
                        columnTest: {
                          type: 'not_null',
                        },
                      },
                    },
                  },
                  {
                    test: {
                      $case: 'presentAndInferred',
                      presentAndInferred: {
                        text: 'Present and inferred',
                        columnTest: {
                          type: 'not_null',
                        },
                      },
                    },
                  },
                  {
                    test: {
                      $case: 'notPresentButInferred',
                      notPresentButInferred: {
                        text: 'Not present but inferred',
                        columnTest: {
                          type: 'not_null',
                        },
                      },
                    },
                  },
                ],
                description: {
                  description: {
                    $case: 'present',
                    present: 'This is just a present description',
                  },
                },
              },
            },
          },
        ],
      },
    },
  },
}

export const PresentInDefinitions: Story = {
  argTypes,
  args,
}

export const NotPresent: Story = {
  argTypes,
  args: {
    table: {
      tableType: {
        $case: 'notPresent',
        notPresent: {
          rows: [
            {
              title: 'Present in SQL and definitions',
              tests: [],
              description: {
                description: {
                  $case: 'present',
                  present: 'This is just a present description',
                },
              },
            },
            {
              title: 'Present in SQL and definitions',
              tests: [],
              description: {
                description: {
                  $case: 'presentAndInferredIdentical',
                  presentAndInferredIdentical:
                    "This is just a present description that's also inferred",
                },
              },
            },
            {
              title: 'Present in SQL and definitions',
              tests: [],
              description: {
                description: {
                  $case: 'presentWithDifferentInference',
                  presentWithDifferentInference: {
                    present: "This is the description that's present",
                    inferred: "This is the description that's inferred",
                  },
                },
              },
            },
            {
              title: 'Present in SQL and definitions',
              tests: [],
              description: {
                description: {
                  $case: 'inferred',
                  inferred: 'This is just an inferred description',
                },
              },
            },
            {
              title: 'Present in SQL and definitions',
              tests: [],
              description: {
                description: {
                  $case: 'notPresent',
                  notPresent: {},
                },
              },
            },
            {
              title: 'Present in definition but not recognisable in SQL',
              tests: [],
              description: {
                description: {
                  $case: 'notPresent',
                  notPresent: {},
                },
              },
            },
            {
              title: 'Present in definition but not recognisable in SQL',
              tests: [],
              description: {
                description: {
                  $case: 'present',
                  present: 'This is just a present description',
                },
              },
            },
            {
              title: 'Present in definition but not recognisable in SQL',
              tests: [
                {
                  test: {
                    $case: 'presentAndNotInferred',
                    presentAndNotInferred: 'Present and not inferred',
                  },
                },
                {
                  test: {
                    $case: 'presentAndInferred',
                    presentAndInferred: 'Present and inferred',
                  },
                },
                {
                  test: {
                    $case: 'notPresentButInferred',
                    notPresentButInferred: {
                      text: 'Not present but inferred',
                      columnTest: {
                        type: 'not_null',
                      },
                    },
                  },
                },
              ],
              description: {
                description: {
                  $case: 'present',
                  present: 'This is just a present description',
                },
              },
            },
          ],
        },
      },
    },
  },
}

export const CallbacksNotPresent: Story = {
  args,
}
