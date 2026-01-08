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
    const [zilean, torrentio] = await Promise.all([
        invoke('call_plugin_method', {
            pluginName: 'com.zilean.indexer',
            methodName: 'GetIndexerSources',
            args: ['tt0111161', null, null],
        }),
        invoke('call_plugin_method', {
            pluginName: 'fun.strem.torrentio',
            methodName: 'GetIndexerSources',
            args: ['tt0111161', null, null],
        }),
    ])

    console.log(zilean, torrentio)
}

initializeTauri()

call_plugin()

// tt12637874 2, 2
// tt0111161
