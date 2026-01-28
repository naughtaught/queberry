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
        type Api,
    } from '$lib'

    let backgroundColor = $state('bg-black')
    let destroyListeners: (() => void) | undefined
    let isHoveringControls = $state(false)
    let hideTimeout: ReturnType<typeof setTimeout>
    let showCursor = $state(true)
    let currentModal = $state(null)
    let previousVolume = $derived($sessionSettings.volume)

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

        window.addEventListener('mousemove', resetCursorTimeout)
        resetCursorTimeout()
        window.addEventListener('keydown', handleKeydown)
        window.addEventListener('dblclick', handleDoubleClick)

        destroyListeners = await setupListeners()

        // 'D:/Media/Movies/The Raid (2012)',
        // 'https://dn710604.ca.archive.org/0/items/BigBuckBunny_124/Content/big_buck_bunny_720p_surround.mp4',

        const response = await invokeFunction('load_video', {
            value: {
                // Add the 'value' wrapper!
                url: 'D:/Media/Movies/The Raid (2012)',
                userId: 1,
            },
        })

        if (response.success) {
            backgroundColor = 'bg-transparent'
        } else {
            // TODO redirect
            goto(resolve('/', {}))
        }
    })

    onDestroy(() => {
        if (destroyListeners) destroyListeners()
        document.body.removeAttribute('data-page')
        backgroundColor = 'bg-black'

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

    const handleKeydown = async (e: KeyboardEvent): Promise<void> => {
        if (document.activeElement !== document.body || isHoveringControls) return
        const shortcut = $keyboardShortcuts.find((x) => x.key === e.code)

        console.log(e.code)

        if (!shortcut) return
        e.preventDefault()

        if (shortcut.id === 'pause') {
            const response = await invokeFunction('toggle_play', { value: $videoState.isPaused })
            if (response.success) $videoState.isPaused = response.data.value
        }

        if (shortcut.id === 'fullscreen') toggleFullscreen()

        if (shortcut.id === 'mute') {
            const response = await invokeFunction('set_volume', {
                value: $sessionSettings.volume === 0 ? previousVolume : 0,
            })
            if (response.success) {
                previousVolume = response.data.value
            } else {
                $sessionSettings.volume = previousVolume
            }
        }

        // TODO
    }

    const handleDoubleClick = (e: MouseEvent): void => {
        if (isHoveringControls) return
        if (e.detail === 2) toggleFullscreen()
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
