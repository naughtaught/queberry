import { invoke } from '@tauri-apps/api/core'

const initializeTauri = async () => {
    try {
        console.log('Loading Tauri plugins...')
        const plugins = await invoke('get_plugins')
        console.log('Plugins loaded:', plugins)
    } catch (error) {
        console.error('Failed to initialize Tauri:', error)
    }
}

initializeTauri()
