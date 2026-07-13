import type { PageLoad } from './$types'
import { get } from 'svelte/store'
import { navigating } from '$app/state'
import { cache, currentFilters, defaultFilters } from '$lib/stores/pages'
import { fetchData } from '$lib/db/fetchData'
import { handleError } from '$lib/functions/errors/errorHandling'

export const load: PageLoad = ({ url }) => {
    const currentCache = get(cache)
    const current = get(currentFilters)

    if (navigating.type === 'popstate' || url.pathname === '/video' || navigating.from?.url.pathname === '/video')
        return {
            success: true,
            data: currentCache['watchlist'].media,
        }

    currentFilters.set({
        ...get(defaultFilters),
        includeFavourites: true,
        includeHidden: true,
        includeWatched: true,
        includeWatchlisted: true,
        onlyWatchlisted: true,
    })

    try {
        const dataPromise = (async () => {
            const response = await fetchData()

            if (!response.success) throw response.error

            currentCache['watchlist'].media = [...response.data]
            currentCache['watchlist'].paginationPage = 0
            currentCache['watchlist'].lastUpdated = Date.now()
            currentCache['watchlist'].filters = current

            return response.data
        })()

        return {
            success: true,
            data: dataPromise,
        }
    } catch (err) {
        return handleError(err)
    }
}
