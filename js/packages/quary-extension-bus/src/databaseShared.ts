import { isErr, Ok, Result } from './result'
import { TableAddress } from '@quary/proto/quary/service/v1/table_address'
import {
  CacheViewInformation,
  CacheViewInformationPaths,
} from '@quary/proto/quary/service/v1/wasm_rust_rpc_calls'
import { Empty } from '@quary/proto/google/protobuf/empty'
import { DatabaseDependentSettings } from './config'

export const cacheViewBuilder = async (
  databaseDependentSettings: DatabaseDependentSettings,
  listViews: () => Promise<Result<Array<TableAddress>>>,
): Promise<Result<CacheViewInformation>> => {
  if (databaseDependentSettings.lookForCacheViews) {
    const tables = await listViews()
    if (isErr(tables)) {
      return tables
    }
    return Ok({
      cacheView: {
        $case: 'cacheViewInformation',
        cacheViewInformation: CacheViewInformationPaths.create({
          cacheViewPaths: tables.value.map((table) => table.fullPath),
        }),
      },
    })
  }
  return Ok({
    cacheView: {
      $case: 'doNotUse',
      doNotUse: Empty.create({}),
    },
  })
}
