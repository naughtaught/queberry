import { enabledResolverPlugins } from '$lib/stores/plugins'
import type { Video } from '$lib/types/video'
import { get } from 'svelte/store'
import { checkForResolvedFiles } from '$lib/functions/video/checkForResolvedFiles'
import { fetchLocalMedia } from '$lib/functions/video/fetchLocalMedia'
import { unrestrictLink } from '$lib/functions/video/unrestrictLink'
import { fetchVideoFromSources } from '$lib/functions/video/fetchVideoFromSources'
import { addToResolvedCache } from '$lib/functions/video/addToResolvedCache'
import type { Api } from '$lib/types/api'
import type { Plugins } from '$lib/types/plugins'
import { createError } from '$lib/functions/errors/errorHandling'

export const resolveVideoData = async (
    imdbId: string,
    media: Api.MediaItem,
    originalSeasonNumber: number | null,
    originalEpisodeNumber: number | null,
    episode: Api.Episode | null,
    checkCancellation: (() => void) | null,
    existingSources: Plugins.IndexerSource[] | null,
    targeted = false,
): Promise<Video.VideoData> => {
    checkCancellation?.()

    if (targeted && existingSources && existingSources.length > 0) {
        if (existingSources[0].source === 'Local Media' && existingSources[0].filePath) {
            return {
                videoUrl: existingSources[0].filePath,
                filename: existingSources[0].filename,
                files: [],
                infohash: null,
                resolver: 'Local Media',
            }
        }
    }

    if (!targeted) {
        const localResults = await fetchLocalMedia(
            imdbId,
            media.title,
            media.released,
            media.type,
            originalSeasonNumber,
            originalEpisodeNumber,
        )

        if (localResults.length > 0 && localResults[0].filePath) {
            return {
                videoUrl: localResults[0].filePath,
                filename: localResults[0].filename,
                files: [],
                infohash: null,
                resolver: 'Local Media',
            }
        }
    }

    const resolvedData = await checkForResolvedFiles(imdbId, originalSeasonNumber, originalEpisodeNumber)

    checkCancellation?.()

    if (resolvedData) {
        const plugin = get(enabledResolverPlugins).find((x) => x.id === resolvedData.pluginId)

        if (plugin) {
            const downloadLink = await unrestrictLink(plugin.apikey, resolvedData.link, plugin)

            if (downloadLink) {
                return {
                    videoUrl: downloadLink,
                    filename: resolvedData.filename,
                    files: resolvedData.files,
                    infohash: resolvedData.infohash,
                    resolver: plugin,
                }
            }
        }
    }

    checkCancellation?.()

    const videoData = await fetchVideoFromSources(
        {
            imdbId,
            title: media.title,
            released: media.released,
            type: media.type,
            seasonNumber: media.type === 'tv' ? originalSeasonNumber : null,
            episodeNumber: media.type === 'tv' ? originalEpisodeNumber : null,
            episodeId: episode?.episode_id ?? null,
        },
        existingSources ?? [],
    )

    if (!videoData?.videoUrl) {
        throw createError(`Unexpected: No video URL after successful source fetch for ${imdbId} - ${media.title}`, 500)
    }

    checkCancellation?.()

    if (videoData.infohash && videoData.videoUrl && videoData.resolver) {
        addToResolvedCache(imdbId, {
            videoUrl: videoData.videoUrl,
            files: videoData.files,
            infohash: videoData.infohash,
            resolver: videoData.resolver.id,
        })
    }

    return videoData
}
