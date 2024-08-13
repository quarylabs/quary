import {
  BigQueryBase,
} from '@shared/databaseBigQuery'
import {
  SourcesLister,
} from '@shared/database'


export class BigQueryOauthHeadless
  extends BigQueryBase
  implements SourcesLister {}
