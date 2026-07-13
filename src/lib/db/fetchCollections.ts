import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { createError, formatError } from '$lib/functions/errors/errorHandling'
import { validateUser } from '$lib/functions/user/validateUser'
import type { App } from '$lib/types/app'

export const fetchCollections = async (media_id: number): Promise<App.Response> => {
    try {
        const currentUser = validateUser()
        if (!currentUser) throw createError('Missing User', 401, { log: false })

        const response = await invokeFunction('api_fetch_collections', {
            postgresId: currentUser.postgresId,
            token: currentUser.token,
            mediaId: media_id,
        })
        if (!response.success) throw response.error
        return {
            success: true,
            data: response.data,
        }
    } catch (error) {
        return formatError(error)
    }
}
