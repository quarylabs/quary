import { SqlLanguage } from '@shared/config'

export type Language = SqlLanguage

export const toSingleLine = (str: string) =>
  str.replace(/\n/g, ' ').replace(/\s+/g, ' ').trim()
