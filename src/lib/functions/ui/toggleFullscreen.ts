import { appState } from '$lib/stores/app'
import { handleError } from '$lib/stores/errorHandling'
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
        handleError(error, {
            context: 'toggle fullscreen failed',
        })
    }
}
