import { videoMetadata } from '$lib/stores/video'
import { get } from 'svelte/store'
import { getNextEpisode } from '$lib/functions/utility/getNextEpisode'
import type { Video } from '$lib/types/video'
import { parseFilenameForEpisode } from '$lib/functions/utility/parseFilenameForEpisode'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { updateVideoMetadata } from '$lib/functions/video/updateVideoMetadata'
import { createError, handleError } from '$lib/functions/errors/errorHandling'
import { fetchVideoFromSources } from '$lib/functions/video/fetchVideoFromSources'
import { checkParentalControls } from '$lib/functions/video/checkParentalControls'
import { addCollectionItemToPlaylist } from '$lib/functions/video/addCollectionItemToPlaylist'
import { shuffleSettings } from '$lib/stores/pages'
import { addShuffleItemToPlaylist } from '$lib/functions/video/addShuffleItemToPlaylist'
import { fetchLocalMedia } from '$lib/functions/video/fetchLocalMedia'
import { checkMethodApi } from '$lib/functions/plugins/checkMethodApi'

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

                checkMethodApi(playlistItem.resolver, 'UnrestrictLink')

                const resp = await invokeFunction('call_plugin_method', {
                    pluginName: playlistItem.resolver.id,
                    methodName: 'UnrestrictLink',
                    args: [playlistItem.resolver.apikey ?? null, file.link],
                })
                if (!resp.success) throw resp.error
                if (resp.data?.link) {
                    playlistItem.videoUrl = resp.data.link
                    playlistItem.filename = file.filename
                }
            }
        }

        checkParentalControls(metadata.media)

        if (!playlistItem.videoUrl) {
            const originalSeasonNumber = newEpisode?.original_season_num ?? newSeasonNumber
            const originalEpisodeNumber = newEpisode?.original_episode_num ?? newEpisodeNumber
            const imdbId = newEpisode.imdb_id ?? metadata.media.imdb_id

            const localResults = await fetchLocalMedia(
                imdbId,
                metadata.media.title,
                metadata.media.released,
                metadata.media.type,
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
                    title: metadata.media.title,
                    released: metadata.media.released,
                    type: metadata.media.type,
                    seasonNumber: originalSeasonNumber,
                    episodeNumber: originalEpisodeNumber,
                    episodeId: newEpisode.episode_id,
                })
            }

            Object.assign(playlistItem, videoData)
        }

        const resp = await invokeFunction('add_playlist_item', { value: { url: playlistItem.videoUrl } })
        if (!resp.success) throw resp.error

        updateVideoMetadata(metadata, playlistItem)
    } catch (error) {
        handleError(error, { context: 'adding an item to the playlist failed' })
    }
}
