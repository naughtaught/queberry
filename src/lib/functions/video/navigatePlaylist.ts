import { videoMetadata } from '$lib'
import { handleError } from '$lib/stores/errorHandling'
import { get } from 'svelte/store'
import { invokeFunction } from '../api/invokeFunction'

export const navigatePlaylist = async (direction: 'next' | 'previous'): Promise<void> => {
    try {
        const resp = await invokeFunction(`${direction}_playlist_item`, {})

        if (resp.error) throw resp.error

        const metadata = get(videoMetadata)
        const { playlist, playlistIndex: currentIndex } = metadata
        const newPlaylistIndex = direction === 'next' ? currentIndex + 1 : currentIndex - 1
        const playlistItem = playlist.find((x) => x.playlistIndex === newPlaylistIndex)
        const newMetadata = {
            ...playlistItem!,
            playlist: metadata.playlist,
        }

        videoMetadata.set(newMetadata)
    } catch (error) {
        handleError(error, {
            context: 'next_playlist_item invocation failed',
        })
    }
}
