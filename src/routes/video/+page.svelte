<script lang="ts">
    import VideoControls from '$lib/components/videoplayer/VideoControls.svelte'
    import { invoke } from '@tauri-apps/api/core'
    import { onDestroy, onMount } from 'svelte'

    let isPaused = $state(false)
    let backgroundColor = $state('bg-black')

    onMount(async () => {
        document.body.setAttribute('data-page', 'video')
        backgroundColor = 'bg-transparent'

        try {
            const response = await invoke('load_video', {
                url: 'D:/Media/Movies/The Raid (2012)',
            })
            console.log('Video load response:', response)

            if (response) console.log(response)
        } catch (error) {
            console.error('Error invoking load_video:', error)
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
                <VideoControls bind:isPaused />
            </div>
        </div>
    </div>
</div>
