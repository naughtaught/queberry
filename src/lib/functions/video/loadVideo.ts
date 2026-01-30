import { goto } from '$app/navigation'
import { handleError, invokeFunction } from '$lib'
import { resolve } from '$app/paths'

export const loadVideo = async () => {
    // 'D:/Media/Movies/The Raid (2012)',
    // 'https://dn710604.ca.archive.org/0/items/BigBuckBunny_124/Content/big_buck_bunny_720p_surround.mp4',

    const resp = await invokeFunction('load_video', {
        value: {
            url: 'D:/Media/Movies/The Raid (2012)',
            userId: 1,
        },
    })

    if (resp.success) goto(resolve('/video', { bg: 'transparent' }))
    if (resp.error) handleError(resp.error)
}
