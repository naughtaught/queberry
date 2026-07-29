import { videoMetadata } from '$lib/stores/video'
import { get } from 'svelte/store'
import { getNextEpisode } from '$lib/functions/utility/getNextEpisode'
import type { Video } from '$lib/types/video'
import { parseFilenameForEpisode } from '$lib/functions/utility/parseFilenameForEpisode'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { updateVideoMetadata } from '$lib/functions/video/updateVideoMetadata'
import { createError, handleError } from '$lib/functions/errors/errorHandling'
import { checkParentalControls } from '$lib/functions/video/checkParentalControls'
import { addCollectionItemToPlaylist } from '$lib/functions/video/addCollectionItemToPlaylist'
import { shuffleSettings } from '$lib/stores/pages'
import { addShuffleItemToPlaylist } from '$lib/functions/video/addShuffleItemToPlaylist'
import { unrestrictLink } from './unrestrictLink'
import { resolveVideoData } from './resolveVideoData'

export const addPlaylistItem = async (): Promise<void> => {
    try {
        const metadata = get(videoMetadata)

        if (get(shuffleSettings).continuousPlay || get(shuffleSettings).randomEpisodes) {
            addShuffleItemToPlaylist(metadata)
            return
        }

        const { newSeasonNumber, newEpisode, newEpisodeNumber } = getNextEpisode()

        if (!newSeasonNumber || !newEpisode || !newEpisodeNumber || !metadata.media) {
            if (metadata.collectionItems.length > 0) addCollectionItemToPlaylist(metadata)
            return
        }

        const playlistItem: Video.PlaylistItem = {
            ...metadata,
            seasonNumber: newSeasonNumber,
            episode: newEpisode,
            playlistIndex: metadata.playlistIndex! + 1,
            videoUrl: null,
            filename: null,
        }

        if (metadata.files.length > 1 && metadata.media.type === 'tv') {
            const file = parseFilenameForEpisode(newSeasonNumber, newEpisodeNumber, metadata.files)
            if (file) {
                if (!playlistItem.resolver) throw createError('Resolver not found', 404, {})

                const downloadLink = await unrestrictLink(
                    playlistItem.resolver.apikey,
                    file.link,
                    playlistItem.resolver,
                )

                if (downloadLink) {
                    playlistItem.videoUrl = downloadLink
                    playlistItem.filename = file.filename
                }
            }
        }

        checkParentalControls(metadata.media)

        if (!playlistItem.videoUrl) {
            const originalSeasonNumber = newEpisode?.original_season_num ?? newSeasonNumber
            const originalEpisodeNumber = newEpisode?.original_episode_num ?? newEpisodeNumber
            const imdbId = newEpisode.imdb_id ?? metadata.media.imdb_id

            const videoData = await resolveVideoData(
                imdbId,
                metadata.media,
                originalSeasonNumber,
                originalEpisodeNumber,
                newEpisode,
                null,
                null,
            )
            Object.assign(playlistItem, videoData)
        }

        const resp = await invokeFunction('add_playlist_item', { value: { url: playlistItem.videoUrl } })
        if (!resp.success) throw resp.error

        updateVideoMetadata(metadata, playlistItem)
    } catch (error) {
        handleError(error, { context: 'adding an item to the playlist failed' })
    }
}
