<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { onDestroy, onMount } from 'svelte'
    import { goto } from '$app/navigation'
    import { resolve } from '$app/paths'
    import { page } from '$app/state'
    import {
        HeaderButton,
        videoMetadata,
        appState,
        toggleFullscreen,
        minimizeApp,
        handleError,
        VideoMenuButton,
        type Api,
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
            const response: Api.ApiResponse = await invoke('close_video_player', {})
            if (response.success) {
                // TODO Navigation from here
                goto(resolve('/', {}))
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
    }
</script>

<div
    class="fixed top-0 left-0 flex h-7 w-full items-center justify-between bg-black px-2"
    data-tauri-drag-region="true">
    <button><VideoMenuButton /></button>
    <p class="text-xs">{$videoMetadata.title}</p>
    <div class="flex items-center gap-4">
        <p class="text-xs {page.url.pathname.includes('details') ? 'text-detailsPageTextColor' : 'text-textColor'}">
            {localUserTime}
        </p>
        <button onclick={minimizeApp} data-action="minimize">
            <HeaderButton name="minimizeApp" path="M240-120v-80h480v80H240Z" />
        </button>
        <button onclick={toggleFullscreen}>
            {#if $appState.isFullscreen}
                <HeaderButton
                    name="fullscreenOff"
                    path="M240-120v-120H120v-80h200v200h-80Zm400 0v-200h200v80H720v120h-80ZM120-640v-80h120v-120h80v200H120Zm520 0v-200h80v120h120v80H640Z" />
            {:else}
                <HeaderButton
                    name="fullscreenOn"
                    path="M144-144v-192h72v120h120v72H144Zm480 0v-72h120v-120h72v192H624ZM144-624v-192h192v72H216v120h-72Zm600 0v-120H624v-72h192v192h-72Z" />
            {/if}
        </button>
        <button onclick={closeVideoPlayer} data-action="close">
            <HeaderButton
                name="closeVideoPlayer"
                path="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z" />
        </button>
    </div>
</div>
