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

// const call_plugin = async () => {
//     try {
//         const data = await invoke('call_plugin_method', {
//             pluginName: 'app.torbox.resolver',
//             methodName: 'CreateTorrent',
//             args: [
//                 'b11e149e-7570-4f56-ad1f-e4d05c863f27',
//                 '16d20542c3d718c32667153c069f6511dcba3db2',
//             ],
//         })
//         console.log(data)
//     } catch (error) {
//         console.log(error)
//     }
// }

initializeTauri()

call_plugin()

// tt12637874 2, 2
// tt0111161

// args: [
//     'b11e149e-7570-4f56-ad1f-e4d05c863f27',
//     ['16d20542c3d718c32667153c069f6511dcba3db2'],
// ],
