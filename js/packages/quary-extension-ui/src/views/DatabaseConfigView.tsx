import { ConnectionConfig } from '@quary/proto/quary/service/v1/connection_config'
import { PageTitle } from '@ui/components/PageTitle'

interface Props {
  config?: ConnectionConfig
}

export const DatabaseConfigView: React.FC<Props> = ({ config }) => {
  if (!config) {
    return <div>Config is not set</div>
  }
  return (
    <div>
      <div className="pt-5">
        <PageTitle>Database Config</PageTitle>
      </div>
      <div className="pt-5">
        <pre>{JSON.stringify(config)}</pre>
      </div>
    </div>
  )
}
