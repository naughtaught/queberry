import type { Api } from '$lib/types/api'
import type { App } from '$lib/types/app'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { createError, formatError } from '$lib/functions/errors/errorHandling'
import { validateUser } from '$lib/functions/user/validateUser'

export const setMediaStates = async (
    media: Api.MediaItem,
    watchedStates: Api.WatchedStates,
): Promise<App.Response> => {
    try {
        const currentUser = validateUser()
        if (!currentUser) throw createError('Missing User', 401, { log: false })

        const data = {
            ...watchedStates,
            inCollection: watchedStates.in_collection,
            inWatchlist: watchedStates.in_watchlist,
            episodeId: watchedStates.episode_id,
            postgresId: currentUser.postgresId,
            mediaId: media.id,
        }

        const upsertResponse = await invokeFunction('api_upsert_user_media', {
            postgresId: currentUser.postgresId,
            token: currentUser.token,
            data,
        })

        if (!upsertResponse.success) throw upsertResponse.error

        const result = { ...media, ...watchedStates }

        return {
            success: true,
            data: result,
        }
    } catch (error) {
        return formatError(error)
    }
}
