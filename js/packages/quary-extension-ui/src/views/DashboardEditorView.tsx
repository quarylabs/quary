import Dashboard from '@ui/components/Dashboard/Dashboard'
import { DashboardEditorData } from '@shared/globalViewState'

interface Props {
  data: DashboardEditorData
}

export const DashboardEditorView: React.FC<Props> = ({ data }) => (
  <div>
    <Dashboard dashboardItems={data.items} />
  </div>
)
