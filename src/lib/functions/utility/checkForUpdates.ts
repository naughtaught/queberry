import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import { formatError, handleError } from '$lib/functions/errors/errorHandling'
import type { App } from '$lib/types/app'
import { getVersion } from '@tauri-apps/api/app'
import patchNotesData from '$lib/data/patchnotes.json'
import { appData } from '$lib/stores/app'

export const checkForUpdates = async (): Promise<App.Response> => {
    try {
        const currentVersion = await getVersion()

        try {
            const currentNotes = (patchNotesData as App.PatchNotesMap)[currentVersion] || null

            appData.update((version) => ({
                ...version,
                currentVersion,
                currentNotes,
            }))
        } catch (noteError) {
            handleError(noteError)
        }

        const update = await check()

        if (update) {
            let notes: App.PatchNotes | string = 'No release notes available.'
            try {
                const parsed = JSON.parse(update.body || '{}')
                if (parsed.features || parsed.fixes || parsed.known_issues) {
                    notes = parsed
                } else {
                    notes = update.body || 'No release notes available.'
                }
            } catch {
                notes = update.body || 'No release notes available.'
            }

            appData.update((version) => ({
                ...version,
                updateAvailable: true,
                pendingUpdate: {
                    version: update.version,
                    notes,
                },
            }))

            // TODO style this
            const userConfirmed = confirm(`Update to version ${update.version}?`)

            if (userConfirmed) {
                await update.downloadAndInstall()
                await relaunch()
            }
        }
        return {
            success: true,
            data: null,
        }
    } catch (error) {
        return formatError(error)
    }
}
