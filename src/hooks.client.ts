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
        args: [
            'tt0111161', // imdbId as first argument
            null, // season as second argument
            null, // episode as third argument
        ],
    })

    console.log(data)
}

initializeTauri()

call_plugin()
