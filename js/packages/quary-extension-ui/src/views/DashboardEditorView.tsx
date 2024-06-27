import Dashboard from '@ui/components/Dashboard/Dashboard'
import { DashboardEditorData } from '@shared/globalViewState'

interface Props {
  data: DashboardEditorData
}

export const DashboardEditorView: React.FC<Props> = ({ data }) => {
    console.log('dashboard editor view data', data)
    return (
        <div>
            <Dashboard dashboardItems={data.items} />
        </div>
    )
}
