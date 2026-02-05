<script lang="ts">
    import { videoProperties, SelectModal, invokeFunction } from '$lib'
    import SubtitlesIcon from 'virtual:icons/mdi/subtitles'

    let { currentModal = $bindable() } = $props()

    const setSubtitleTrack = async (trackId: number): Promise<void> => {
        const response = await invokeFunction('set_subtitle_track', {
            value: trackId,
        })
        if (response.success) $videoProperties.currentSubtitleTrack = response.data.value
    }
</script>

{#if currentModal === 'Subtitles'}
    <SelectModal
        bind:currentModal
        tracks={$videoProperties.subtitleTracks}
        currentTrack={$videoProperties.currentSubtitleTrack}
        func={(trackId: number) => {
            setSubtitleTrack(trackId)
        }}
        bottom="bottom-16"
        left="left-23" />
{/if}

<button
    aria-label="Subtitle Track"
    onclick={() => {
        if ($videoProperties.subtitleTracks?.length >= 1) {
            if (currentModal === 'Subtitles') {
                currentModal = null
            } else {
                currentModal = 'Subtitles'
            }
        }
    }}>
    <SubtitlesIcon
        class={$videoProperties.subtitleTracks?.length >= 1
            ? 'text-white transition-colors  hover:text-neutral-400'
            : 'text-neutral-700'} />
</button>
