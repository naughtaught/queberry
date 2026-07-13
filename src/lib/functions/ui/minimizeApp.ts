import { getCurrentWindow } from '@tauri-apps/api/window'
import { handleError } from '$lib/functions/errors/errorHandling'

export const minimizeApp = async (): Promise<void> => {
    const appWindow = getCurrentWindow()

    try {
        await appWindow.minimize()
    } catch (error) {
        handleError(error, {
            context: 'minimize app failed',
        })
    }
}
