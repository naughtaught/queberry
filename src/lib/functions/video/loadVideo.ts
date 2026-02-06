import { goto } from '$app/navigation'
import { handleError, invokeFunction, videoMetadata } from '$lib'
import { resolve } from '$app/paths'

// TODO
export const loadVideo = async (): Promise<void> => {
    // 'D:/Media/Movies/The Raid (2012)',
    // 'D:/Media/Movies/(500) Days of Summer (2009)'
    // 'https://dn710604.ca.archive.org/0/items/BigBuckBunny_124/Content/big_buck_bunny_720p_surround.mp4',

    try {
        const resp = await invokeFunction('load_video', {
            value: {
                url: 'D:/Media/Movies/The Raid (2012)',
                userId: 1,
            },
        })

        if (resp.error) throw resp.error

        const playlistItem = {
            title: 'The Raid (2012)',
            file: 'D:/Media/Movies/The Raid (2012)',
            type: 'movie',
            seasonNumber: null,
            episodeNumber: null,
            playlistIndex: 0,
        }

        const metadata = {
            ...playlistItem,
            playlist: [playlistItem],
        }

        videoMetadata.set(metadata)

        goto(resolve('/video', { bg: 'transparent' }))
    } catch (error) {
        handleError(error, {
            context: 'loading a video failed',
        })
    }
}
