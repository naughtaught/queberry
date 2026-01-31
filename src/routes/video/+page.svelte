<script lang="ts">
    import { goto } from '$app/navigation'
    import { resolve } from '$app/paths'
    import { listen, type UnlistenFn } from '@tauri-apps/api/event'
    import { onDestroy, onMount } from 'svelte'
    import {
        VideoControls,
        videoMetadata,
        videoState,
        VideoHeader,
        defaultVideoMetadata,
        defaultVideoState,
        sessionSettings,
        defaultSessionSettings,
        toggleFullscreen,
        invokeFunction,
        keyboardShortcuts,
        seekAmount,
        setVideoVolume,
        VideoOverlay,
        type Api,
    } from '$lib'

    let backgroundColor = $state('bg-black')
    let destroyListeners: (() => void) | undefined
    let isHoveringControls = $state(false)
    let hideTimeout: ReturnType<typeof setTimeout>
    let showCursor = $state(true)
    let currentModal = $state(null)
    let previousVolume = $state(0)
    let icon: string | number = $state('')

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
            // TODO
        })
        unlisteners.push(completeUnlisten)

        const metadataUnlisten = await listen<Api.Metadata>('video-metadata', (event) => {
            $videoMetadata = { ...event.payload }
        })
        unlisteners.push(metadataUnlisten)

        const shutdownUnlisten = await listen('video-shutdown', (_event) => {
            // TODO nav to previous page
            goto(resolve('/', {}))
        })
        unlisteners.push(shutdownUnlisten)

        return () => {
            unlisteners.forEach((unlisten) => unlisten())
        }
    }

    onMount(async () => {
        document.body.setAttribute('data-page', 'video')
        backgroundColor = 'bg-transparent'

        window.addEventListener('mousemove', resetCursorTimeout)
        window.addEventListener('keydown', handleKeydown)
        window.addEventListener('dblclick', handleMouseEvent)
        window.addEventListener('wheel', handleMouseEvent)
        window.addEventListener('contextmenu', handleMouseEvent)

        resetCursorTimeout()

        destroyListeners = await setupListeners()
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

        $videoMetadata = $defaultVideoMetadata
        $videoState = $defaultVideoState
        $sessionSettings = $defaultSessionSettings
    })

    const handleControlsMouseEnter = (): void => {
        isHoveringControls = true
        showCursor = true
        // TODO check if subtitles position is above the control bar and if so don't emit
        // EventsEmit('ToggleSubtitleShift', true)
        clearTimeout(hideTimeout)
    }

    const handleControlsMouseLeave = (): void => {
        isHoveringControls = false
        // TODO check if subtitles position is above the control bar by default and if so don't emit
        // EventsEmit('ToggleSubtitleShift', false)
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
                const response = await invokeFunction('toggle_play', { value: $videoState.isPaused })
                if (response.success) {
                    $videoState.isPaused = response.data.value

                    const playState = $videoState.isPaused ? 'paused' : 'playing'

                    setIcon(playState)
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

        console.log(shortcut)

        switch (shortcut.id) {
            case 'pause': {
                const response = await invokeFunction('toggle_play', { value: $videoState.isPaused })
                if (response.success) {
                    $videoState.isPaused = response.data.value
                    const playState = $videoState.isPaused ? 'paused' : 'playing'

                    setIcon(playState)
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
                const response = await invokeFunction('seek', { value: $seekAmount })
                if (response.success) setIcon('forward')
                break
            }
            case 'rewind': {
                const response = await invokeFunction('seek', { value: -Math.abs($seekAmount) })
                if (response.success) setIcon('rewind')
                break
            }
            case 'close': {
                const response = await invokeFunction('close_video_player', {})
                if (response.success) {
                    // TODO Navigation from here
                    goto(resolve('/', {}))
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
            case 'testModifiers': {
                // TODO
                console.log('test modifiers')
                break
            }
            case 'cycleAudioTracks': {
                // TODO
                console.log('cycleAudioTracks')
                break
            }
            case 'cycleSubtitleTracks': {
                // TODO
                console.log('cycleSubtitleTracks')

                break
            }
            case 'increasePlaybackSpeed': {
                // TODO
                console.log('increasePlaybackSpeed')
                break
            }
            case 'decreasePlaybackSpeed': {
                // TODO
                console.log('decreasePlaybackSpeed')
                break
            }
            case 'playlistNext': {
                // TODO
                console.log('playlistNext')
                break
            }
            case 'playlistPrevious': {
                // TODO
                console.log('playlistPrevious')
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
