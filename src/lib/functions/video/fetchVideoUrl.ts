import { enabledResolverPlugins } from '$lib/stores/plugins'
import { settings } from '$lib/stores/user'
import type { App } from '$lib/types/app'
import type { Plugins } from '$lib/types/plugins'
import { get } from 'svelte/store'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { createError, formatError, handleError } from '$lib/functions/errors/errorHandling'

export const fetchVideoUrl = async (
    source: Plugins.IndexerSource,
    seasonNumber: number | null,
    episodeNumber: number | null,
): Promise<App.Response> => {
    const userSettings = get(settings)
    const defaultResolver = userSettings.defaultResolver
    let resolvers = get(enabledResolverPlugins)

    if (resolvers.length === 0) {
        throw createError('No enabled resolvers found', 501, {
            log: false,
        })
    }

    resolvers = resolvers
        .filter((resolver) => {
            const method = resolver.methods?.find((m) => m.interfaceMethod === 'FetchVideoUrl')
            return !method?.requiresApiKey || !!resolver.apikey
        })
        .sort((a, b) => {
            if (a.id === defaultResolver) return -1
            if (b.id === defaultResolver) return 1

            const aCacheless = a.cacheless === true ? 1 : 0
            const bCacheless = b.cacheless === true ? 1 : 0

            return aCacheless - bCacheless
        })

    if (resolvers.length === 0) {
        return {
            success: true,
            data: null,
        }
    }

    const allResolversUseCache = resolvers.every((resolver) => !resolver.cacheless)

    try {
        for (const resolver of resolvers) {
            if (!resolver.cacheless && source.cached && source.cached.length < 1) {
                if (allResolversUseCache) {
                    throw createError('No sources found.', 404, {
                        log: false,
                    })
                }
                continue
            }

            const response = await invokeFunction('call_plugin_method', {
                pluginName: resolver.id,
                methodName: 'FetchVideoUrl',
                args: [resolver.apikey ?? null, source.info_hash, seasonNumber, episodeNumber],
            })

            if (!response.success) {
                handleError(response.error)
                continue
            }

            if (response.data) {
                return {
                    success: true,
                    data: {
                        ...response.data,
                        infohash: source.info_hash,
                        resolver,
                        files: response.data.files,
                    },
                }
            }
        }

        return {
            success: true,
            data: null,
        }
    } catch (error) {
        return formatError(error)
    }
}
