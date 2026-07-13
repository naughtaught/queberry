import { parentalControlsAreEnabled } from '$lib/stores/app'
import { restrictedContent } from '$lib/stores/user'
import type { Api } from '$lib/types/api'
import { get } from 'svelte/store'
import { createError } from '$lib/functions/errors/errorHandling'

export const checkParentalControls = (media: Api.MediaItem): void => {
    if (!get(parentalControlsAreEnabled)) return

    const { tv, movies } = get(restrictedContent)

    if (!media.content_rating) {
        throw createError('Parental Lock: Content restricted due to unknown content rating', 403, { log: false })
    }

    if (tv.includes(media.content_rating) || movies.includes(media.content_rating)) {
        throw createError('Parental Lock: Content restricted due to content rating', 403, { log: false })
    }
}
