import { handleError, invokeFunction, videoMetadata } from '$lib'
import { resolve } from '$app/paths'
import { goto } from '$app/navigation'
import type { Video } from '$lib/types/video'
import { updateVideoMetadata } from './updateVideoMetadata'
import { get } from 'svelte/store'

// TODO
export const addPlaylistItem = async (): Promise<void> => {
    try {
        const resp = await invokeFunction('add_playlist_item', {
            value: {
                url: 'D:/Media/Movies/(500) Days of Summer (2009)',
            },
        })

        if (resp.error) throw resp.error

        goto(resolve('/video', { bg: 'transparent' }))

        const metadata = get(videoMetadata)
        const playlistItem: Video.PlaylistItem = {
            ...metadata,
            title: '(500) Days of Summer (2009)',
            file: 'D:/Media/Movies/(500) Days of Summer (2009)',
            type: 'movie',
            seasonNumber: null,
            episodeNumber: null,
            playlistIndex: metadata.playlistIndex! + 1,
        }

        updateVideoMetadata(metadata, playlistItem)
    } catch (error) {
        handleError(error, {
            context: 'adding an item to the playlist failed',
        })
    }
}
