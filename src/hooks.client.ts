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

const call_plugin = async () => {
    const data = await invoke('call_plugin_method', {
        pluginName: 'com.zilean.indexer',

        // pluginName: 'fun.strem.torrentio',
        methodName: 'GetIndexerSources',
        args: ['tt12637874', 2, 2],
    })

    console.log(data)
}

initializeTauri()

call_plugin()

// tt12637874 2, 2
// tt0111161
