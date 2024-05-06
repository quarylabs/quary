/**
 * Sleep for a given amount of time
 *
 * @param ms - The amount of time to sleep in milliseconds
 */
export const sleep = (ms: number): Promise<void> =>
  new Promise((resolve) => setTimeout(resolve, ms))
