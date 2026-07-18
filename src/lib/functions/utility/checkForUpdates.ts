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
                showUpdateModal: true,
                pendingUpdate: {
                    version: update.version,
                    notes,
                },
            }))

            const userConfirmed = await new Promise<boolean>((resolve) => {
                appData.update((state) => ({
                    ...state,
                    updateResolver: resolve,
                }))
            })

            appData.update((state) => ({
                ...state,
                updateResolver: null,
            }))

            if (userConfirmed) {
                appData.update((state) => ({
                    ...state,
                    isDownloading: true,
                    downloadProgress: 0,
                }))

                try {
                    let totalSize = 0
                    let downloadedSoFar = 0

                    await update.downloadAndInstall((event) => {
                        switch (event.event) {
                            case 'Started':
                                totalSize = event.data.contentLength ?? 0
                                break
                            case 'Progress':
                                downloadedSoFar += event.data.chunkLength
                                if (totalSize > 0) {
                                    const progress = Math.min(Math.round((downloadedSoFar / totalSize) * 100), 100)
                                    appData.update((state) => ({
                                        ...state,
                                        downloadProgress: progress,
                                    }))
                                }
                                break
                            case 'Finished':
                                appData.update((state) => ({
                                    ...state,
                                    downloadProgress: 100,
                                    isDownloading: false,
                                    isInstalling: true,
                                }))
                                break
                        }
                    })

                    await relaunch()
                } catch (downloadError) {
                    appData.update((state) => ({
                        ...state,
                        isDownloading: false,
                        downloadProgress: 0,
                    }))
                    throw downloadError
                }
            }
        }
        return {
            success: true,
            data: null,
        }
    } catch (error) {
        appData.update((state) => ({
            ...state,
            showUpdateModal: false,
            updateResolver: null,
            isDownloading: false,
            downloadProgress: 0,
        }))
        return formatError(error)
    }
}
