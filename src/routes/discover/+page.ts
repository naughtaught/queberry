import type { PageLoad } from './$types'
import { get } from 'svelte/store'
import { navigating } from '$app/state'
import { cache, currentFilters, defaultFilters } from '$lib/stores/pages'
import { fetchData } from '$lib/db/fetchData'
import { handleError } from '$lib/functions/errors/errorHandling'

export const load: PageLoad = ({ url }) => {
    const currentCache = get(cache)
    const current = get(currentFilters)

    if (navigating.type === 'popstate' || url.pathname === '/video' || navigating.from?.url.pathname === '/video') {
        return {
            success: true,
            data: currentCache['discover'].media,
        }
    }

    currentFilters.set({
        ...get(defaultFilters),
    })

    try {
        const dataPromise = (async () => {
            const response = await fetchData()

            if (!response.success) throw response.error

            currentCache['discover'].media = [...response.data]
            currentCache['discover'].paginationPage = 0
            currentCache['discover'].lastUpdated = Date.now()
            currentCache['discover'].filters = current

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
