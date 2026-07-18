import { directories, loadingStates } from '$lib/stores/app'
import { get } from 'svelte/store'
import { invokeFunction } from '../api/invokeFunction'
import type { App } from '$lib/types/app'
import { handleError } from '../errors/errorHandling'

export const getLocalMedia = async (): Promise<void> => {
    loadingStates.update((states) => ({
        ...states,
        isLocalMediaLoading: true,
    }))

    const tvDirectory = get(directories).tv
    const movieDirectory = get(directories).movies

    try {
        if (tvDirectory || movieDirectory) {
            if (tvDirectory === movieDirectory) {
                try {
                    const mediaDir = await invokeFunction('scan_local_folder', {
                        directory: tvDirectory,
                    })
                    if (!mediaDir.success) throw mediaDir.error
                } catch (error) {
                    const err = error as App.ErrorDetail
                    if (err.message.includes('I/O error: The system cannot find the path specified')) {
                        directories.update((dirs) => ({
                            ...dirs,
                            tv: null,
                            movies: null,
                        }))
                    } else {
                        handleError(error)
                    }
                }
            } else {
                const results = await Promise.allSettled([
                    invokeFunction('scan_local_folder', {
                        directory: tvDirectory,
                    }),
                    invokeFunction('scan_local_folder', {
                        directory: movieDirectory,
                    }),
                ])

                results.forEach((result, index) => {
                    if (result.status === 'rejected') {
                        const error = result.reason
                        if (error.message.includes('I/O error: The system cannot find the path specified')) {
                            directories.update((dirs) => ({
                                ...dirs,
                                [index === 0 ? 'tv' : 'movies']: null,
                            }))
                        } else {
                            handleError(error)
                        }
                    } else if (result.status === 'fulfilled') {
                        const media = result.value
                        if (!media.success) {
                            handleError(media.error)
                        }
                    }
                })
            }
        }
    } catch (error) {
        handleError(error)
    } finally {
        loadingStates.update((states) => ({
            ...states,
            isLocalMediaLoading: false,
        }))
    }
}
