import { useEffect, useState } from 'react'
import { useCallBackFrontEnd } from '@shared/callBacks'
import {
  USE_GLOBAL_STATE_MESSAGE_TYPE_NOT_SET,
  USE_GLOBAL_STATE_MESSAGE_TYPE_SET,
  View,
} from '@shared/globalViewState'
import { vscode } from '@/utils/VSCodeAPIWrapper'

export const useGlobalState = (): [View] => {
  const [state, setState] = useState<View>({ type: 'loading' })
  const { useGlobalStateNotSet } = useCallBackFrontEnd(
    [USE_GLOBAL_STATE_MESSAGE_TYPE_NOT_SET],
    vscode.postMessage,
  )

  // This useEffect is used to register listeners for messages from the extension.
  useEffect(() => {
    const callback = async () => {
      const storedState = vscode.getState() as View | undefined
      if (storedState !== undefined && storedState.type === 'loading') {
        setState(storedState)
        return
      }

      if (state.type === 'loading') {
        vscode.addEventListener((event) => {
          if (event.data.type === USE_GLOBAL_STATE_MESSAGE_TYPE_SET) {
            const payload = event.data.payload as View
            setState(payload)
            vscode.setState(payload)
          }
        })
        // eslint-disable-next-line react-hooks/rules-of-hooks
        useGlobalStateNotSet(null)
      }
    }

    callback()
  }, [useGlobalStateNotSet, setState, state.type])

  return [state]
}
