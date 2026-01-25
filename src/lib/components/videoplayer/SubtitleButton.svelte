<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { videoMetadata, SelectModal, handleError, type Api } from '$lib'

    let { currentModal = $bindable() } = $props()

    const setSubtitleTrack = async (trackId: number) => {
        try {
            const response: Api.ApiResponse = await invoke('set_subtitle', {
                subtitleId: trackId,
            })
            if (response.success) {
                $videoMetadata.currentSubtitleTrack = response.data!.current_subtitle
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

{#if currentModal === 'Subtitles'}
    <SelectModal
        bind:currentModal
        tracks={$videoMetadata.subtitleTracks}
        currentTrack={$videoMetadata.currentSubtitleTrack}
        func={(trackId: number) => {
            setSubtitleTrack(trackId)
        }}
        bottom="bottom-16"
        left="left-21" />
{/if}

<button
    aria-label="Subtitle Track"
    class={$videoMetadata.subtitleTracks?.length >= 1 ? 'fill-white hover:cursor-pointer' : 'fill-neutral-700'}
    onclick={() => {
        if ($videoMetadata.subtitleTracks?.length >= 1) {
            if (currentModal === 'Subtitles') {
                currentModal = null
            } else {
                currentModal = 'Subtitles'
            }
        }
    }}>
    <svg width="18px" height="18px" viewBox="0 0 16 16">
        <path
            fill-rule="evenodd"
            clip-rule="evenodd"
            d="M0 2H16V14H0V2ZM2 6C2 4.89543 2.89543 4 4 4H7V6H4V10H7V12H4C2.89543 12 2 11.1046 2 10V6ZM11 4C9.89543 4 9 4.89543 9 6V10C9 11.1046 9.89543 12 11 12H14V10H11V6H14V4H11Z" />
    </svg>
</button>
