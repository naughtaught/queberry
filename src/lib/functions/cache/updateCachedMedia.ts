import { activeGridViewSideBarItem, cache } from '$lib/stores/pages'
import { videoMetadata } from '$lib/stores/video'
import type { Api } from '$lib/types/api'
import { get } from 'svelte/store'

export const updateCachedMedia = (media: Api.MediaItem): void => {
    const sideBarItem = get(activeGridViewSideBarItem)
    if (sideBarItem && sideBarItem.id === media.id) activeGridViewSideBarItem.set(media)

    cache.update((currentCache) => {
        const routes = Object.keys(currentCache) as Array<keyof typeof currentCache>

        for (const route of routes) {
            const cacheItem = currentCache[route]

            if (route === 'person') {
                const personArray = cacheItem.media as Api.PersonData[]
                cacheItem.media = personArray.map((person: Api.PersonData) => {
                    const index = person.media.findIndex((item: Api.MediaItem) => item.id === media.id)
                    if (index !== -1) {
                        return {
                            ...person,
                            media: [...person.media.slice(0, index), media, ...person.media.slice(index + 1)],
                        }
                    }
                    return person
                })
            } else {
                const mediaArray = cacheItem.media as Api.MediaItem[]
                const index = mediaArray.findIndex((item: Api.MediaItem) => item.id === media.id)
                if (index !== -1) {
                    cacheItem.media = [...mediaArray.slice(0, index), media, ...mediaArray.slice(index + 1)]
                }
            }
        }

        if (get(videoMetadata).videoUrl) {
            videoMetadata.update((data) => {
                data.media = media
                data.playlist.forEach((playlist) => {
                    playlist.media = media
                })
                return data
            })
        }

        return currentCache
    })
}
