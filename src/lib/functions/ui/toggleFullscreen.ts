import { isAppFullscreen } from '$lib/stores/app'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { handleError } from '$lib/functions/errors/errorHandling'

export const toggleFullscreen = async (): Promise<void> => {
    const window = getCurrentWindow()
    try {
        const isFullscreen = await window.isFullscreen()

        await window.setFullscreen(!isFullscreen)
        isAppFullscreen.set(!isFullscreen)
    } catch (error) {
        handleError(error, {
            context: 'toggle fullscreen failed',
        })
    }
}
