import { Project } from '@quary/proto/quary/service/v1/project'

interface Props {
  project: Project
  seedQueries: string[]
}

export const FullProjectView: React.FC<Props> = ({ project, seedQueries }) => {
  const renderProjectTable = (project: Project) => (
    <table>
      <thead>
        <tr>
          <th>Field</th>
          <th>Value</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Models</td>
          <td>{JSON.stringify(project.models)}</td>
        </tr>
        <tr>
          <td>Seeds</td>
          <td>{JSON.stringify(project.seeds)}</td>
        </tr>
      </tbody>
    </table>
  )

  const renderSeedQueries = (queries: string[]) => (
    <table>
      <thead>
        <tr>
          <th>Query</th>
        </tr>
      </thead>
      <tbody>
        {queries.map((query) => (
          <tr key={query}>
            <td>{query}</td>
          </tr>
        ))}
      </tbody>
    </table>
  )

  return (
    <body>
      <h2>Project Config</h2>
      <div>{renderProjectTable(project)}</div>
      <div>
        <h2>Seed Queries</h2>
        <div>{renderSeedQueries(seedQueries)}</div>
      </div>
    </body>
  )
}
