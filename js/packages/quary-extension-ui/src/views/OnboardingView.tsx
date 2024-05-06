import React, { useState, useEffect } from 'react'
import {
  DatabaseOnboardingOptions,
  OnboardingViewStates,
} from '@shared/globalViewState'
import { ConnectionConfig } from '@quary/proto/quary/service/v1/connection_config'
import { Callbacks, useCallBackFrontEnd } from '@shared/callBacks'
import { Button } from '@/components/ui/button'
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectItem,
  SelectContent,
} from '@/components/ui/select'
import { Input } from '@/components/ui/input'
import { Card } from '@/components/Card'
import { vscode } from '@/utils/VSCodeAPIWrapper'
import { Warning } from '@/components/Warning'
import { ProgressRing } from '@/components/ProgressRing'
import { Label } from '@/components/ui/label'

interface Props {
  states: OnboardingViewStates
}

export const OnboardingView: React.FC<Props> = ({ states }) => {
  const {
    onboardingViewSelectDatabase,
    onboardingViewRestartFlow,
    onboardingViewGenerateProject,
  } = useCallBackFrontEnd(
    [
      'onboardingViewSelectDatabase',
      'onboardingViewRestartFlow',
      'onboardingViewGenerateProject',
    ],
    vscode.postMessage,
  )

  const Step: React.FC = () => {
    switch (states.type) {
      case 'init':
        return (
          <DatabaseSelection
            onSelectDatabase={onboardingViewSelectDatabase}
            onGenerateProject={onboardingViewGenerateProject}
          />
        )
      case 'listSourcesLoading':
        return (
          <div className="flex items-center justify-center">
            <ProgressRing />
          </div>
        )
      case 'generateProjectError':
        return (
          <>
            <Warning title="An error occured">{states.error}</Warning>
            <div className="flex justify-between pt-4">
              <Button size="sm" onClick={() => onboardingViewRestartFlow(null)}>
                Back
              </Button>
            </div>
          </>
        )
      case 'listSourcesError':
        return (
          <>
            <Warning title="An error occured">{states.error}</Warning>
            <div className="flex justify-between pt-4">
              <Button size="sm" onClick={() => onboardingViewRestartFlow(null)}>
                Back
              </Button>
            </div>
          </>
        )
      case 'listSourcesSuccess':
        return (
          <ConfigurationDetails
            onGenerateProject={onboardingViewGenerateProject}
            onRestartOnboardingFlow={onboardingViewRestartFlow}
            states={states}
          />
        )
      default:
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-expect-error
        throw Error(`invalid state type ${states.type}`)
    }
  }

  return (
    <Card title="Project Configuration">
      <Step />
    </Card>
  )
}

const DatabaseSelection: React.FC<{
  onSelectDatabase: (message: Callbacks['onboardingViewSelectDatabase']) => void
  onGenerateProject: (
    message: Callbacks['onboardingViewGenerateProject'],
  ) => void
}> = ({ onSelectDatabase, onGenerateProject }) => {
  const [database, setDatabase] = useState<DatabaseOnboardingOptions>(
    DatabaseOnboardingOptions.BigQuery,
  )
  const [sqlitePath, setSqlitePath] = useState<string>('db.sqlite')
  const [snowflakeAccountUrl, setSnowflakeAccountUrl] = useState<string>('')
  const [snowflakeClientId, setSnowflakeClientId] = useState<string>('')
  const [snowflakeWarehouse, setSnowflakeWarehouse] = useState<string>('')
  const [snowflakeClientSecret, setSnowflakeClientSecret] = useState<string>('')
  const [snowflakeRole, setSnowflakeRole] = useState<string>('')

  return (
    <>
      <h1>Connection</h1>
      <div className="box-border flex flex-col items-start justify-start">
        <Select
          value={database}
          onValueChange={(db) => {
            setDatabase(db as DatabaseOnboardingOptions)
          }}
        >
          <SelectTrigger className="w-full">
            <SelectValue placeholder="Select a database" />
          </SelectTrigger>

          <SelectContent>
            {Object.keys(DatabaseOnboardingOptions).map((key) => (
              <SelectItem
                key={key}
                value={
                  DatabaseOnboardingOptions[
                    key as keyof typeof DatabaseOnboardingOptions
                  ]
                }
              >
                {key}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        {database === DatabaseOnboardingOptions.SQLite && (
          <>
            <h1 className="pt-4">Local Database File</h1>
            <Label htmlFor="sqlitePath" className="py-2">
              Enter the path to your SQLite database file
            </Label>
            <Input
              id="sqlitePath"
              value={sqlitePath}
              onInput={(e) =>
                setSqlitePath((e.target as HTMLInputElement).value)
              }
              placeholder="i.e. database.db"
              className="w-full"
            />
          </>
        )}

        {database === DatabaseOnboardingOptions.Snowflake && (
          <>
            <h1 className="pt-4">Account URL</h1>
            <Label htmlFor="snowflakeAccountUrl" className="py-2">
              Enter your snowflake account URL
            </Label>
            <Input
              id="snowflakeAccountUrl"
              value={snowflakeAccountUrl}
              onChange={(e) => {
                setSnowflakeAccountUrl(e.target.value)
              }}
              placeholder="i.e. https://<your_account>.snowflakecomputing.com"
              className="w-full"
            />
            <h1 className="pt-4">Client ID</h1>
            <Label htmlFor="snowflakeClientId" className="py-2">
              Enter your snowflake Client ID
            </Label>
            <Input
              id="snowflakeClientId"
              value={snowflakeClientId}
              onChange={(e) => {
                setSnowflakeClientId(e.target.value)
              }}
              placeholder="Client Id"
              className="w-full"
            />
            <h1 className="pt-4">Client Secret</h1>
            <Label htmlFor="snowflakeClientSecret" className="py-2">
              Enter your snowflake Client Secret
            </Label>
            <Input
              id="snowflakeClientSecret"
              value={snowflakeClientSecret}
              onChange={(e) => {
                setSnowflakeClientSecret(e.target.value)
              }}
              placeholder="Client Secret"
              className="w-full"
            />
            <h1 className="pt-4">Role</h1>
            <Label htmlFor="snowflakeRole" className="py-2">
              Enter your snowflake Role
            </Label>
            <Input
              id="snowflakeRole"
              value={snowflakeRole}
              onChange={(e) => {
                setSnowflakeRole(e.target.value)
              }}
              placeholder="i.e. QUARY_ROLE"
              className="w-full"
            />
            <h1 className="pt-4">Warehouse</h1>
            <Label htmlFor="snowflakeWarehouse" className="py-2">
              Enter your snowflake default execution Warehouse
            </Label>
            <Input
              id="snowflakeWarehouse"
              value={snowflakeWarehouse}
              onChange={(e) => {
                setSnowflakeWarehouse(e.target.value)
              }}
              placeholder="i.e. QUARY_WAREHOUSE"
              className="w-full"
            />
          </>
        )}
      </div>
      <div className="flex justify-between pt-4">
        <Button
          size="sm"
          onClick={() => {
            switch (database) {
              case DatabaseOnboardingOptions.BigQuery: {
                onSelectDatabase({
                  type: DatabaseOnboardingOptions.BigQuery,
                })
                break
              }
              case DatabaseOnboardingOptions.SQLite: {
                onSelectDatabase({
                  type: DatabaseOnboardingOptions.SQLite,
                  path: sqlitePath,
                })
                break
              }
              case DatabaseOnboardingOptions.SQLiteInMemory: {
                onGenerateProject({
                  config: {
                    $case: 'sqliteInMemory',
                    sqliteInMemory: {},
                  },
                  vars: [],
                })
                break
              }
              case DatabaseOnboardingOptions.Snowflake: {
                onSelectDatabase({
                  type: DatabaseOnboardingOptions.Snowflake,
                  accountUrl: snowflakeAccountUrl,
                  clientId: snowflakeClientId,
                  clientSecret: snowflakeClientSecret,
                  warehouse: snowflakeWarehouse,
                  role: snowflakeRole,
                })
                break
              }

              default: {
                break
              }
            }
          }}
        >
          Next
        </Button>
      </div>
    </>
  )
}

interface ConfigurationDetailsProps {
  states: Extract<OnboardingViewStates, { type: 'listSourcesSuccess' }>
  onGenerateProject: (
    message: Callbacks['onboardingViewGenerateProject'],
  ) => void
  onRestartOnboardingFlow: (
    message: Callbacks['onboardingViewRestartFlow'],
  ) => void
}

const ConfigurationDetails: React.FC<ConfigurationDetailsProps> = ({
  states,
  onGenerateProject,
  onRestartOnboardingFlow,
}) => {
  // target options (bigquery)
  const [selectedProject, setSelectedProject] = useState<string>('')
  const [selectedDataset, setSelectedDataset] = useState<string>('')
  // target options (snowflake)
  const [selectedDatabase, setSelectedDatabase] = useState<string>('')
  const [selectedSchema, setSelectedSchema] = useState<string>('')

  const getConfig = (): ConnectionConfig => {
    switch (states.sourceDetails.type) {
      case DatabaseOnboardingOptions.BigQuery:
        return {
          config: {
            $case: 'bigQuery',
            bigQuery: {
              projectId: selectedProject,
              datasetId: selectedDataset,
            },
          },
          vars: [],
        }
      case DatabaseOnboardingOptions.SQLite:
        return {
          config: {
            $case: 'sqlite',
            sqlite: {
              path: states.sourceDetails.path,
            },
          },
          vars: [],
        }
      case DatabaseOnboardingOptions.Snowflake:
        return {
          config: {
            $case: 'snowflake',
            snowflake: {
              database: selectedDatabase,
              schema: selectedSchema,
              ...states.sourceDetails.config,
            },
          },
          vars: [],
        }
      default: {
        throw new Error(`invalid database type ${states.sourceDetails.type}`)
      }
    }
  }

  return (
    <div>
      {states.sourceDetails.type === DatabaseOnboardingOptions.Snowflake && (
        <TargetSelection
          setPrimaryObject={setSelectedDatabase}
          setSecondaryObject={setSelectedSchema}
          primaryObject={selectedDatabase}
          secondaryObject={selectedSchema}
          primaryObjectName="Database"
          secondaryObjectName="Schema"
          availableObjects={states.sourceDetails.databasesAndSchemas}
        />
      )}
      {states.sourceDetails.type === DatabaseOnboardingOptions.BigQuery && (
        <TargetSelection
          setPrimaryObject={setSelectedProject}
          setSecondaryObject={setSelectedDataset}
          primaryObject={selectedProject}
          secondaryObject={selectedDataset}
          primaryObjectName="Project"
          secondaryObjectName="Dataset"
          availableObjects={states.sourceDetails.projectsAndDatasets}
        />
      )}
      {states.sourceDetails.type === DatabaseOnboardingOptions.SQLite && (
        <>
          <h1>Configuration</h1>
          <hr />
          <div className="pt-3">
            <h1 className="text-sm">
              Path: <strong>{states.sourceDetails.path}</strong>
            </h1>
          </div>
        </>
      )}

      <div className="flex justify-between pt-4">
        <Button size="sm" onClick={() => onRestartOnboardingFlow(null)}>
          Previous
        </Button>
        <Button
          size="sm"
          onClick={() => {
            const config = getConfig()
            onGenerateProject(config)
          }}
        >
          Create Project
        </Button>
      </div>
    </div>
  )
}

interface TargetSelectionProps {
  setPrimaryObject: (e: string) => void // i.e. Database (Snowflake)/Project(BigQuery)
  primaryObject: string
  primaryObjectName: string
  setSecondaryObject: (e: string) => void // i.e. Schema (Snowflake)/Dataset(BigQuery)
  secondaryObject: string
  secondaryObjectName: string
  availableObjects: Record<string, string[]>
}

const TargetSelection: React.FC<TargetSelectionProps> = ({
  setPrimaryObject,
  primaryObject,
  primaryObjectName,
  setSecondaryObject,
  secondaryObject,
  secondaryObjectName,
  availableObjects,
}) => {
  const availablePrimaryObjects = Object.keys(availableObjects) || []
  const availableSecondaryObjects = availableObjects[primaryObject] || []

  useEffect(() => {
    if (availableObjects) {
      const initalPrimaryObject = Object.keys(availableObjects)[0]
      setPrimaryObject(initalPrimaryObject || '')
      setSecondaryObject(availableObjects[initalPrimaryObject][0] || '')
    }
  }, [availableObjects, setPrimaryObject, setSecondaryObject])
  return (
    <>
      <h1>Configuration</h1>
      <hr />
      <div className="pt-3">
        <h1 className="text-sm">{primaryObjectName}</h1>
        <div className="box-border flex flex-col items-start justify-start">
          <Select
            value={primaryObject}
            onValueChange={(primaryObject) => {
              setPrimaryObject(primaryObject)
              setSecondaryObject(availableObjects[primaryObject]?.[0] || '')
            }}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select a database" />
            </SelectTrigger>

            <SelectContent>
              {availablePrimaryObjects.map((primaryObjectId) => (
                <SelectItem key={primaryObjectId} value={primaryObjectId}>
                  {primaryObjectId}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      </div>
      <div className="pt-3">
        <h1 className="text-sm">{secondaryObjectName}</h1>
        <div className="box-border flex flex-col items-start justify-start">
          <Select
            value={secondaryObject}
            onValueChange={(secondaryObject) => {
              setSecondaryObject(secondaryObject)
            }}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select a schema" />
            </SelectTrigger>

            <SelectContent>
              {availableSecondaryObjects.map((secondaryObjectId) => (
                <SelectItem key={secondaryObjectId} value={secondaryObjectId}>
                  {secondaryObjectId}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      </div>
    </>
  )
}
