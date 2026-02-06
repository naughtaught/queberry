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

    if (userSettings.error) throw userSettings.error

    if (userSettings.success) {
        settings.set(userSettings.data)
        defaultSessionSettings.update((current) => ({
            ...current,
            volume: userSettings.data.volume,
        }))
    }
} catch (error) {
    handleError(error, {
        context: 'userSettings invocation failed',
    })
}

// initializeTauri()
