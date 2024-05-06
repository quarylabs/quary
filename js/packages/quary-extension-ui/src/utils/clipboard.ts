export const copyToClipboard = async (text: string) => {
  await navigator.clipboard.writeText(text)
}
