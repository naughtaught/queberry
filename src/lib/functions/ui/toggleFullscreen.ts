import { appState } from '$lib/stores/app'
import { handleError } from '$lib/stores/errorHandling'
import type { Api } from '$lib/types/api'
import { getCurrentWindow } from '@tauri-apps/api/window'

export const toggleFullscreen = async (): Promise<void> => {
    const window = getCurrentWindow()
    try {
        const isFullscreen = await window.isFullscreen()

        await window.setFullscreen(!isFullscreen)
        appState.set({
            ...appState,
            isFullscreen: !isFullscreen,
        })
    } catch (error) {
        const errorDetail: Api.ErrorDetail = {
            code: 500,
            message: error instanceof Error ? error.message : String(error),
            stack: error instanceof Error ? error.stack : undefined,
        }
        handleError(errorDetail)
    }
}
