import type { PageLoad } from './$types'
import { navigating } from '$app/state'
import { cache } from '$lib/stores/pages'
import { get } from 'svelte/store'
import { createError, handleError } from '$lib/functions/errors/errorHandling'
import { fetchPersonDetails } from '$lib/db/fetchPersonDetails'

export const load: PageLoad = async ({ url }) => {
    const id = url.searchParams.get('id')
    const currentCache = get(cache)

    let data

    if (navigating.type === 'popstate' || url.pathname === '/video' || navigating.from?.url.pathname === '/video') {
        if (id) {
            const index = currentCache.person.media.findIndex((item) => item.id === +id)

            if (index !== -1) {
                return {
                    success: true,
                    data: currentCache.person.media[index],
                }
            }
        }
    }

    try {
        if (id) {
            const index = currentCache.person.media.findIndex((item) => item.id === +id)

            if (index !== -1) {
                data = currentCache.person.media[index]
            } else {
                const resp = await fetchPersonDetails(+id)

                if (!resp.success) throw resp.error

                data = resp.data

                currentCache['person'].media.push(data)
            }
        } else {
            throw createError(`Missing ID  ID: ${id}`, 500, {})
        }

        return {
            success: true,
            data,
        }
    } catch (error) {
        return handleError(error)
    }
}
