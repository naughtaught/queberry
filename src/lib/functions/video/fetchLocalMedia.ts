import type { Sql } from '$lib/types/sql'
import type { Plugins } from '$lib/types/plugins'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { handleError } from '$lib/functions/errors/errorHandling'
import { get } from 'svelte/store'
import { directories } from '$lib/stores/app'

export const fetchLocalMedia = async (
    imdbId: string,
    title: string,
    year: number,
    type: string,
    season: number | null,
    episode: number | null,
): Promise<Plugins.IndexerSource[]> => {
    const tvDir = get(directories).tv
    const movieDir = get(directories).movies

    if (!tvDir || !movieDir) {
        if (type === 'tv' && !tvDir) return []
        if (type === 'movie' && !movieDir) return []
    }

    const local_media = await invokeFunction('find_local_media', {
        imdbId,
        title,
        year,
        season,
        episode,
    })

    if (!local_media.success) handleError(local_media.error, { log: true, display: false })

    if (local_media.success && local_media.data?.filepaths?.length > 0) {
        const localResults = local_media.data.filepaths.map((fp: Sql.LocalFilepath) => ({
            ...fp,
            cached: ['Local Media'],
            indexer: 'Local Media',
            source: 'Local Media',
            info_hash: '',
            seeds: 0,
            blacklisted: false,
            isDefault: fp.isDefault || false,
            filename: fp.filePath.split(/[\\/]/).pop(),
            title: fp.filePath.split(/[\\/]/).pop(),
            language: fp.language ? JSON.parse(fp.language) : [],
            video_filters: fp.videoFilters ? JSON.parse(fp.videoFilters) : [],
            tags: fp.tags ? JSON.parse(fp.tags) : [],
            video_details: {
                codec: fp.videoCodec,
                bitrate: null,
                framerate: null,
            },
            audio_details: {
                codec: fp.audioCodec,
                channels: fp.audioChannels,
                sampling_rate: null,
                bitrate: null,
            },
        }))

        return localResults
    }

    return []
}
