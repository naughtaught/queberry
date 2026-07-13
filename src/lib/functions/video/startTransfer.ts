import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { createError, handleError } from '$lib/functions/errors/errorHandling'
import type { Plugins } from '$lib/types/plugins'
import { get } from 'svelte/store'
import { settings } from '$lib/stores/user'
import { enabledResolverPlugins, transfersInProgress } from '$lib/stores/plugins'
import { checkMethodApi } from '$lib/functions/plugins/checkMethodApi'

export const startTransfer = async (source: Plugins.IndexerSource): Promise<void> => {
    try {
        const defaultResolver = get(settings).defaultResolver
        const resolver = get(enabledResolverPlugins).find((x) => x.id === defaultResolver)

        if (!resolver) {
            throw createError('No Resolver found.', 404, {
                log: false,
            })
        }

        checkMethodApi(resolver, 'AddFilesToCache')

        const response = await invokeFunction('call_plugin_method', {
            pluginName: resolver.id,
            methodName: 'AddFilesToCache',
            args: [resolver.apikey ?? null, source.info_hash],
        })

        if (!response.success) throw response.error

        const resp = await invokeFunction('create_transfer', {
            hash: source.info_hash,
            transfer: {
                transferId: +response.data.id,
                progress: 0,
                status: response.data.status,
                resolver: resolver.id,
            },
        })

        if (!resp.success) throw resp.error

        transfersInProgress.update((transfers) => ({
            ...transfers,
            [source.info_hash]: {
                transferId: response.data.id,
                filename: source.filename,
                progress: response.data.progress,
                hash: source.info_hash,
                status: response.data.status,
                resolver: resolver.id,
                speed: 0,
            },
        }))
    } catch (error) {
        handleError(error)
    }
}
