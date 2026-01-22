import { exit } from '@tauri-apps/plugin-process'
import { handleError, type Api } from '$lib'

export const closeApp = async () => {
    try {
        await exit(0)
    } catch (error) {
        const errorDetail: Api.ErrorDetail = {
            code: 500,
            message: error instanceof Error ? error.message : String(error),
            stack: error instanceof Error ? error.stack : undefined,
        }
        handleError(errorDetail)
    }
}
