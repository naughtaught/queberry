<script lang="ts">
    import { goto } from '$app/navigation'
    import { resolve } from '$app/paths'
    import { listen, type UnlistenFn } from '@tauri-apps/api/event'
    import { onDestroy, onMount } from 'svelte'
    import {
        VideoControls,
        videoProperties,
        videoState,
        VideoHeader,
        defaultVideoProperties,
        defaultVideoState,
        sessionSettings,
        defaultSessionSettings,
        toggleFullscreen,
        invokeFunction,
        keyboardShortcuts,
        seekAmount,
        setVideoVolume,
        VideoOverlay,
        addPlaylistItem,
        videoMetadata,
        defaultVideoMetadata,
        navigatePlaylist,
        handleError,
        type Api,
    } from '$lib'

    const SUBTITLE_SHIFT_POSITION = 84

    let backgroundColor = $state('bg-black')
    let destroyListeners: (() => void) | undefined
    let isHoveringControls = $state(false)
    let hideTimeout: ReturnType<typeof setTimeout>
    let showCursor = $state(true)
    let currentModal = $state(null)
    let previousVolume = $state(0)
    let icon: string | number = $state('')
    let currentSubtitlePos = $derived($sessionSettings.subtitlePos)

    const setIcon = (value: string | number): void => {
        icon = value
        setTimeout(() => {
            icon = ''
        }, 500)
    }

    const setupListeners = async (): Promise<() => void> => {
        const unlisteners: UnlistenFn[] = []

        const timeUnlisten = await listen<Api.VideoState>('current-video-state', (event) => {
            $videoState = event.payload
        })
        unlisteners.push(timeUnlisten)

        const completeUnlisten = await listen<{ isCompleted: boolean }>('video-completed', (_event) => {
            // TODO mark complete function
            addPlaylistItem()
        })
        unlisteners.push(completeUnlisten)

        const videoPropertiesUnlisten = await listen<Api.VideoProperties>('video-properties', (event) => {
            $videoProperties = { ...event.payload }

            backgroundColor = 'bg-transparent'
        })
        unlisteners.push(videoPropertiesUnlisten)

        const shutdownUnlisten = await listen('video-shutdown', (_event) => {
            // TODO nav to previous page
            goto(resolve('/', {}))
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

            if (resp.error) throw resp.error

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

            if (resp.error) throw resp.error

            $sessionSettings.subtitlePos = resp.data.value
        } catch (error) {
            handleError(error, {
                context: 'getting the subtitle position failed',
            })
        }
    }

    onMount(async () => {
        destroyListeners = await setupListeners()
        getSubtitlePos()

        document.body.setAttribute('data-page', 'video')

        window.addEventListener('mousemove', resetCursorTimeout)
        window.addEventListener('keydown', handleKeydown)
        window.addEventListener('dblclick', handleMouseEvent)
        window.addEventListener('wheel', handleMouseEvent)
        window.addEventListener('contextmenu', handleMouseEvent)
        window.addEventListener('focus', shiftSubtitiles)

        resetCursorTimeout()
    })

    onDestroy(() => {
        if (destroyListeners) destroyListeners()
        document.body.removeAttribute('data-page')
        backgroundColor = 'bg-black'

        window.removeEventListener('mousemove', resetCursorTimeout)
        window.removeEventListener('keydown', handleKeydown)
        window.removeEventListener('dblclick', handleMouseEvent)
        window.removeEventListener('wheel', handleMouseEvent)
        window.removeEventListener('contextmenu', handleMouseEvent)
        window.removeEventListener('focus', shiftSubtitiles)

        $videoProperties = $defaultVideoProperties
        $videoState = $defaultVideoState
        $sessionSettings = $defaultSessionSettings
        $videoMetadata = $defaultVideoMetadata
    })

    const handleControlsMouseEnter = (): void => {
        isHoveringControls = true
        showCursor = true
        clearTimeout(hideTimeout)
    }

    const handleControlsMouseLeave = (): void => {
        isHoveringControls = false
        hideTimeout = setTimeout(() => {
            showCursor = false
        }, 2000)
        currentModal = null
    }

    const handleMouseEvent = async (e: MouseEvent): Promise<void> => {
        if (document.activeElement !== document.body || isHoveringControls) return

        const { type } = e as MouseEvent
        const { deltaY } = e as WheelEvent

        if (type === 'contextmenu') e.preventDefault()

        switch (type) {
            case 'contextmenu': {
                try {
                    const resp = await invokeFunction('toggle_play', { value: $videoState.isPaused })

                    if (resp.error) throw resp.error

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
        e.preventDefault()
        if (document.activeElement !== document.body || isHoveringControls) return

        const shortcut = $keyboardShortcuts.find((s) => s.code === e.code && (!s.shiftKey || s.shiftKey === e.shiftKey))

        if (!shortcut) return

        switch (shortcut.id) {
            case 'pause': {
                try {
                    const resp = await invokeFunction('toggle_play', { value: $videoState.isPaused })
                    if (resp.error) throw resp.error

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
                    const resp = await invokeFunction('seek', { value: $seekAmount })
                    if (resp.error) throw resp.error

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
                    const resp = await invokeFunction('seek', { value: -Math.abs($seekAmount) })
                    if (resp.error) throw resp.error

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
                    if (resp.error) throw resp.error

                    goto(resolve('/', {}))
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
            case 'cycleAudioTracks': {
                // TODO cycleAudioTracks
                break
            }
            case 'cycleSubtitleTracks': {
                // TODO cycleSubtitleTracks
                break
            }
            case 'increasePlaybackSpeed': {
                // TODO increasePlaybackSpeed
                break
            }
            case 'decreasePlaybackSpeed': {
                // TODO decreasePlaybackSpeed
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
        }, 2000)
    }
</script>

<div class="relative h-full w-full {backgroundColor} {showCursor ? '' : 'cursor-none'}" id="app-container">
    <div class="group pointer-events-none absolute inset-0 z-20 h-full w-full">
        <div
            class="pointer-events-auto absolute top-0 left-0 w-full"
            role="toolbar"
            tabindex="0"
            onmouseenter={handleControlsMouseEnter}
            onmouseleave={handleControlsMouseLeave}>
            <div class="opacity-0 transition-opacity group-hover:opacity-100">
                <VideoHeader />
            </div>
        </div>
        <div class=" absolute top-5 right-5">
            <VideoOverlay {icon} />
        </div>
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
