import { handleError } from '$lib'
import { getCurrentWindow } from '@tauri-apps/api/window'

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
