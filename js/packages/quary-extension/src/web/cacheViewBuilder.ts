import {
  CacheViewInformation,
  CacheViewInformationPaths,
} from '@quary/proto/quary/service/v1/wasm_rust_rpc_calls'
import { Empty } from '@quary/proto/google/protobuf/empty'
import { isErr, Ok, Result } from '@shared/result'
import { ServicesDatabase } from './servicesDatabase'

export const cacheViewBuilder = async (
  database: ServicesDatabase,
): Promise<Result<CacheViewInformation>> => {
  if (database.returnDatabaseConfiguration().lookForCacheViews) {
    const tables = await database.listViews()
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
