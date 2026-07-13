import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { createError, handleError } from '$lib/functions/errors/errorHandling'
import { validateUser } from '$lib/functions/user/validateUser'
import { loadingStates } from '$lib/stores/app'
import { cache } from '$lib/stores/pages'

export const fetchUpNext = async (): Promise<void> => {
    try {
        const currentUser = validateUser()
        if (!currentUser) throw createError('Missing User', 401, { log: false })

        loadingStates.update((state) => ({
            ...state,
            isUpNextLoading: true,
        }))

        const response = await invokeFunction('api_fetch_up_next', {
            postgresId: currentUser.postgresId,
            token: currentUser.token,
            localDate: new Date().toLocaleDateString('en-CA'),
        })

        if (!response.success) throw response.error
        cache.update((currentCache) => ({
            ...currentCache,
            upNext: {
                ...currentCache.upNext,
                media: response.data,
            },
        }))
    } catch (error) {
        handleError(error)
    } finally {
        loadingStates.update((state) => ({
            ...state,
            isUpNextLoading: false,
        }))
    }
}
