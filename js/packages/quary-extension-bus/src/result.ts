/**
 * Result type for functions that can fail.
 */

export type Result<T> = Ok<T> | Err<Error>
export type ResultE<T, E> = Ok<T> | Err<E>

export type Ok<T> = {
  readonly ok: true
  readonly value: T
}

export type Err<E> = {
  readonly ok: false
  readonly error: E
  readonly details?: ErrorDetails
}

export function Ok<T>(value: T): Ok<T> {
  return { ok: true, value }
}

export function Err(error: Error, details?: ErrorDetails): Err<Error>
export function Err(error: string, details?: ErrorDetails): Err<string>
export function Err<E>(error: E, details?: ErrorDetails): Err<E> {
  return { ok: false, error, details }
}

export function isOk<T, E>(result: ResultE<T, E>): result is Ok<T>
export function isOk<T>(result: Result<T>): result is Ok<T> {
  return result.ok
}

export function isErr<T, E>(result: ResultE<T, E>): result is Err<E>
export function isErr<T>(result: Result<T>): result is Err<Error> {
  return !result.ok
}

export type DetailedError = {
  message: string
  details?: ErrorDetails
}

// adds the ability to add more details to errors
// TODO: add other specific error types
type ErrorDetails = {
  type: 'modelReferenceNotFound'
  message: string // TODO: convert to useful object
}

export function getErrorDetails<E>(err: Err<E>): DetailedError {
  const message =
    err.error instanceof Error ? err.error.message : String(err.error)
  return { message, details: err.details }
}

/**
 * Collects the results of an array of results into a single result.
 */
export function collectResults<T, E>(
  array: Array<ResultE<T, E>>,
): Result<Array<T>> {
  const errors: Array<E> = []
  const values: Array<T> = []
  for (const result of array) {
    if (isErr(result)) {
      errors.push(result.error)
    } else {
      values.push(result.value)
    }
  }
  if (errors.length > 0) {
    return Err(new Error(`Errors: ${errors.join(', ')}`))
  }
  return Ok(values)
}
