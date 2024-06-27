import { useCallBackFrontEnd } from '@shared/callBacks'
import { TablesView } from '@ui/views/TablesView'
import { TestReportView } from '@ui/views/TestReportView'
import { FullProjectView } from '@ui/views/FullProjectView'
import { FullSchemaView } from '@ui/views/FullSchemaView'
import { ResultsView } from '@ui/views/ResultsView'
import { DatabaseConfigView } from '@ui/views/DatabaseConfigView'
import { ProjectFileView } from '@ui/views/ProjectFileView'
import { LoadingView } from '@ui/views/LoadingView'
import { vscode } from '@ui/utils/VSCodeAPIWrapper'
import { DocumentationView } from '@ui/views/DocumentationView'
import { OnboardingView } from '@ui/views/OnboardingView'
import { useGlobalState } from '@ui/hooks/useGlobalState'
import '@ui/index.css'
import { ErrorView } from '@ui/views/ErrorView'
import { ExecuteSQLView } from '@ui/views/ExecuteSQL'
import { ImportSourcesView } from '@ui/views/ImportSourcesView'
import { ChartEditorView } from '@ui/views/ChartEditorView'
import { DashboardEditorView } from '@ui/views/DashboardEditorView'

function App() {
  const [view] = useGlobalState()
  const { importSources } = useCallBackFrontEnd(
    ['importSources'],
    vscode.postMessage,
  )

  console.log('received view', view)

  switch (view.type) {
    case 'loading':
      return <LoadingView />
    case 'error':
      return <ErrorView error={view.error} />
    case 'tests': {
      const { tests, runner } = view
      return <TestReportView tests={tests} testRunner={runner} />
    }
    case 'onboarding': {
      const { states } = view
      return <OnboardingView states={states} />
    }
    case 'schema': {
      const { fullSchema, language } = view
      return <FullSchemaView fullSchema={fullSchema} language={language} />
    }
    case 'queryResults': {
      const { results, originalSql, limit, language } = view
      return (
        <ResultsView
          results={results}
          originalSql={originalSql}
          limit={limit}
          language={language}
        />
      )
    }
    case 'project': {
      const { project, seedQueries } = view
      return <FullProjectView project={project} seedQueries={seedQueries} />
    }
    case 'tables': {
      const { tables, views } = view
      return (
        <div>
          <TablesView tables={tables} views={views} />
        </div>
      )
    }
    case 'databaseConfigShow': {
      return <DatabaseConfigView config={view.config} />
    }
    case 'databaseShowProjectFile': {
      return <ProjectFileView file={view.projectFile} />
    }
    case 'importSources': {
      return (
        <ImportSourcesView
          state={view.state}
          onSelectSources={(sources, folderPath) =>
            importSources({ sources, folderPath })
          }
        />
      )
    }
    case 'docsView': {
      const {
        results,
        tags,
        description,
        modelName,
        limit,
        dag,
        table,
        isModelInSchema,
      } = view
      return (
        <DocumentationView
          modelName={modelName}
          description={description || undefined}
          results={results}
          table={table}
          tags={tags}
          limit={limit}
          dag={dag}
          hideCreateSchemaButton={isModelInSchema}
        />
      )
    }
    case 'executeSQL': {
      return <ExecuteSQLView results={view.results} limit={view.limit} />
    }
    case 'chartEditor': {
      return <ChartEditorView data={view.data} />
    }
    case 'dashboardEditor': {
      return <DashboardEditorView data={view.data} />
    }
    default:
      return <div>Invalid view type {JSON.stringify(view)}</div>
  }
}

export default App
