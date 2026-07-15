import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { createError, formatError } from '$lib/functions/errors/errorHandling'
import { validateUser } from '$lib/functions/user/validateUser'
import type { App } from '$lib/types/app'

export const fetchPersonDetails = async (id: number): Promise<App.Response> => {
    try {
        const currentUser = validateUser()
        if (!currentUser) throw createError('Missing User', 401, { log: false })

        const response = await invokeFunction('api_fetch_person_details', {
            postgresId: currentUser.postgresId,
            token: currentUser.token,
            personId: id,
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
