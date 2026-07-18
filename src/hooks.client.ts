import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { handleError } from '$lib/functions/errors/errorHandling'
import { loginUser } from '$lib/functions/user/loginUser'
import { parentalControlsAreEnabled } from '$lib/stores/app'
import {
    installedIndexerPlugins,
    installedResolverPlugins,
    installedUtilityPlugins,
    transfersInProgress,
} from '$lib/stores/plugins'
import { user, users } from '$lib/stores/user'
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

try {
    initializePlugins()
    getTransfers()

    let userId: number | null = null

    const response = await invokeFunction('get_all_users', {})

    if (!response.success) throw response.error

    users.set(response.data)

    if (response.data.length === 1 && !get(parentalControlsAreEnabled)) {
        user.set(response.data[0])
        userId = response.data[0].id
    }
    if (userId) loginUser(response.data[0])
} catch (error) {
    handleError(error)
}
