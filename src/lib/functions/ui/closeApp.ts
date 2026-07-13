import { exit } from '@tauri-apps/plugin-process'
import { handleError } from '$lib/functions/errors/errorHandling'

export const closeApp = async (): Promise<void> => {
    try {
        await exit(0)
    } catch (error) {
        handleError(error, {
            context: 'close app failed',
        })
    }
}
