<script lang="ts">
    import { afterNavigate, beforeNavigate, goto } from '$app/navigation'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import VideoControls from '$lib/components/videoplayer/VideoControls.svelte'
    import VideoHeader from '$lib/components/videoplayer/VideoHeader.svelte'
    import VideoOverlay from '$lib/components/videoplayer/VideoOverlay.svelte'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { updateCachedMedia } from '$lib/functions/cache/updateCachedMedia'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { toggleFullscreen } from '$lib/functions/ui/toggleFullscreen'
    import { getTimings } from '$lib/functions/video/getTimings'
    import { navigatePlaylist } from '$lib/functions/video/navigatePlaylist'
    import { setVideoVolume } from '$lib/functions/video/setVideoVolume'
    import { videoCompleted } from '$lib/functions/video/videoCompleted'
    import { loadingStates, MIN_COMPLETION_TIME, previousPage } from '$lib/stores/app'
    import { defaultShuffleSettings, shuffleSettings } from '$lib/stores/pages'
    import { settings, user } from '$lib/stores/user'
    import {
        DEFAULT_VIDEO_METADATA,
        DEFAULT_VIDEO_PROPERTIES,
        DEFAULT_VIDEO_STATE,
        defaultSessionSettings,
        keyboardShortcuts,
        SEEK_AMOUNT,
        sessionSettings,
        videoMetadata,
        videoProperties,
        videoState,
    } from '$lib/stores/video'
    import type { Video } from '$lib/types/video'
    import type { Api } from '$lib/types/api'
    import { listen, type UnlistenFn } from '@tauri-apps/api/event'
    import { onDestroy, onMount } from 'svelte'

    const SUBTITLE_SHIFT_POSITION = 84
    const MOUSE_TIMEOUT = 2000
    const mainElement = $derived(document.getElementById('main-page') ?? null)
    let backgroundColor = $state('bg-black')
    let destroyListeners: (() => void) | undefined
    let isHoveringControls = $state(false)
    let hideTimeout: ReturnType<typeof setTimeout>
    let showCursor = $state(false)
    let currentModal = $state(null)
    let previousVolume = $state(0)
    let icon: string | number = $state('')
    let currentSubtitlePos = $derived($sessionSettings.subtitlePos)
    let iconTimeout: ReturnType<typeof setTimeout>
    let isBuffering = $derived(false)

    $effect(() => {
        if (
            $videoState.cacheTime === 0 &&
            $videoState.currentTime > 0 &&
            $videoState.currentTime < $videoProperties.duration
        ) {
            isBuffering = true
        } else {
            isBuffering = false
        }
    })

    const setIcon = (value: string | number): void => {
        clearTimeout(iconTimeout)
        icon = value
        iconTimeout = setTimeout(() => {
            icon = ''
        }, 500)
    }

    const setupListeners = async (): Promise<() => void> => {
        const unlisteners: UnlistenFn[] = []

        const timeUnlisten = await listen<Video.VideoState>('current-video-state', (event) => {
            $videoState = event.payload
        })
        unlisteners.push(timeUnlisten)

        const startUnlisten = await listen('video-started', (_event) => {
            $loadingStates.isVideoLoading = false
            backgroundColor = 'bg-transparent'
        })
        unlisteners.push(startUnlisten)

        const completeUnlisten = await listen<{ isCompleted: boolean }>('video-completed', (_event) => {
            videoCompleted()
        })
        unlisteners.push(completeUnlisten)

        const videoPropertiesUnlisten = await listen<Video.VideoProperties>('video-properties', (event) => {
            $videoProperties = { ...event.payload }

            const playlistItem = $videoMetadata.playlist.find(
                (x) => x.playlistIndex === $videoProperties.playlistPosition,
            )

            $videoMetadata = {
                ...playlistItem!,
                playlist: $videoMetadata.playlist,
            }

            $sessionSettings.isIntroTimingUpdated = false
            $sessionSettings.isRecapTimingUpdated = false
            $sessionSettings.isPreviewTimingUpdated = false
            $sessionSettings.isCreditTimingUpdated = false

            if ($videoProperties.duration && $videoMetadata.media) {
                const ONE_WEEK_MS = 7 * 24 * 60 * 60 * 1000
                const episode = $videoMetadata.episode
                const timingSource = $videoMetadata.media?.type === 'tv' ? episode : $videoMetadata.media
                const isStale = timingSource?.timings_last_updated
                    ? Date.now() - new Date(timingSource.timings_last_updated).getTime() > ONE_WEEK_MS
                    : true

                if (isStale) getTimings()
            }
        })
        unlisteners.push(videoPropertiesUnlisten)

        const shutdownUnlisten = await listen('video-shutdown', (event) => {
            const shutdownState = event.payload as Video.ShutdownState
            const metadataSnapshot = $videoMetadata
            const userSnapshot = $user

            $shuffleSettings = { ...defaultShuffleSettings }
            goto($previousPage)

            if (shutdownState && !shutdownState.isCompleted && shutdownState.currentTime > MIN_COMPLETION_TIME) {
                if (userSnapshot && metadataSnapshot.media) {
                    invokeFunction('api_upsert_user_media', {
                        postgresId: userSnapshot.postgresId,
                        token: userSnapshot.token,
                        data: {
                            postgresId: userSnapshot.postgresId,
                            mediaId: metadataSnapshot.media.id,
                            episodeId: metadataSnapshot.episode?.episode_id ?? null,
                            progress: shutdownState.currentTime,
                        },
                    })
                        .then((resp) => {
                            if (resp.success)
                                updateCachedMedia({
                                    ...metadataSnapshot.media,
                                    progress: shutdownState.currentTime,
                                    continue_watching_episode_id: metadataSnapshot.episode
                                        ? metadataSnapshot.episode.episode_id
                                        : null,
                                } as Api.MediaItem)
                        })
                        .catch((err) => handleError(err))
                }
            }
        })
        unlisteners.push(shutdownUnlisten)

        return () => {
            unlisteners.forEach((unlisten) => unlisten())
        }
    }

    $effect((): void => {
        if (!$sessionSettings || !videoProperties || !currentSubtitlePos) return
        if ($sessionSettings.subtitlePos === undefined || $videoProperties.currentSubtitleTrack?.id === 0) return

        if (isHoveringControls && currentSubtitlePos < SUBTITLE_SHIFT_POSITION) return
        if (!isHoveringControls && currentSubtitlePos === $sessionSettings.subtitlePos) return

        shiftSubtitiles()
    })

    const shiftSubtitiles = async (): Promise<void> => {
        const shiftPositionTo = isHoveringControls ? SUBTITLE_SHIFT_POSITION : $sessionSettings.subtitlePos
        try {
            const resp = await invokeFunction('set_subtitle_pos', { value: shiftPositionTo })

            if (!resp.success) throw resp.error

            currentSubtitlePos = resp.data.value
        } catch (error) {
            handleError(error, {
                context: 'shifting the subtitles failed',
            })
        }
    }

    const getSubtitlePos = async (): Promise<void> => {
        try {
            const resp = await invokeFunction('get_subtitle_pos', {})

            if (!resp.success) throw resp.error

            $sessionSettings.subtitlePos = resp.data.value
        } catch (error) {
            handleError(error, {
                context: 'getting the subtitle position failed',
            })
        }
    }

    afterNavigate((navigation) => {
        if (!navigation.from || navigation.from.url.pathname === '/video') return

        if (!navigation.from?.url) {
            $previousPage = '/'
            return
        }

        $previousPage = navigation.from.url.pathname + navigation.from.url.search
    })

    beforeNavigate(({ to }) => {
        if (window.location.pathname.startsWith('/video') && to?.url.pathname !== '/video') {
            history.replaceState(null, '', to?.url.pathname || '/')
        }
    })

    const handleMouseLeaveWindow = (): void => {
        showCursor = true
        if (mainElement) mainElement.classList.remove('cursor-none')
        clearTimeout(hideTimeout)
    }

    const handleWindowBlur = (): void => {
        showCursor = true
        clearTimeout(hideTimeout)
        if (mainElement) mainElement.classList.remove('cursor-none')
    }

    const handleWindowFocus = (): void => {
        if (mainElement) mainElement.classList.remove('cursor-none')
        resetCursorTimeout()
        shiftSubtitiles()
    }

    onMount(async () => {
        document.body.setAttribute('data-page', 'video')

        $sessionSettings.volume = $settings.volume

        destroyListeners = await setupListeners()
        getSubtitlePos()

        window.addEventListener('mousemove', resetCursorTimeout)
        window.addEventListener('keydown', handleKeydown)
        window.addEventListener('dblclick', handleMouseEvent)
        window.addEventListener('wheel', handleMouseEvent)
        window.addEventListener('contextmenu', handleMouseEvent)
        window.addEventListener('blur', handleWindowBlur)
        window.addEventListener('focus', handleWindowFocus)
        window.addEventListener('mouseleave', handleMouseLeaveWindow)

        resetCursorTimeout()
    })

    onDestroy(() => {
        if (destroyListeners) destroyListeners()

        if (hideTimeout) clearTimeout(hideTimeout)
        if (iconTimeout) clearTimeout(iconTimeout)

        document.body.removeAttribute('data-page')
        backgroundColor = 'bg-black'

        window.removeEventListener('mousemove', resetCursorTimeout)
        window.removeEventListener('keydown', handleKeydown)
        window.removeEventListener('dblclick', handleMouseEvent)
        window.removeEventListener('wheel', handleMouseEvent)
        window.removeEventListener('contextmenu', handleMouseEvent)
        window.removeEventListener('blur', handleWindowBlur)
        window.removeEventListener('focus', handleWindowFocus)
        window.removeEventListener('mouseleave', handleMouseLeaveWindow)

        $videoProperties = $DEFAULT_VIDEO_PROPERTIES
        $videoState = $DEFAULT_VIDEO_STATE
        $sessionSettings = $defaultSessionSettings
        $videoMetadata = $DEFAULT_VIDEO_METADATA
    })

    const handleControlsMouseEnter = (): void => {
        isHoveringControls = true
        showCursor = true
        clearTimeout(hideTimeout)
    }

    const handleControlsMouseLeave = (): void => {
        isHoveringControls = false
        if (hideTimeout) clearTimeout(hideTimeout)
        hideTimeout = setTimeout(() => {
            showCursor = false
        }, MOUSE_TIMEOUT)
        currentModal = null
    }

    const handleMouseEvent = async (e: MouseEvent): Promise<void> => {
        if (!$user) return
        if (document.activeElement !== document.body || isHoveringControls) return

        const { type } = e as MouseEvent
        const { deltaY } = e as WheelEvent

        if (type === 'contextmenu') e.preventDefault()

        switch (type) {
            case 'contextmenu': {
                try {
                    const resp = await invokeFunction('toggle_play', { value: $videoState.isPaused })

                    if (!resp.success) throw resp.error

                    $videoState.isPaused = resp.data.value

                    const playState = $videoState.isPaused ? 'paused' : 'playing'

                    setIcon(playState)
                } catch (error) {
                    handleError(error, {
                        context: 'toggling the play state from mouse bindings failed',
                    })
                }
                break
            }
            case 'wheel': {
                if (deltaY < 0 && $sessionSettings.volume < 100) {
                    $sessionSettings.volume += 1
                } else if (deltaY > 0 && $sessionSettings.volume > 0) {
                    $sessionSettings.volume -= 1
                }

                const { newValue, previousValue } = await setVideoVolume(
                    $sessionSettings.volume,
                    previousVolume,
                    previousVolume,
                )

                $sessionSettings.volume = newValue
                previousVolume = previousValue

                const currentVolume = $sessionSettings.volume === 0 ? 'muted' : $sessionSettings.volume

                setIcon(currentVolume)
                break
            }
            case 'dblclick': {
                if (e.detail === 2) toggleFullscreen()
            }
        }
    }

    const handleKeydown = async (e: KeyboardEvent): Promise<void> => {
        if (!$user) return
        if (document.activeElement !== document.body || isHoveringControls) return

        const shortcut = $keyboardShortcuts.find((s) => s.code === e.code && (!s.shiftKey || s.shiftKey === e.shiftKey))

        if (!shortcut) return

        e.preventDefault()

        switch (shortcut.id) {
            case 'togglePlay': {
                try {
                    const resp = await invokeFunction('toggle_play', { value: $videoState.isPaused })
                    if (!resp.success) throw resp.error

                    $videoState.isPaused = resp.data.value
                    const playState = $videoState.isPaused ? 'paused' : 'playing'

                    setIcon(playState)
                } catch (error) {
                    handleError(error, {
                        context: 'toggling the play state from keybindings failed',
                    })
                }
                break
            }
            case 'fullscreen': {
                toggleFullscreen()
                break
            }
            case 'mute': {
                const { newValue, previousValue } = await setVideoVolume(
                    $sessionSettings.volume === 0 ? previousVolume : 0,
                    $sessionSettings.volume,
                    previousVolume,
                )

                $sessionSettings.volume = newValue
                previousVolume = previousValue

                const currentVolume = $sessionSettings.volume === 0 ? 'muted' : $sessionSettings.volume

                setIcon(currentVolume)
                break
            }
            case 'forward': {
                try {
                    const resp = await invokeFunction('seek', { value: $SEEK_AMOUNT })
                    if (!resp.success) throw resp.error

                    setIcon('forward')
                } catch (error) {
                    handleError(error, {
                        context: `seeking forward from keybindings failed`,
                    })
                }

                break
            }
            case 'rewind': {
                try {
                    const resp = await invokeFunction('seek', { value: -Math.abs($SEEK_AMOUNT) })
                    if (!resp.success) throw resp.error

                    setIcon('rewind')
                } catch (error) {
                    handleError(error, {
                        context: `seeking backward from keybindings failed`,
                    })
                }

                break
            }
            case 'close': {
                try {
                    const resp = await invokeFunction('close_video_player', {})
                    if (!resp.success) throw resp.error

                    goto($previousPage)
                } catch (error) {
                    handleError(error, {
                        context: `closing the video from keybindings failed`,
                    })
                }
                break
            }
            case 'volumeUp': {
                if ($sessionSettings.volume >= 100) return
                const { newValue, previousValue } = await setVideoVolume(
                    ($sessionSettings.volume += 1),
                    $sessionSettings.volume,
                    previousVolume,
                )

                $sessionSettings.volume = newValue
                previousVolume = previousValue

                setIcon($sessionSettings.volume)
                break
            }
            case 'volumeDown': {
                if ($sessionSettings.volume === 0) return
                const { newValue, previousValue } = await setVideoVolume(
                    ($sessionSettings.volume -= 1),
                    $sessionSettings.volume,
                    previousVolume,
                )

                $sessionSettings.volume = newValue
                previousVolume = previousValue

                const currentVolume = $sessionSettings.volume === 0 ? 'muted' : $sessionSettings.volume

                setIcon(currentVolume)
                break
            }
            case 'playlistNext': {
                const hasNextPlaylistItem = $videoProperties.playlistPosition < $videoProperties.playlistCount - 1
                if (hasNextPlaylistItem) await navigatePlaylist('next')
                break
            }
            case 'playlistPrevious': {
                const hasPreviousPlaylistItem = $videoProperties.playlistPosition > 0
                if (hasPreviousPlaylistItem) await navigatePlaylist('previous')
                break
            }
        }
    }

    function resetCursorTimeout(): void {
        showCursor = true
        clearTimeout(hideTimeout)
        hideTimeout = setTimeout(() => {
            showCursor = false
        }, MOUSE_TIMEOUT)
    }

    $effect.pre(() => {
        if (mainElement) mainElement.classList.toggle('cursor-none', !showCursor)
    })
</script>

<div class="relative h-full w-full {backgroundColor}">
    <div class="group pointer-events-none absolute inset-0 z-20 h-full w-full">
        <div
            class="pointer-events-auto absolute top-0 left-0 w-full"
            role="toolbar"
            tabindex="0"
            onmouseenter={handleControlsMouseEnter}
            onmouseleave={handleControlsMouseLeave}>
            <div class="opacity-0 transition-opacity group-hover:opacity-100">
                <VideoHeader bind:currentModal />
            </div>
        </div>
        <div class="absolute top-5 right-5">
            <VideoOverlay {icon} />
        </div>
        {#if isBuffering}
            <div class="absolute inset-0 z-40 flex h-screen w-screen items-center justify-center">
                <LoadingSpinner size="max-w-8" />
            </div>
        {/if}
        <div
            class="pointer-events-auto absolute bottom-0 left-0 w-full"
            role="toolbar"
            tabindex="0"
            onmouseenter={handleControlsMouseEnter}
            onmouseleave={handleControlsMouseLeave}>
            <div class="opacity-0 transition-opacity group-hover:opacity-100">
                <VideoControls bind:currentModal bind:previousVolume />
            </div>
        </div>
    </div>
</div>
