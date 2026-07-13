import { settings } from '$lib/stores/user'
import { defaultSessionSettings } from '$lib/stores/video'
import { invokeFunction } from '$lib/functions/api/invokeFunction'

export const setUserSettings = async (id: number): Promise<void> => {
    const userSettingsResponse = await invokeFunction('get_user_settings', { userId: id })

    if (!userSettingsResponse.success) throw userSettingsResponse.error

    if (userSettingsResponse.success) {
        settings.set(userSettingsResponse.data)

        defaultSessionSettings.update((sessionSettings) => ({
            ...sessionSettings,
            volume: userSettingsResponse.data.volume,
        }))
    }
}
