import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { createError, formatError } from '$lib/functions/errors/errorHandling'
import { validateUser } from '$lib/functions/user/validateUser'
import type { App } from '$lib/types/app'

export const fetchRecentReleases = async (): Promise<App.Response> => {
    try {
        const currentUser = validateUser()
        if (!currentUser) throw createError('Missing User', 401, { log: false })

        const response = await invokeFunction('api_fetch_recent_releases', {
            postgresId: currentUser.postgresId,
            token: currentUser.token,
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
