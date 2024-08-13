import {
  Err,
  isErr,
  Ok,
  Result,
  collectResults,
  ErrorCodes,
} from '@shared/result'
import * as vscode from 'vscode'
import { DatabaseDependentSettings, SqlLanguage } from '@shared/config'
import {
  makeSnowflakeRequest,
  snowflakeRunStatement,
} from '@shared/databaseSnowflake'
import { TableAddress } from '@quary/proto/quary/service/v1/table_address'
import { ProjectFileSource } from '@quary/proto/quary/service/v1/project_file'
import {
  ModifiedConnectionConfig,
  ServicesDatabase,
  SourcesLister,
} from '@shared/database'

export class SnowflakeHeadless extends SnowflakeBase implements SourcesLister {}
