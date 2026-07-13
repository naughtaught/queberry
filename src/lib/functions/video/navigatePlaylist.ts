import { get } from 'svelte/store'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { videoMetadata } from '$lib/stores/video'
import { handleError } from '$lib/functions/errors/errorHandling'

export const navigatePlaylist = async (direction: 'next' | 'previous'): Promise<void> => {
    try {
        const resp = await invokeFunction(`${direction}_playlist_item`, {})

        if (!resp.success) throw resp.error

        const metadata = get(videoMetadata)
        const { playlist, playlistIndex: currentIndex } = metadata
        const newPlaylistIndex = direction === 'next' ? currentIndex + 1 : currentIndex - 1
        const playlistItem = playlist.find((x: { playlistIndex: number }) => x.playlistIndex === newPlaylistIndex)
        const newMetadata = {
            ...playlistItem!,
            media: metadata.media,
            playlist: metadata.playlist,
        }

        videoMetadata.set(newMetadata)
    } catch (error) {
        handleError(error, {
            context: 'next_playlist_item invocation failed',
        })
    }
}
