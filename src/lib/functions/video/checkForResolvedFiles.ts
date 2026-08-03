import type { Video } from '$lib/types/video'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { handleError } from '$lib/functions/errors/errorHandling'
import { parseFilenameForEpisode } from '$lib/functions/utility/parseFilenameForEpisode'

export const checkForResolvedFiles = async (
    imdbId: string,
    seasonNum: number | null,
    episodeNum: number | null,
): Promise<{ link: string; pluginId: string; filename: string; infohash: string; files: Video.Files[] } | null> => {
    try {
        const response = await invokeFunction('get_resolved_cache_by_imdb_id', {
            imdbId,
        })

        if (!response.success) throw response.error
        if (response.data.length === 0) return null

        for (const cached of response.data) {
            const files: Video.Files[] = JSON.parse(cached.filesJson)

            let link: string | undefined
            let filename: string | undefined

            if (seasonNum && episodeNum) {
                const file = parseFilenameForEpisode(seasonNum, episodeNum, files)
                if (file) {
                    link = file.link
                    filename = file.filename
                }
            } else {
                const target = files.reduce((a, b) => (b.size > a.size ? b : a))
                link = target.link
                filename = target.filename
            }

            if (link && filename) {
                return {
                    link,
                    filename,
                    files,
                    infohash: cached.infohash,
                    pluginId: cached.pluginId,
                }
            }
        }

        return null
    } catch (error) {
        handleError(error, { display: false })
        return null
    }
}
