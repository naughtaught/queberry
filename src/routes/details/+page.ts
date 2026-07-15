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

    let data

    if (navigating.type === 'popstate' || url.pathname === '/video' || navigating.from?.url.pathname === '/video') {
        if (id) {
            const index = currentCache.details.media.findIndex((item) => item.id === +id)

            if (index !== -1) {
                return {
                    success: true,
                    data: currentCache.details.media[index],
                }
            }
        }
    }

    try {
        if (id && type) {
            const index = currentCache.details.media.findIndex((item) => item.id === +id)

            if (index !== -1) {
                data = currentCache.details.media[index]
            } else {
                const mediaItemResp = await fetchMediaItem(+id, type)

                if (!mediaItemResp.success) {
                    if (id && type && mediaItemResp.error.message.toLowerCase().includes('no data')) {
                        const newType = type === 'movie' ? 'tv' : 'movie'

                        const newTypeMediaResponse = await fetchMediaItem(+id, newType)

                        if (!newTypeMediaResponse.success) throw newTypeMediaResponse.error

                        data = newTypeMediaResponse.data
                        data.type = newType
                    } else {
                        throw mediaItemResp.error
                    }
                } else {
                    data = mediaItemResp.data
                }
            }

            if (data.type === 'tv') {
                const seasonResp = await fetchSeasonData(data.id)
                if (!seasonResp.success) throw seasonResp.error

                data.seasons = seasonResp.data
            }

            currentCache['details'].media.push(data)
        } else {
            throw createError(`Missing ID or Type: ID: ${id}, TYPE: ${type}`, 500, {})
        }

        return {
            success: true,
            data,
        }
    } catch (error) {
        return handleError(error)
    }
}
