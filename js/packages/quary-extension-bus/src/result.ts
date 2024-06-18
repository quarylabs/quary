/**
 * Result type for functions that can fail.
 */

export type Result<T> = Ok<T> | Err<QuaryError>
export type ResultE<T, E> = Ok<T> | Err<E>

export type Ok<T> = {
  readonly ok: true
  readonly value: T
}

export type Err<E> = {
  readonly ok: false
  readonly error: E
}

export function Ok<T>(value: T): Ok<T> {
  return { ok: true, value }
}

export function Err<E>(error: E): Err<E>
export function Err(error: QuaryError): Err<QuaryError> {
  return { ok: false, error }
}

export function isOk<T, E>(result: ResultE<T, E>): result is Ok<T>
export function isOk<T>(result: Result<T>): result is Ok<T> {
  return result.ok
}

export function isErr<T, E>(result: ResultE<T, E>): result is Err<E>
export function isErr<T>(result: Result<T>): result is Err<QuaryError> {
  return !result.ok
}

/**
 * Collects the results of an array of results into a single result.
 */
export function collectResults<T>(
  array: Array<Result<T>>,
): ResultE<Array<T>, Array<QuaryError>> {
  const errors: Array<QuaryError> = []
  const values: Array<T> = []
  for (const result of array) {
    if (isErr(result)) {
      errors.push(result.error)
    } else {
      values.push(result.value)
    }
  }
  if (errors.length > 0) {
    return Err(errors)
  }
  return Ok(values)
}

export enum ErrorCodes {
  OK = 0,
  CANCELLED = 1,
  UNKNOWN = 2,
  INVALID_ARGUMENT = 3,
  DEADLINE_EXCEEDED = 4,
  NOT_FOUND = 5,
  ALREADY_EXISTS = 6,
  PERMISSION_DENIED = 7,
  RESOURCE_EXHAUSTED = 8,
  FAILED_PRECONDITION = 9,
  ABORTED = 10,
  OUT_OF_RANGE = 11,
  UNIMPLEMENTED = 12,
  INTERNAL = 13,
  UNAVAILABLE = 14,
  DATA_LOSS = 15,
  UNAUTHENTICATED = 16,
}

/**
 * Converts an error code to a human-readable string.
 *
 */
export function codeToString(code: ErrorCodes): string {
  switch (code) {
    case ErrorCodes.OK:
      return 'Ok'
    case ErrorCodes.CANCELLED:
      return 'Cancelled'
    case ErrorCodes.UNKNOWN:
      return 'Unknown Error'
    case ErrorCodes.INVALID_ARGUMENT:
      return 'Invalid Argument'
    case ErrorCodes.DEADLINE_EXCEEDED:
      return 'Deadline Exceeded'
    case ErrorCodes.NOT_FOUND:
      return 'Not Found'
    case ErrorCodes.ALREADY_EXISTS:
      return 'Already Exists'
    case ErrorCodes.PERMISSION_DENIED:
      return 'Permission Denied'
    case ErrorCodes.RESOURCE_EXHAUSTED:
      return 'Resource Exhausted'
    case ErrorCodes.FAILED_PRECONDITION:
      return 'Failed Precondition'
    case ErrorCodes.ABORTED:
      return 'Aborted'
    case ErrorCodes.OUT_OF_RANGE:
      return 'Out of Range'
    case ErrorCodes.UNIMPLEMENTED:
      return 'Unimplemented'
    case ErrorCodes.INTERNAL:
      return 'Internal'
    case ErrorCodes.UNAVAILABLE:
      return 'Unavailable'
    case ErrorCodes.DATA_LOSS:
      return 'Data Loss'
    case ErrorCodes.UNAUTHENTICATED:
      return 'Unauthenticated'
    default:
      throw new Error(`Unknown error code: ${code} that should never happen`)
  }
}

export type QuaryError = {
  code: ErrorCodes
  message: string
  tree?: QuaryError
}

export function isQuaryError(value: unknown): value is QuaryError {
  return (
    typeof value === 'object' &&
    value !== null &&
    'code' in value &&
    'message' in value
  )
}
