export type JSONStruct = { [x: string]: JSONValue }

export type JSONValue =
  | string
  | number
  | boolean
  | { [x: string]: JSONValue }
  | Array<JSONValue>

export type JSONValueWithNulls =
  | string
  | number
  | boolean
  | { [x: string]: JSONValueWithNulls }
  | Array<JSONValueWithNulls>
  | null

/**
 * Convert a non-legal JSONValueWithNulls to a valid JSONValue by recursively removing all null values
 */
export const dropNullValuesInJSONLike = (
  jsonLike: JSONValueWithNulls,
): JSONValue => {
  if (jsonLike === null) {
    throw new Error('Top-level value cannot be null')
  }

  if (Array.isArray(jsonLike)) {
    return jsonLike.reduce((acc: JSONValue[], curr: JSONValueWithNulls) => {
      if (curr !== null) {
        acc.push(dropNullValuesInJSONLike(curr) as JSONValue)
      }
      return acc
    }, [])
  }

  if (typeof jsonLike === 'object') {
    return Object.entries(jsonLike).reduce(
      (acc: { [x: string]: JSONValue }, [key, value]) => {
        if (value !== null) {
          acc[key] = dropNullValuesInJSONLike(value) as JSONValue
        }
        return acc
      },
      {},
    )
  }

  return jsonLike
}
