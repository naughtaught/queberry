<script lang="ts">
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { videoProperties } from '$lib/stores/video'
    import SubtitlesIcon from 'virtual:icons/mdi/subtitles'
    import SelectModal from '$lib/components/videoplayer/SelectModal.svelte'

    let { currentModal = $bindable() } = $props()

    const setSubtitleTrack = async (trackId: number): Promise<void> => {
        try {
            const resp = await invokeFunction('set_subtitle_track', {
                value: trackId,
            })
            if (!resp.success) throw resp.error

            $videoProperties.currentSubtitleTrack = resp.data.value
        } catch (error) {
            handleError(error, {
                context: 'setting the subtitle track failed',
            })
        }
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
            ? 'text-white transition-colors  hover:text-primaryColor'
            : 'cursor-default  text-neutral-700'} />
</button>
