import { fetchRandom } from '$lib/db/fetchRandom'
import { fetchSeasonData } from '$lib/db/fetchSeasons'
import { shuffleSettings } from '$lib/stores/pages'
import type { Api } from '$lib/types/api'
import type { Video } from '$lib/types/video'
import { get } from 'svelte/store'
import { createError, getErrorMessage } from '$lib/functions/errors/errorHandling'
import { getRandomEpisode } from '$lib/functions/video/getRandomEpisode'
import { fetchVideoFromSources } from '$lib/functions/video/fetchVideoFromSources'
import { checkParentalControls } from '$lib/functions/video/checkParentalControls'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { updateVideoMetadata } from '$lib/functions/video/updateVideoMetadata'
import { fetchLocalMedia } from '$lib/functions/video/fetchLocalMedia'

export const addShuffleItemToPlaylist = async (metadata: Video.Metadata): Promise<void> => {
    if (!metadata.media) return

    if (get(shuffleSettings).continuousPlay) {
        shuffleSettings.update((s) => ({
            ...s,
            excludeMediaIds: [...new Set([...s.excludeMediaIds, metadata.media!.id])],
        }))
    }

    if (get(shuffleSettings).randomEpisodes && metadata.episode) {
        shuffleSettings.update((s) => ({
            ...s,
            playedEpisodeIds: [...new Set([...s.playedEpisodeIds, metadata.episode!.episode_id])],
        }))
    }

    const MAX_RETRIES = 3
    let attempt = 0

    while (attempt < MAX_RETRIES) {
        attempt++

        const newMedia = await fetchRandom()
        if (!newMedia.success) throw newMedia.error
        if (!newMedia.data) return

        const media: Api.MediaItem = newMedia.data

        let seasonNumber: number | null = null
        let episode: Api.Episode | null = null

        if (media.type === 'tv') {
            const seasonResp = await fetchSeasonData(media.id)
            if (!seasonResp.success) {
                shuffleSettings.update((s) => ({
                    ...s,
                    excludeMediaIds: [...new Set([...s.excludeMediaIds, media.id])],
                }))
                if (attempt < MAX_RETRIES) continue
                throw seasonResp.error
            }
            media.seasons = seasonResp.data
            if (!media.seasons) {
                shuffleSettings.update((s) => ({
                    ...s,
                    excludeMediaIds: [...new Set([...s.excludeMediaIds, media.id])],
                }))
                if (attempt < MAX_RETRIES) continue
                return
            }

            const result = getRandomEpisode(media.seasons)
            if (result) {
                seasonNumber = result.seasonNumber
                episode = result.episode
            } else {
                shuffleSettings.update((s) => ({
                    ...s,
                    excludeMediaIds: [...new Set([...s.excludeMediaIds, media.id])],
                }))
                if (attempt < MAX_RETRIES) continue
            }
        }

        const playlistItem: Video.PlaylistItem = {
            videoUrl: null,
            filename: null,
            files: [],
            backdrop: null,
            seasonNumber,
            episode,
            language: null,
            infohash: null,
            resolver: null,
            playlistIndex: metadata.playlistIndex! + 1,
            media,
            collectionItems: [],
            collectionIndex: 0,
        }

        checkParentalControls(media)

        if (media.type === 'tv' && !episode) {
            shuffleSettings.update((s) => ({
                ...s,
                excludeMediaIds: [...new Set([...s.excludeMediaIds, media.id])],
            }))
            if (attempt < MAX_RETRIES) continue
            throw createError(`Missing episode data ${media.imdb_id}`, 500, { log: true })
        }

        const imdbId = episode?.imdb_id ?? media.imdb_id

        try {
            const localResults = await fetchLocalMedia(
                imdbId,
                media.title,
                media.released,
                media.type,
                media.type === 'tv' ? seasonNumber : null,
                media.type === 'tv' ? (episode?.episode_num ?? null) : null,
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
                    seasonNumber: media.type === 'tv' ? seasonNumber : null,
                    episodeNumber: media.type === 'tv' ? (episode?.episode_num ?? null) : null,
                    episodeId: episode?.episode_id ?? null,
                })
            }

            Object.assign(playlistItem, videoData)

            const resp = await invokeFunction('add_playlist_item', { value: { url: playlistItem.videoUrl } })
            if (!resp.success) throw resp.error

            updateVideoMetadata(metadata, playlistItem)
            return
        } catch (error) {
            const errorMessage = getErrorMessage(error).toLowerCase()
            const isNoSourcesError =
                errorMessage.includes('no sources found') || errorMessage.includes('no video file found')

            if (isNoSourcesError && attempt < MAX_RETRIES) {
                shuffleSettings.update((s) => ({
                    ...s,
                    excludeMediaIds: [...new Set([...s.excludeMediaIds, media.id])],
                }))
                continue
            }

            throw error
        }
    }

    throw createError('Max retries exceeded', 504, { log: false })
}
