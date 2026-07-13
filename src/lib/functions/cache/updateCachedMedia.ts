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

            const index = cacheItem.media.findIndex((item: { id: number }) => item.id === media.id)

            if (index !== -1) {
                cacheItem.media = [...cacheItem.media.slice(0, index), media, ...cacheItem.media.slice(index + 1)]
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
