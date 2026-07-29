import type { Video } from '$lib/types/video'
import { invokeFunction } from '../api/invokeFunction'
import { handleError } from '../errors/errorHandling'

export const addToResolvedCache = async (
    imdbId: string,
    data: {
        videoUrl: string
        files: Video.Files[]
        infohash: string
        resolver: string
    },
): Promise<void> => {
    try {
        const response = await invokeFunction('create_resolved_cache', {
            imdbId,
            infohash: data.infohash,
            pluginId: data.resolver,
            filesJson: JSON.stringify(data.files),
        })

        if (!response.success) throw response.error
    } catch (error) {
        handleError(error, { display: false })
    }
}
