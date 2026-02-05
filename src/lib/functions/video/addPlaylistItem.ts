import { handleError, invokeFunction, videoMetadata } from '$lib'
import { resolve } from '$app/paths'
import { goto } from '$app/navigation'

export const addPlaylistItem = async (): Promise<void> => {
    const resp = await invokeFunction('add_playlist_item', {
        value: {
            url: 'D:/Media/Movies/(500) Days of Summer (2009)',
        },
    })

    if (resp.error) {
        handleError(resp.error)
        return
    }

    if (resp.success) {
        goto(resolve('/video', { bg: 'transparent' }))
        videoMetadata.set({
            title: 'D:/Media/Movies/(500) Days of Summer (2009)',
            type: 'movie',
        })
    }
}
