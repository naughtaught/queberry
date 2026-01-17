<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { onDestroy, onMount } from 'svelte'
    import { handleError, VideoControls } from '$lib'
    import type { Api } from '$lib'

    let isPaused = $state(false)
    let backgroundColor = $state('bg-black')
    let currentTime = $state(2)

    onMount(async (): Promise<void> => {
        document.body.setAttribute('data-page', 'video')

        try {
            const response: Api.ApiResponse = await invoke('load_video', {
                url: 'D:/Media/Movies/The Raid (2012)',
            })

            if (response.success) {
                backgroundColor = 'bg-transparent'
            } else {
                handleError(response.error!)
            }
        } catch (error) {
            const errorDetail: Api.ErrorDetail = {
                code: 500,
                message: error instanceof Error ? error.message : String(error),
                stack: error instanceof Error ? error.stack : undefined,
            }
            handleError(errorDetail)
        }
    })

    onDestroy(() => {
        document.body.removeAttribute('data-page')
        backgroundColor = 'bg-black'
    })
</script>

<div class="relative h-full w-full {backgroundColor}" id="app-container">
    <div class="group pointer-events-none absolute inset-0 z-20 h-full w-full">
        <div class="pointer-events-auto absolute bottom-0 left-0 w-full">
            <div class="opacity-0 transition-opacity group-hover:opacity-100">
                <VideoControls bind:isPaused bind:currentTime />
            </div>
        </div>
    </div>
</div>
