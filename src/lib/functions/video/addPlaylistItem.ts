import { invokeFunction } from '$lib'

export const addPlaylistItem = async (): Promise<void> => {
    const resp = await invokeFunction('add_playlist_item', {
        value: {
            url: 'D:/Media/Movies/(500) Days of Summer (2009)',
        },
    })

    console.log(resp)
}
