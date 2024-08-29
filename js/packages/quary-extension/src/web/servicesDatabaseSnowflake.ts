import { SnowflakeBase } from '@shared/databaseSnowflake'
import { SourcesLister } from '@shared/database'

export class SnowflakeHeadless extends SnowflakeBase implements SourcesLister {}
