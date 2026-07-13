import { enabledUtilityPlugins } from '$lib/stores/plugins'
import { videoMetadata, videoProperties } from '$lib/stores/video'
import type { Api } from '$lib/types/api'
import { get } from 'svelte/store'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { handleError } from '$lib/functions/errors/errorHandling'
import { updateLocalTimings } from '$lib/functions/video/updateLocalTimings'

export interface IntroDbResponse {
    type: string
    intro: Api.TimingRange[]
    recap: Api.TimingRange[]
    credits: Api.TimingRange[]
    preview: Api.TimingRange[]
}

export const getTimings = async (): Promise<void> => {
    const TimingPlugins = get(enabledUtilityPlugins).filter((plugin) => {
        const method = plugin.methods?.find((m) => m.interfaceMethod === 'GetIntroTimings')
        return method && (!method.requiresApiKey || !!plugin.apikey)
    })

    const metadata = get(videoMetadata)
    const properties = get(videoProperties)

    for (const plugin of TimingPlugins) {
        try {
            const resp = await invokeFunction('call_plugin_method', {
                pluginName: plugin.id,
                methodName: 'GetIntroTimings',
                args: [
                    plugin.apikey ?? null,
                    metadata.media?.imdb_id,
                    metadata.media?.tmdb_id,
                    metadata.media?.tvdb_id,
                    Math.round(properties.duration),
                    metadata.seasonNumber,
                    metadata.episode?.episode_num,
                    metadata.media?.type,
                ],
            })

            if (!resp.success) throw resp.error

            if (resp.data.length > 0) {
                const timingData = resp.data[0]

                updateLocalTimings(timingData)
            }
        } catch (error) {
            handleError(error)
        }
    }
}
