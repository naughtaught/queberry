import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { handleError } from '$lib/functions/errors/errorHandling'
import { loginUser } from '$lib/functions/user/loginUser'
import { checkForUpdates } from '$lib/functions/utility/checkForUpdates'
import { directories, loadingStates, parentalControlsAreEnabled, primaryUser } from '$lib/stores/app'
import {
    installedIndexerPlugins,
    installedResolverPlugins,
    installedUtilityPlugins,
    transfersInProgress,
} from '$lib/stores/plugins'
import { user, users } from '$lib/stores/user'
import type { App } from '$lib/types/app'
import type { Plugins } from '$lib/types/plugins'
import { get } from 'svelte/store'

const initializePlugins = async (): Promise<void> => {
    const plugins = await invokeFunction('get_plugins', {})

    if (!plugins.success) throw plugins.error

    installedIndexerPlugins.set(plugins.data.filter((plugin: Plugins.Plugin) => plugin.types.includes('Indexer')))
    installedResolverPlugins.set(plugins.data.filter((plugin: Plugins.Plugin) => plugin.types.includes('Resolver')))
    installedUtilityPlugins.set(plugins.data.filter((plugin: Plugins.Plugin) => plugin.types.includes('Utility')))
}

const getTransfers = async (): Promise<void> => {
    try {
        const transfers = await invokeFunction('list_transfers', {})
        if (!transfers.success) throw transfers.error

        transfersInProgress.set({
            ...transfers.data,
        })
    } catch (error) {
        handleError(error, {
            display: false,
        })
    }
}

const getLocalMedia = async (): Promise<void> => {
    loadingStates.update((states) => ({
        ...states,
        isLocalMediaLoading: true,
    }))

    const tvDirectory = get(directories).tv
    const movieDirectory = get(directories).movies

    try {
        if (tvDirectory || movieDirectory) {
            if (tvDirectory === movieDirectory) {
                try {
                    const mediaDir = await invokeFunction('scan_local_folder', {
                        directory: tvDirectory,
                    })
                    if (!mediaDir.success) throw mediaDir.error
                } catch (error) {
                    const err = error as App.ErrorDetail
                    if (err.message.includes('I/O error: The system cannot find the path specified')) {
                        directories.update((dirs) => ({
                            ...dirs,
                            tv: null,
                            movies: null,
                        }))
                    } else {
                        handleError(error)
                    }
                }
            } else {
                const results = await Promise.allSettled([
                    invokeFunction('scan_local_folder', {
                        directory: tvDirectory,
                    }),
                    invokeFunction('scan_local_folder', {
                        directory: movieDirectory,
                    }),
                ])

                results.forEach((result, index) => {
                    if (result.status === 'rejected') {
                        const error = result.reason
                        if (error.message.includes('I/O error: The system cannot find the path specified')) {
                            directories.update((dirs) => ({
                                ...dirs,
                                [index === 0 ? 'tv' : 'movies']: null,
                            }))
                        } else {
                            handleError(error)
                        }
                    } else if (result.status === 'fulfilled') {
                        const media = result.value
                        if (!media.success) {
                            handleError(media.error)
                        }
                    }
                })
            }
        }
    } catch (error) {
        handleError(error)
    } finally {
        loadingStates.update((states) => ({
            ...states,
            isLocalMediaLoading: false,
        }))
    }
}

setTimeout(() => {
    checkForUpdates().catch((error) => handleError(error, { display: false }))
}, 2000)

try {
    initializePlugins()
    getTransfers()

    let userId: number | null = null

    const response = await invokeFunction('get_all_users', {})

    if (!response.success) throw response.error

    users.set(response.data)

    if (response.data.length > 0) {
        const resp = await invokeFunction('get_global_settings', {})

        if (!resp.success) throw resp.error

        if (resp.data) {
            primaryUser.set(resp.data.primaryUserId)
            parentalControlsAreEnabled.set(resp.data.parentalControlsAreEnabled)
            directories.set({
                tv: resp.data.tvDirectory,
                movies: resp.data.movieDirectory,
            })
            getLocalMedia()
        }
    }

    if (response.data.length === 1 && !get(parentalControlsAreEnabled)) {
        user.set(response.data[0])
        userId = response.data[0].id
    }
    if (userId) loginUser(response.data[0])
} catch (error) {
    handleError(error)
}
