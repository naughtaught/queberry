import { handleError, settings, type Api } from '$lib'
import { invoke } from '@tauri-apps/api/core'
import { get } from 'svelte/store'

const initializeTauri = async () => {
    try {
        const plugins = await invoke('get_plugins')
        console.log('Plugins loaded:', plugins)
    } catch (error) {
        console.error('Failed to initialize Tauri:', error)
    }
}

try {
    const userSettings: Api.ApiResponse = await invoke('get_user_settings', { userId: 1 })
    if (userSettings.success) {
        settings.set(userSettings.data)
        console.log(get(settings))
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

initializeTauri()
