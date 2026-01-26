<script lang="ts">
    import { goto } from '$app/navigation'
    import { resolve } from '$app/paths'
    import { invoke } from '@tauri-apps/api/core'
    import { listen, type UnlistenFn } from '@tauri-apps/api/event'
    import { onDestroy, onMount } from 'svelte'
    import {
        handleError,
        VideoControls,
        videoMetadata,
        videoState,
        VideoHeader,
        defaultVideoMetadata,
        defaultVideoState,
        type Api,
    } from '$lib'

    let backgroundColor = $state('bg-black')
    let destroyListeners: (() => void) | undefined

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

        // 'D:/Media/Movies/The Raid (2012)',
        // 'https://dn710604.ca.archive.org/0/items/BigBuckBunny_124/Content/big_buck_bunny_720p_surround.mp4',

        try {
            const response: Api.ApiResponse = await invoke('load_video', {
                url: 'D:/Media/Movies/The Raid (2012)',
                userId: 1,
            })

            if (response.success) {
                backgroundColor = 'bg-transparent'
            } else {
                // TODO redirect
                handleError(response.error!)
            }

            destroyListeners = await setupListeners()
        } catch (error) {
            const errorDetail: Api.ErrorDetail = {
                code: 500,
                message: error instanceof Error ? error.message : String(error),
                stack: error instanceof Error ? error.stack : undefined,
            }
            // TODO redirect
            handleError(errorDetail)
        }
    })

    onDestroy(() => {
        if (destroyListeners) destroyListeners()
        document.body.removeAttribute('data-page')
        backgroundColor = 'bg-black'

        $videoMetadata = $defaultVideoMetadata
        $videoState = $defaultVideoState
    })
</script>

<div class="relative h-full w-full {backgroundColor}" id="app-container">
    <div class="group pointer-events-none absolute inset-0 z-20 h-full w-full">
        <div class="pointer-events-auto absolute top-0 left-0 w-full">
            <div class="opacity-0 transition-opacity group-hover:opacity-100">
                <VideoHeader />
            </div>
        </div>
        <div class="pointer-events-auto absolute bottom-0 left-0 w-full">
            <div class="opacity-0 transition-opacity group-hover:opacity-100">
                <VideoControls />
            </div>
        </div>
    </div>
</div>
