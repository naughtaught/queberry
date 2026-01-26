import { handleError, type Api } from '$lib'
import { getCurrentWindow } from '@tauri-apps/api/window'

export const minimizeApp = async (): Promise<void> => {
    const appWindow = getCurrentWindow()

    try {
        await appWindow.minimize()
    } catch (error) {
        const errorDetail: Api.ErrorDetail = {
            code: 500,
            message: error instanceof Error ? error.message : String(error),
            stack: error instanceof Error ? error.stack : undefined,
        }
        handleError(errorDetail)
    }
}
