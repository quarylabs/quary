import { useCallBackFrontEnd } from '@shared/callBacks'
import { TablesView } from '@/views/TablesView'
import { TestReportView } from '@/views/TestReportView'
import { FullProjectView } from '@/views/FullProjectView'
import { FullSchemaView } from '@/views/FullSchemaView'
import { ResultsView } from '@/views/ResultsView'
import { DatabaseConfigView } from '@/views/DatabaseConfigView'
import { ProjectFileView } from '@/views/ProjectFileView'
import { LoadingView } from '@/views/LoadingView'
import { AIView } from '@/views/AIView'
import { vscode } from '@/utils/VSCodeAPIWrapper'
import { DocumentationView } from '@/views/DocumentationView'
import { OnboardingView } from '@/views/OnboardingView'
import { useGlobalState } from '@/hooks/useGlobalState'
import '@/index.css'
import { ErrorView } from '@/views/ErrorView'
import { ExecuteSQLView } from '@/views/ExecuteSQL'
import { ImportSourcesView } from '@/views/ImportSourcesView'

function App() {
  const [view] = useGlobalState()
  const { createFile, importSources } = useCallBackFrontEnd(
    ['createFile', 'importSources'],
    vscode.postMessage,
  )

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
    case 'aiGeneratedQuery': {
      return (
        <AIView
          onClickCreateFile={(content: string) =>
            createFile({ content, language: 'sql' })
          }
          {...view}
        />
      )
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
    case 'sqlDocumentation': {
      const { results, tags, documentation, modelName, limit, dag, table } =
        view
      return (
        <DocumentationView
          modelName={modelName}
          description={documentation || undefined}
          results={results}
          table={table}
          tags={tags}
          limit={limit}
          dag={dag}
        />
      )
    }
    case 'executeSQL': {
      return <ExecuteSQLView results={view.results} limit={view.limit} />
    }

    default:
      return <div>Invalid view type {JSON.stringify(view)}</div>
  }
}

export default App
