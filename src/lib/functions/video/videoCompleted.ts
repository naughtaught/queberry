import { settings } from '$lib/stores/user'
import { videoMetadata } from '$lib/stores/video'
import { get } from 'svelte/store'
import { setEpisodeWatchedState } from '$lib/functions/states/setEpisodeWatchedState'
import { setMediaStates } from '$lib/functions/states/setMediaStates'
import { updateCachedMedia } from '$lib/functions/cache/updateCachedMedia'
import { fetchUpNext } from '$lib/db/fetchUpNext'
import { addPlaylistItem } from '$lib/functions/video/addPlaylistItem'
import { createError, handleError } from '$lib/functions/errors/errorHandling'
import { shuffleSettings } from '$lib/stores/pages'

export const videoCompleted = async (): Promise<void> => {
    try {
        const metadata = get(videoMetadata)
        const userSettings = get(settings)
        if (!metadata) throw createError(`No metadata found`, 500, {})
        if (!metadata.media) throw createError(`No metadata media found`, 500, {})

        let updatedMedia = metadata.media

        if (metadata.media?.type === 'tv') {
            if (!metadata.episode) throw createError(`No episode found`, 500, {})
            const response = await setEpisodeWatchedState(metadata.episode, metadata.media, true)
            if (!response.success) throw response.error
            updatedMedia = response.data.media
        }

        if (!metadata.media.watched && !metadata.media.hidden) {
            const resp = await setMediaStates(updatedMedia, {
                watched: true,
                in_watchlist: false,
                hidden: false,
                progress: null,
                episode_id: null,
            })
            if (!resp.success) throw resp.error

            updateCachedMedia({ ...resp.data })
        } else if (metadata.media.progress) {
            const resp = await setMediaStates(updatedMedia, {
                watched: metadata.media.watched ?? false,
                in_watchlist: false,
                hidden: metadata.media.hidden ?? false,
                progress: null,
                episode_id: null,
            })
            if (!resp.success) throw resp.error

            updateCachedMedia({ ...resp.data })
        }

        if (metadata.media.type === 'tv') await fetchUpNext()

        if (
            get(shuffleSettings).continuousPlay ||
            get(shuffleSettings).randomEpisodes ||
            metadata.collectionIndex + 1 < metadata.collectionItems.length ||
            (metadata.media.type === 'tv' && userSettings.autoplay)
        ) {
            addPlaylistItem()
        }
    } catch (error) {
        handleError(error, { context: 'Attempting to add a playlist item' })
    }
}
