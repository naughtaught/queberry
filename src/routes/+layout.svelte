<script lang="ts">
    import '../app.css'
    import { page } from '$app/state'
    import { loadingStates, modals, parentalControlsAreEnabled } from '$lib/stores/app'
    import VideoLoadingScreen from '$lib/components/ui/VideoLoadingScreen.svelte'
    import { settings, user } from '$lib/stores/user'
    import UserModal from '$lib/components/modals/UserModal.svelte'
    import FiltersModal from '$lib/components/modals/FiltersModal.svelte'
    import DragBar from '$lib/components/bars/DragBar.svelte'
    import NavSideBar from '$lib/components/bars/NavSideBar.svelte'
    import { onDestroy, onMount } from 'svelte'
    import { invoke } from '@tauri-apps/api/core'
    import ToastNotification from '$lib/components/modals/ToastNotification.svelte'
    import SearchModal from '$lib/components/modals/SearchModal.svelte'
    import { afterNavigate, goto } from '$app/navigation'
    import { defaultShuffleSettings, shuffleSettings } from '$lib/stores/pages'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { createError, handleError } from '$lib/functions/errors/errorHandling'
    import { getImagePath } from '$lib/functions/ui/getImagePath'
    import { DEFAULT_VIDEO_METADATA, videoMetadata, videoState } from '$lib/stores/video'
    import { downloadsInProgress } from '$lib/stores/plugins'
    import { listen } from '@tauri-apps/api/event'
    import type { Sql } from '$lib/types/sql'
    import { closeVideoPlayer } from '$lib/functions/video/closeVideoPlayer'

    const { children } = $props()

    const screensaverTimeout = 10 * 60000 // 10 minutes

    let inactivityTimer: ReturnType<typeof setTimeout> | null = $state(null)
    const inactivityTimeout = $derived($settings.screensaverTimeout * 60000)
    let screensaverRefreshInterval: ReturnType<typeof setTimeout> | null = $state(null)
    let isUserInactive = $state(false)
    let screensaverPath: string | null = $state(null)
    let debounceTimer: ReturnType<typeof setTimeout> | null = null
    let cleanupDownloadsListeners: (() => void) | undefined = $state(undefined)
    let lastUserId: number | null = $state(null)
    const pausedState = $derived($videoState.isPaused)
    let unlistenVideoLoadFailed: (() => void) | undefined
    let cleanupVideoPlayerListeners: (() => void) | undefined = $state(undefined)

    $effect(() => {
        if (page.url.pathname !== '/video') return

        if (pausedState) {
            resetInactivityTimer()
        } else {
            clearScreensaverTimers()
            if (isUserInactive) {
                isUserInactive = false
                screensaverPath = null
            }
        }
    })

    $effect(() => {
        if ($modals.user || $modals.trailer || $modals.filters) {
            document.body.classList.add('overflow-hidden')
        } else {
            document.body.classList.remove('overflow-hidden')
        }
    })

    $effect(() => {
        const currentUserId = $user?.id ?? null

        if (currentUserId === lastUserId) return

        if (cleanupDownloadsListeners) {
            cleanupDownloadsListeners()
            cleanupDownloadsListeners = undefined
        }

        lastUserId = currentUserId

        if (!currentUserId) {
            $downloadsInProgress = []
            return
        }

        downloadsListenersSetup().then((cleanup) => {
            if (cleanup) cleanupDownloadsListeners = cleanup
        })

        return () => {
            if (cleanupDownloadsListeners) {
                cleanupDownloadsListeners()
                cleanupDownloadsListeners = undefined
            }
        }
    })

    const processQueue = async (): Promise<void> => {
        try {
            if (!$user) throw createError('Missing User', 401, { log: false })

            const response = await invokeFunction('process_download_queue', { userId: $user.id })
            if (!response.success) throw response.error
        } catch (error) {
            handleError(error)
        }
    }

    const downloadsListenersSetup = async (): Promise<(() => void) | undefined> => {
        try {
            if (!$user) throw createError('Missing User', 401, { log: false })

            const cleanupResult = await invokeFunction('clear_completed_downloads', { userId: $user.id, hours: 48 })
            if (!cleanupResult.success) throw cleanupResult.error

            const resp = await invokeFunction('get_all_downloads', { userId: $user.id })
            if (!resp.success) throw resp.error

            $downloadsInProgress = resp.data ?? []

            await invokeFunction('process_download_queue', { userId: $user.id })

            let unlistenProgress: (() => void) | undefined
            let unlistenError: (() => void) | undefined
            let unlistenCompleted: (() => void) | undefined
            let unlistenFailed: (() => void) | undefined
            let unlistenQueued: (() => void) | undefined
            let unlistenWarning: (() => void) | undefined

            const setup = async (): Promise<void> => {
                unlistenProgress = await listen<Sql.DownloadProgress>('download_progress', (event) => {
                    const { fileIndex, uuid, progress, speed, eta } = event.payload

                    downloadsInProgress.update((downloads) =>
                        downloads.map((d) =>
                            d.link.uuid === uuid ? { ...d, fileIndex, progress, speed, eta, status: 'downloading' } : d,
                        ),
                    )
                })

                unlistenError = await listen<Sql.DownloadError>('download_error', (event) => {
                    const { uuid, message } = event.payload

                    downloadsInProgress.update((downloads) =>
                        downloads.map((d) => (d.link.uuid === uuid ? { ...d, status: 'error', error: message } : d)),
                    )

                    handleError(message)
                })

                unlistenCompleted = await listen<{ uuid: string; filename: string }>(
                    'download_completed',
                    async (event) => {
                        const { uuid } = event.payload

                        downloadsInProgress.update((downloads) =>
                            downloads.map((d) =>
                                d.link.uuid === uuid ? { ...d, status: 'completed', progress: 100 } : d,
                            ),
                        )

                        setTimeout(() => {
                            downloadsInProgress.update((downloads) => downloads.filter((d) => d.link.uuid !== uuid))
                        }, 3000)

                        await processQueue()
                    },
                )

                unlistenFailed = await listen<{ uuid: string; filename: string; error: string }>(
                    'download_failed',
                    (event) => {
                        const { uuid, filename, error } = event.payload

                        if (error.includes('cancelled') || error.includes('Cancelled')) {
                            downloadsInProgress.update((downloads) =>
                                downloads.map((d) => (d.link.uuid === uuid ? { ...d, status: 'cancelled' } : d)),
                            )

                            setTimeout(() => {
                                downloadsInProgress.update((downloads) => downloads.filter((d) => d.link.uuid !== uuid))
                            }, 2000)

                            processQueue()
                            return
                        }

                        handleError(`Download failed: ${filename} - ${error}`)

                        downloadsInProgress.update((downloads) =>
                            downloads.map((d) => (d.link.uuid === uuid ? { ...d, status: 'failed', error } : d)),
                        )

                        processQueue()
                    },
                )

                unlistenQueued = await listen<{ uuid: string }>('download_queued', (event) => {
                    const { uuid } = event.payload

                    downloadsInProgress.update((downloads) =>
                        downloads.map((d) => (d.link.uuid === uuid ? { ...d, status: 'queued' } : d)),
                    )
                })

                unlistenWarning = await listen<{ type: string; message: string }>('download_warning', (event) => {
                    const { type, message } = event.payload
                    handleError(`Download warning (${type}): ${message}`, { log: false })
                })
            }

            await setup()

            return () => {
                unlistenProgress?.()
                unlistenError?.()
                unlistenCompleted?.()
                unlistenFailed?.()
                unlistenQueued?.()
                unlistenWarning?.()
            }
        } catch (error) {
            handleError(error)
            return undefined
        }
    }

    const videoPlayerListenersSetup = async (): Promise<() => void> => {
        unlistenVideoLoadFailed = await listen<number>('playback-error', async (event) => {
            handleError(`Video failed to load and could not be recovered (code ${event.payload}).`, {
                context: 'video playback failed',
            })

            await closeVideoPlayer()

            $videoMetadata = $DEFAULT_VIDEO_METADATA

            loadingStates.update((states) => ({
                ...states,
                isVideoLoading: false,
            }))

            if (page.url.pathname === '/video') goto('/')
        })

        return () => {
            unlistenVideoLoadFailed?.()
        }
    }

    onMount(() => {
        invoke('show_window')
        if ($settings.screensaverTimeout > 0) {
            resetInactivityTimer()
            window.addEventListener('mousemove', resetInactivityTimer)
            window.addEventListener('mousedown', resetInactivityTimer)
            window.addEventListener('keypress', resetInactivityTimer)
            window.addEventListener('touchmove', resetInactivityTimer)
            window.addEventListener('scroll', resetInactivityTimer)
        }

        videoPlayerListenersSetup().then((cleanup) => {
            cleanupVideoPlayerListeners = cleanup
        })
    })

    const clearScreensaverTimers = (): void => {
        if (inactivityTimer) {
            clearTimeout(inactivityTimer)
            inactivityTimer = null
        }
        if (screensaverRefreshInterval) {
            clearInterval(screensaverRefreshInterval)
            screensaverRefreshInterval = null
        }
    }

    const resetScreensaverTimer = (): void => {
        if (!$videoState.isPaused && page.url.pathname === '/video') return

        if (isUserInactive) {
            if ($parentalControlsAreEnabled) $user = null
            isUserInactive = false
            screensaverPath = null
        }

        clearScreensaverTimers()

        if (inactivityTimeout === 0) return

        inactivityTimer = setTimeout(async () => {
            if (!$videoState.isPaused && page.url.pathname === '/video') return

            await getRandomBackdrop()

            if (!inactivityTimer) return

            if (screensaverPath) {
                isUserInactive = true
                screensaverRefreshInterval = setInterval(getRandomBackdrop, screensaverTimeout)
            }
        }, inactivityTimeout)
    }

    const getRandomBackdrop = async (): Promise<void> => {
        if (!$user) return
        try {
            const resp = await invokeFunction('api_fetch_random_backdrop', {
                postgresId: $user?.postgresId,
                token: $user?.token,
            })

            if (!resp.success) throw resp.error

            screensaverPath = getImagePath(resp.data, 'original')
        } catch (error) {
            handleError(error)
            if (screensaverRefreshInterval) {
                clearInterval(screensaverRefreshInterval)
                screensaverRefreshInterval = null
            }
        }
    }

    const resetInactivityTimer = (): void => {
        if (debounceTimer) clearTimeout(debounceTimer)
        debounceTimer = setTimeout(resetScreensaverTimer, 50)
    }

    afterNavigate((navigation) => {
        if (navigation.to?.route.id !== '/video') $shuffleSettings = { ...defaultShuffleSettings }
        clearScreensaverTimers()
        if (isUserInactive) {
            isUserInactive = false
            screensaverPath = null
        }
    })

    onDestroy(() => {
        if (inactivityTimer) clearTimeout(inactivityTimer)
        if (screensaverRefreshInterval) clearInterval(screensaverRefreshInterval)
        if (debounceTimer) clearTimeout(debounceTimer)
        window.removeEventListener('mousemove', resetInactivityTimer)
        window.removeEventListener('mousedown', resetInactivityTimer)
        window.removeEventListener('keypress', resetInactivityTimer)
        window.removeEventListener('touchmove', resetInactivityTimer)
        window.removeEventListener('scroll', resetInactivityTimer)
        cleanupVideoPlayerListeners?.()
    })
</script>

{#if $loadingStates.isVideoLoading}
    <VideoLoadingScreen />
{/if}

{#if $modals.user || !$user}
    <UserModal />
{/if}

{#if $modals.filters}
    <FiltersModal />
{/if}

{#if $modals.search}
    <SearchModal />
{/if}

{#if screensaverPath}
    <img
        class="fixed inset-0 z-40 flex h-full w-full bg-backgroundColor brightness-50"
        src={screensaverPath}
        alt="Screensaver" />
{/if}

<ToastNotification />

<main class="min-h-screen {!page.url.pathname.includes('video') ? '' : 'pt-0'} flex" id="main-page">
    {#if page.url.pathname !== '/video'}
        <DragBar />
        <NavSideBar />
    {/if}
    {@render children()}
</main>
