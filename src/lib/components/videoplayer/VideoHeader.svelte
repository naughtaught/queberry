<script lang="ts">
    import FullscreenIcon from 'virtual:icons/material-symbols/fullscreen'
    import ExitFullscreenIcon from 'virtual:icons/material-symbols/fullscreen-exit'
    import MinimizeIcon from 'virtual:icons/material-symbols/minimize'
    import CloseIcon from 'virtual:icons/material-symbols/close'
    import { onDestroy, onMount } from 'svelte'
    import { goto } from '$app/navigation'
    import { resolve } from '$app/paths'
    import { page } from '$app/state'
    import {
        videoMetadata,
        appState,
        toggleFullscreen,
        minimizeApp,
        VideoMenuButton,
        invokeFunction,
        handleError,
    } from '$lib'

    let now = $state(new Date())
    let interval: ReturnType<typeof setInterval>

    const localUserTime = $derived(now.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' }))

    onMount(() => {
        interval = setInterval(() => {
            now = new Date()
        }, 60000)
    })

    onDestroy(() => {
        clearInterval(interval)
    })

    const closeVideoPlayer = async (): Promise<void> => {
        try {
            const resp = await invokeFunction('close_video_player', {})
            if (resp.error) throw resp.error

            // TODO Navigation from here
            goto(resolve('/', {}))
        } catch (error) {
            handleError(error, {
                context: 'closing the video failed',
            })
        }
    }
</script>

<div
    class="fixed top-0 left-0 flex h-7 w-full items-center justify-between bg-black px-2"
    data-tauri-drag-region="true">
    <button><VideoMenuButton /></button>
    <p class="text-xs">{$videoMetadata.title} | {$videoMetadata.file}</p>
    <div class="flex items-center gap-4">
        <p class="text-xs {page.url.pathname.includes('details') ? 'text-detailsPageTextColor' : 'text-textColor'}">
            {localUserTime}
        </p>
        <button onclick={minimizeApp} data-action="minimize">
            <MinimizeIcon class="text-white transition-colors hover:text-neutral-400" />
        </button>
        <button onclick={toggleFullscreen}>
            {#if $appState.isFullscreen}
                <ExitFullscreenIcon class="text-white transition-colors hover:text-neutral-400" />
            {:else}
                <FullscreenIcon class="text-white transition-colors hover:text-neutral-400" />
            {/if}
        </button>
        <button onclick={closeVideoPlayer} data-action="close">
            <CloseIcon class="text-white transition-colors hover:text-neutral-400" />
        </button>
    </div>
</div>
