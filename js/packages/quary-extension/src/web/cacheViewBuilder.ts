import {
  CacheViewInformation,
  CacheViewInformationPaths,
} from '@quary/proto/quary/service/v1/wasm_rust_rpc_calls'
import { Empty } from '@quary/proto/google/protobuf/empty'
import { Err, isErr, Ok, ResultE } from '@shared/result'
import { ServicesDatabase } from './servicesDatabase'

export const cacheViewBuilder = async (
  database: ServicesDatabase,
): Promise<ResultE<CacheViewInformation, string>> => {
  if (database.returnDatabaseConfiguration().lookForCacheViews) {
    const tables = await database.listViews()
    if (isErr(tables)) {
      return Err(`Error listing tables: ${tables.error}`)
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
