import { defaultSessionSettings, handleError, settings, type Api } from '$lib'
import { invoke } from '@tauri-apps/api/core'

// const initializeTauri = async () => {
//     try {
//         // TODO pass to store
//         await invoke('get_plugins')
//     } catch (error) {
//         console.error('Failed to initialize Tauri:', error)
//     }
// }

// TODO AUTH
try {
    const userSettings: Api.ApiResponse = await invoke('get_user_settings', { userId: 1 })
    if (userSettings.success) {
        settings.set(userSettings.data)
        defaultSessionSettings.update((current) => ({
            ...current,
            volume: userSettings.data.volume,
        }))
    } else if (userSettings.error) {
        handleError(userSettings.error)
    }
} catch (error) {
    const errorDetail: Api.ErrorDetail = {
        code: 500,
        message: error instanceof Error ? error.message : String(error),
        stack: error instanceof Error ? error.stack : undefined,
    }
    handleError(errorDetail)
}

// initializeTauri()
