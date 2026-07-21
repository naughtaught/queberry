import type { PageLoad } from './$types'
import { get } from 'svelte/store'
import { navigating } from '$app/state'
import { cache } from '$lib/stores/pages'
import { fetchMediaItem } from '$lib/db/fetchMediaItem'
import { fetchSeasonData } from '$lib/db/fetchSeasons'
import { createError, handleError } from '$lib/functions/errors/errorHandling'

export const load: PageLoad = async ({ url }) => {
    const id = url.searchParams.get('id')
    const type = url.searchParams.get('type')

    const currentCache = get(cache)

    if (navigating.type === 'popstate' || url.pathname === '/video' || navigating.from?.url.pathname === '/video') {
        if (id) {
            const cached = currentCache.details.media.find((item) => item.id === +id)
            if (cached) {
                return {
                    success: true,
                    data: cached,
                }
            }
        }
    }

    if (!id || !type) {
        return handleError(createError(`Missing ID or Type: ID: ${id}, TYPE: ${type}`, 500, {}))
    }

    try {
        const cached = currentCache.details.media.find((item) => item.id === +id)
        if (cached) {
            return {
                success: true,
                data: cached,
            }
        }

        const mediaItemPromise = fetchMediaItem(+id, type)
        let seasonPromise = type === 'tv' ? fetchSeasonData(+id) : null

        const mediaItemResp = await mediaItemPromise

        let data
        if (!mediaItemResp.success) {
            if (mediaItemResp.error.message.toLowerCase().includes('no data')) {
                seasonPromise = null

                const newType = type === 'movie' ? 'tv' : 'movie'
                const newTypeMediaResponse = await fetchMediaItem(+id, newType)

                if (!newTypeMediaResponse.success) throw newTypeMediaResponse.error
                data = newTypeMediaResponse.data
                data.type = newType

                if (data.type === 'tv') {
                    seasonPromise = fetchSeasonData(data.id)
                }
            } else {
                throw mediaItemResp.error
            }
        } else {
            data = mediaItemResp.data
        }

        if (data.type === 'tv' && !data.seasons && seasonPromise) {
            const seasonResp = await seasonPromise
            if (!seasonResp.success) throw seasonResp.error
            data.seasons = seasonResp.data
        }

        cache.update((current) => {
            const mediaIndex = current.details.media.findIndex((item) => item.id === +id)
            if (mediaIndex === -1) {
                current.details.media.push(data)
            } else {
                current.details.media[mediaIndex] = data
            }
            return current
        })

        return {
            success: true,
            data,
        }
    } catch (error) {
        return handleError(error)
    }
}
