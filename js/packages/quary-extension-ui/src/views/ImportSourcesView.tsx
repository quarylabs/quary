import { useMemo, useState } from 'react'
import { ImportSourcesViewState } from '@shared/globalViewState'
import { VSCodeBadge } from '@vscode/webview-ui-toolkit/react'
import Fuse from 'fuse.js'
import { ProjectFileSource } from '@quary/proto/quary/service/v1/project_file'
import { ProgressRing } from '@ui/components/ProgressRing'
import { Warning } from '@ui/components/Warning'
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '@ui/components/ui/card'
import { Input } from '@ui/components/ui/input'
import { Separator } from '@ui/components/ui/separator'
import { Label } from '@ui/components/ui/label'
import { Button } from '@ui/components/ui/button'
import { Checkbox } from '@ui/components/ui/checkbox'

interface Props {
  state: ImportSourcesViewState
  onSelectSources: (sources: ProjectFileSource[], folderPath: string) => void
}

export const ImportSourcesView: React.FC<Props> = ({
  state,
  onSelectSources,
}) => {
  const State: React.FC = () => {
    switch (state.type) {
      case 'loading':
        return (
          <div className="flex items-center justify-center">
            <ProgressRing className="h-10 w-10" />
          </div>
        )
      case 'error': {
        const error = state.error
        return <Warning title="Error loading sources">{error}</Warning>
      }
      case 'success': {
        if (state.sources.length === 0) {
          return (
            <Warning title="No sources found">
              No sources found in the database. Make sure you have tables with
              data in the database.
            </Warning>
          )
        }
        return (
          <SourceSelector
            onSelectSources={onSelectSources}
            sources={state.sources}
          />
        )
      }
      default:
        throw Error(`Unknown state ${state}`)
    }
  }
  return (
    <div className="flex h-screen items-center justify-center">
      <Card className="sm:w-96">
        <CardHeader>
          <CardTitle>Import Sources</CardTitle>
        </CardHeader>
        <CardContent>
          <State />
        </CardContent>
      </Card>
    </div>
  )
}

type Source = ProjectFileSource

const SourceSelector: React.FC<{
  sources: Source[]
  onSelectSources: (sources: Source[], folderLocation: string) => void
}> = ({ sources, onSelectSources }) => {
  const [selectedSources, setSelectedSources] = useState<Source[]>([])
  const [sourcesSearchText, setSourcesSearchText] = useState<string>('')
  const [folderLocation, setFolderLocation] = useState<string>('models/staging')

  const filteredSources = useMemo(() => {
    const fuse = new Fuse(sources, {
      keys: ['name', 'path'],
    })
    if (!sourcesSearchText.trim()) {
      return sources // return all sources if search text is empty
    }
    return fuse.search(sourcesSearchText).map((result) => result.item)
  }, [sources, sourcesSearchText])

  const groupedSourcesByPath = useMemo(() => {
    const groupedTables: Record<string, typeof sources> = {}
    const sortedSources = [...filteredSources].sort((a, b) =>
      a.name.localeCompare(b.name),
    )
    sortedSources.forEach((source) => {
      const pathParts = source.path.split('.')
      pathParts.pop()
      const pathKey = pathParts.join('.')
      if (!groupedTables[pathKey]) {
        groupedTables[pathKey] = []
      }
      groupedTables[pathKey].push(source)
    })
    const sortedGroupKeys = Object.keys(groupedTables).sort()
    const sortedGroups: Record<string, typeof sources> = {}
    sortedGroupKeys.forEach((key) => {
      sortedGroups[key] = groupedTables[key]
    })
    return sortedGroups
  }, [filteredSources])

  const handleSourceSelection = (
    checked: string | boolean,
    selectedSource: Source,
  ) => {
    setSelectedSources((prevSelectedSources) => {
      if (checked) {
        return [...prevSelectedSources, selectedSource]
      }
      return prevSelectedSources.filter(
        (source) => source.path !== selectedSource.path,
      )
    })
  }

  return (
    <div>
      <Label>Sources location</Label>
      <Input
        onChange={(e) => setFolderLocation(e.target.value)}
        value={folderLocation}
      />
      <div className="py-3">
        <Separator />
      </div>
      <div className="pb-3">
        <Input
          placeholder="Search"
          value={sourcesSearchText}
          onChange={(e) => setSourcesSearchText(e.target.value)}
        />
      </div>
      <div className="max-h-[50vh] overflow-auto pb-3">
        {Object.keys(groupedSourcesByPath).map((path) => (
          <div key={path} className="mb-4">
            <p className="text-vscode-editor-foreground mb-2 font-semibold">
              {path}
            </p>
            <div className="border-l border-gray-200 pl-4">
              {groupedSourcesByPath[path].length > 0 ? (
                groupedSourcesByPath[path].map((source) => (
                  <div key={source.path} className="mt-2">
                    <ul className="ml-2 list-none" key={source.path}>
                      <li>
                        <div className="flex gap-1">
                          <Checkbox
                            id={source.path}
                            checked={selectedSources.some(
                              (s) => s.path === source.path,
                            )}
                            onCheckedChange={(checked) =>
                              handleSourceSelection(checked, source)
                            }
                          />
                          <label
                            htmlFor={source.path}
                            className="text-sm leading-none"
                          >
                            {source.name}
                          </label>
                        </div>
                      </li>
                    </ul>
                  </div>
                ))
              ) : (
                <VSCodeBadge className="ml-2">No sources found</VSCodeBadge>
              )}
            </div>
          </div>
        ))}
      </div>
      <div className="flex flex-col">
        <Button
          onClick={() => onSelectSources(selectedSources, folderLocation)}
        >
          Import sources
        </Button>
      </div>
    </div>
  )
}
