import type { Sql } from '$lib/types/sql'
import type { App } from '$lib/types/app'
import { get } from 'svelte/store'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { handleError } from '$lib/functions/errors/errorHandling'
import { hashBlacklist, restrictedContent, settings, user, users } from '$lib/stores/user'
import { directories, modals, parentalControlsAreEnabled, primaryUser } from '$lib/stores/app'
import { setUserSettings } from '$lib/functions/user/setUserSettings'
import { defaultSessionSettings, keyboardShortcuts, sessionSettings } from '$lib/stores/video'
import { currentFilters, defaultFilters } from '$lib/stores/pages'
import { updateCarousels } from '$lib/functions/utility/updateCarousels'
import { setEnabledPlugins } from '$lib/functions/plugins/setEnabledPlugins'
import { toggleFullscreen } from '$lib/functions/ui/toggleFullscreen'
import { getLocalMedia } from '../utility/getLocalMedia'

export const loginUser = async (userData: Sql.User, skipVerification = false): Promise<void> => {
    try {
        if (!skipVerification) {
            const tokenResponse = await invokeFunction('api_verify_token', {
                postgresId: userData.postgresId,
                token: userData.token,
            })
            if (!tokenResponse.success) {
                user.set(null)
                throw tokenResponse.error
            }
            if (!tokenResponse.data) {
                user.set(null)
                handleError('Session invalid. Please re-add your account.')
                return
            }
        }

        const allUsers = await invokeFunction('get_all_users', {})
        if (!allUsers.success) throw allUsers.error

        users.set(allUsers.data)

        modals.update((modal) => ({
            ...modal,
            user: false,
        }))

        await setUserSettings(userData.id)

        if (get(settings).openAppFullscreen) toggleFullscreen()

        user.set(userData)

        sessionSettings.set(get(defaultSessionSettings))

        const usersKeyboardShortcuts = await invokeFunction('get_user_keyboard_shortcuts', {
            userId: userData.id,
        })

        if (!usersKeyboardShortcuts.success) throw usersKeyboardShortcuts.error

        if (usersKeyboardShortcuts.success && usersKeyboardShortcuts.data) {
            const defaultShortcuts = get(keyboardShortcuts)

            const mappedShortcuts = defaultShortcuts.map((shortcut) => ({
                ...shortcut,
                code: usersKeyboardShortcuts.data[shortcut.id]?.code || shortcut.code,
                shiftKey: usersKeyboardShortcuts.data[shortcut.id]?.shiftKey || shortcut.shiftKey,
            }))

            keyboardShortcuts.set(mappedShortcuts)
        }

        if (get(parentalControlsAreEnabled)) {
            const content_ratings = await invokeFunction('get_user_content_ratings', {
                userId: userData.id,
            })

            if (!content_ratings.success) throw content_ratings.error

            if (content_ratings.data) {
                const newRestrictedContent = {
                    tv: content_ratings.data.restrictedTv || [],
                    movies: content_ratings.data.restrictedMovies || [],
                }

                restrictedContent.set(newRestrictedContent)

                defaultFilters.update((filters) => ({
                    ...filters,
                    contentRatings: newRestrictedContent.movies,
                }))
            }
        }

        currentFilters.set(get(defaultFilters))

        updateCarousels().catch((err) => handleError(err))

        const blacklistedResponse: App.Response = await invokeFunction('get_users_blacklisted', {
            userId: userData.id,
        })

        if (blacklistedResponse.success && blacklistedResponse.data.length > 0) {
            hashBlacklist.set(blacklistedResponse.data)
        }

        const pluginsResp = await setEnabledPlugins()
        if (!pluginsResp.success) throw pluginsResp.error

        const globalSettingsResp = await invokeFunction('get_global_settings', {})
        if (!globalSettingsResp.success) handleError(globalSettingsResp.error, { display: false })

        if (globalSettingsResp.success && globalSettingsResp.data) {
            primaryUser.set(globalSettingsResp.data.primaryUserId)
            parentalControlsAreEnabled.set(globalSettingsResp.data.parentalControlsAreEnabled)

            directories.set({
                tv: globalSettingsResp.data.tvDirectory ?? null,
                movies: globalSettingsResp.data.movieDirectory ?? null,
            })

            getLocalMedia()
        }

        try {
            const updateDownloads: App.Response = await invokeFunction('cleanup_downloads_on_login', {
                userId: userData.id,
            })
            if (!updateDownloads.success) throw updateDownloads.error
        } catch (error) {
            handleError(error)
        }
    } catch (error) {
        handleError(error)
    }
}
