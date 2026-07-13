import { SKIP_ERRORS } from '$lib/stores/app'
import type { Plugins } from '$lib/types/plugins'
import { fetchSources } from '$lib/functions/plugins/fetchSources'
import { fetchVideoUrl } from '$lib/functions/video/fetchVideoUrl'
import { createError } from '$lib/functions/errors/errorHandling'

export const fetchVideoFromSources = async (
    options: Plugins.FetchVideoOptions,
    existingSources?: Plugins.IndexerSource[],
): Promise<Plugins.FetchVideoResult> => {
    const { imdbId, title, released, type, seasonNumber, episodeNumber, episodeId, skipErrors = true } = options

    let sources: Plugins.IndexerSource[]

    if (existingSources && existingSources.length > 0) {
        sources = existingSources
    } else {
        const resp = await fetchSources(imdbId, title, released, type, seasonNumber, episodeNumber, episodeId)
        if (!resp.success) throw resp.error
        if (resp.data.length === 0) {
            throw createError(`No sources found for ${imdbId}`, 404, { log: false })
        }
        sources = resp.data
    }

    for (const source of sources) {
        const response = await fetchVideoUrl(source, seasonNumber, episodeNumber)

        if (!response.success) {
            const errorMessage = response?.error?.message?.toLowerCase() || ''
            if (skipErrors && SKIP_ERRORS.some((e) => errorMessage.includes(e))) continue
            throw response.error
        }

        if (response.data?.download_link) {
            return {
                videoUrl: response.data.download_link,
                filename: response.data.filename,
                files: response.data.files,
                infohash: response.data.infohash,
                resolver: response.data.resolver,
            }
        }
    }

    throw createError('No Video File Found', 400, { log: false })
}
