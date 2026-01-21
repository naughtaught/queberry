<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { listen } from '@tauri-apps/api/event'
    import type { UnlistenFn } from '@tauri-apps/api/event'
    import { onDestroy, onMount } from 'svelte'
    import { handleError, VideoControls, videoMetadata, videoState } from '$lib'
    import type { Api, Video } from '$lib'

    let backgroundColor = $state('bg-black')
    let is_completed = $state(false)
    let destroyListeners: (() => void) | undefined

    const setupListeners = async (): Promise<() => void> => {
        const unlisteners: UnlistenFn[] = []

        const timeUnlisten = await listen<Api.VideoState>('current-video-state', (event) => {
            $videoState = event.payload
        })
        unlisteners.push(timeUnlisten)

        const completeUnlisten = await listen<{ is_completed: boolean }>(
            'video-completed',
            (event) => {
                is_completed = event.payload.is_completed
            },
        )
        unlisteners.push(completeUnlisten)

        const metadataUnlisten = await listen<Api.Metadata>('video-metadata', (event) => {
            $videoMetadata = event.payload
        })
        unlisteners.push(metadataUnlisten)

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
                url: 'https://dn710604.ca.archive.org/0/items/BigBuckBunny_124/Content/big_buck_bunny_720p_surround.mp4',
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
    })
</script>

<div class="relative h-full w-full {backgroundColor}" id="app-container">
    <div class="group pointer-events-none absolute inset-0 z-20 h-full w-full">
        <div class="pointer-events-auto absolute bottom-0 left-0 w-full">
            <div class="opacity-0 transition-opacity group-hover:opacity-100">
                <VideoControls />
            </div>
        </div>
    </div>
</div>
