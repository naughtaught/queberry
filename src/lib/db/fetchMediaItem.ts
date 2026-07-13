import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { createError, formatError } from '$lib/functions/errors/errorHandling'
import { validateUser } from '$lib/functions/user/validateUser'
import type { App } from '$lib/types/app'

export const fetchMediaItem = async (id: number, type: string): Promise<App.Response> => {
    const mediaType = type === 'tv' ? 'tv' : 'movies'
    try {
        const currentUser = validateUser()
        if (!currentUser) throw createError('Missing User', 401, { log: false })

        const response = await invokeFunction('api_fetch_media_item', {
            postgresId: currentUser.postgresId,
            token: currentUser.token,
            id,
            mediaType,
        })
        if (!response.success) throw response.error
        return {
            success: true,
            data: response.data,
        }
    } catch (err) {
        return formatError(err)
    }
}
