import { directories } from '$lib/stores/app'
import type { Sql } from '$lib/types/sql'
import type { App } from '$lib/types/app'
import { get } from 'svelte/store'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { handleError } from '$lib/functions/errors/errorHandling'

export const updateGlobalSettings = async (user: Sql.User, parentalControls: boolean): Promise<App.Response> => {
    try {
        const response = await invokeFunction('update_global_settings', {
            parentalControlsAreEnabled: parentalControls,
            primaryUserId: user.id,
            tvDirectory: get(directories).tv,
            movieDirectory: get(directories).movies,
        })

        if (!response.success) throw response.error

        return {
            success: true,
            data: response.data,
        }
    } catch (error) {
        return handleError(error)
    }
}
