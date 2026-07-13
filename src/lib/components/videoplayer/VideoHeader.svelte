<script lang="ts">
    import FullscreenIcon from 'virtual:icons/material-symbols/fullscreen'
    import ExitFullscreenIcon from 'virtual:icons/material-symbols/fullscreen-exit'
    import MinimizeIcon from 'virtual:icons/material-symbols/minimize'
    import CloseIcon from 'virtual:icons/material-symbols/close'
    import { onDestroy, onMount } from 'svelte'
    import { videoMetadata } from '$lib/stores/video'
    import { setVideoTitle } from '$lib/functions/video/setVideoTitle'
    import VideoMenuButton from '$lib/components/videoplayer/VideoMenuButton.svelte'
    import { minimizeApp } from '$lib/functions/ui/minimizeApp'
    import { toggleFullscreen } from '$lib/functions/ui/toggleFullscreen'
    import { isAppFullscreen } from '$lib/stores/app'
    import { closeVideoPlayer } from '$lib/functions/video/closeVideoPlayer'

    let { currentModal = $bindable() } = $props()

    let now = $state(new Date())
    let interval: ReturnType<typeof setInterval>

    const localUserTime = $derived(now.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' }))

    let title = $state('')

    $effect(() => {
        if ($videoMetadata) title = setVideoTitle()
    })

    onMount(() => {
        interval = setInterval(() => {
            now = new Date()
        }, 60000)
    })

    onDestroy(() => {
        clearInterval(interval)
    })
</script>

<div class="fixed top-0 left-0 flex h-7 w-full items-center justify-between bg-black px-2">
    <button><VideoMenuButton bind:currentModal /></button>
    <p class="flex-1 pl-2 text-left text-xs" data-tauri-drag-region="true">{title}</p>
    <div class="flex items-center gap-4">
        <p class="text-xs text-white">
            {localUserTime}
        </p>
        <button onclick={minimizeApp}>
            <MinimizeIcon class="text-white transition-colors hover:text-primaryColor" />
        </button>
        <button onclick={toggleFullscreen}>
            {#if $isAppFullscreen}
                <ExitFullscreenIcon class="text-white transition-colors hover:text-primaryColor" />
            {:else}
                <FullscreenIcon class="text-white transition-colors hover:text-primaryColor" />
            {/if}
        </button>
        <button onclick={closeVideoPlayer}>
            <CloseIcon class="text-white transition-colors hover:text-primaryColor" />
        </button>
    </div>
</div>
