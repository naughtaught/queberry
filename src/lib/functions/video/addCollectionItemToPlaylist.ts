import { fetchMediaItem } from '$lib/db/fetchMediaItem'
import { fetchSeasonData } from '$lib/db/fetchSeasons'
import type { Video } from '$lib/types/video'
import { createError, handleError } from '$lib/functions/errors/errorHandling'
import { fetchVideoFromSources } from '$lib/functions/video/fetchVideoFromSources'
import { checkParentalControls } from '$lib/functions/video/checkParentalControls'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { updateVideoMetadata } from '$lib/functions/video/updateVideoMetadata'
import { fetchLocalMedia } from '$lib/functions/video/fetchLocalMedia'

export const addCollectionItemToPlaylist = async (metadata: Video.Metadata): Promise<void> => {
    try {
        const newCollectionIndex = metadata.collectionIndex + 1
        const collectionItem = metadata.collectionItems[newCollectionIndex]

        let mediaResp = await fetchMediaItem(collectionItem.media_id, collectionItem.type)
        let newType = collectionItem.type

        if (!mediaResp.success) {
            if (!mediaResp.error.message.toLowerCase().includes('no data')) throw mediaResp.error
            newType = newType === 'movie' ? 'tv' : 'movie'
            mediaResp = await fetchMediaItem(collectionItem.media_id, newType)
            if (!mediaResp.success) throw mediaResp.error
        }

        if (!mediaResp.data) throw createError('Missing media data', 500, { log: false })

        const media = { ...mediaResp.data, type: newType }

        if (media.type === 'tv') {
            const seasonResp = await fetchSeasonData(media.id)
            if (!seasonResp.success) throw seasonResp.error
            media.seasons = seasonResp.data
        }

        const episode =
            media.type === 'tv'
                ? (media.seasons.seasons
                      .find((s: { season_num: number }) => s.season_num === 1)
                      ?.default_episodes.find((ep: { episode_num: number }) => ep.episode_num === 1) ?? null)
                : null

        const playlistItem: Video.PlaylistItem = {
            videoUrl: null,
            filename: null,
            files: [],
            backdrop: null,
            seasonNumber: media.type === 'tv' ? 1 : null,
            episode,
            language: null,
            infohash: null,
            resolver: null,
            playlistIndex: metadata.playlistIndex! + 1,
            media,
            collectionItems: metadata.collectionItems,
            collectionIndex: newCollectionIndex,
        }

        checkParentalControls(media)

        if (media.type === 'tv' && !playlistItem.episode) {
            throw createError(`Missing episode data ${media.imdb_id}`, 500, { log: true })
        }

        const originalSeasonNumber = episode?.original_season_num ?? 1
        const originalEpisodeNumber = episode?.original_episode_num ?? 1
        const imdbId = episode?.imdb_id ?? media.imdb_id

        const localResults = await fetchLocalMedia(
            imdbId,
            media.title,
            media.released,
            media.type,
            originalSeasonNumber,
            originalEpisodeNumber,
        )

        let videoData
        if (localResults.length > 0) {
            videoData = {
                videoUrl: localResults[0].filePath,
                filename: localResults[0].filename,
                files: [],
                infohash: null,
                resolver: 'Local Media',
            }
        } else {
            videoData = await fetchVideoFromSources({
                imdbId,
                title: media.title,
                released: media.released,
                type: media.type,
                seasonNumber: media.type === 'tv' ? originalSeasonNumber : null,
                episodeNumber: media.type === 'tv' ? originalEpisodeNumber : null,
                episodeId: episode?.episode_id ?? null,
            })
        }

        Object.assign(playlistItem, videoData)

        const resp = await invokeFunction('add_playlist_item', { value: { url: playlistItem.videoUrl } })
        if (!resp.success) throw resp.error

        updateVideoMetadata(metadata, playlistItem)
    } catch (error) {
        handleError(error)
    }
}
