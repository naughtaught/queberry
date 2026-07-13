<script lang="ts">
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { videoProperties } from '$lib/stores/video'
    import AudioTracksIcon from 'virtual:icons/fa6-solid/language'
    import SelectModal from '$lib/components/videoplayer/SelectModal.svelte'

    let { currentModal = $bindable() } = $props()

    const setAudioTrack = async (trackId: number): Promise<void> => {
        try {
            const response = await invokeFunction('set_audio_track', {
                value: trackId,
            })
            if (!response.success) throw response.error

            $videoProperties.currentAudioTrack = response.data.value
        } catch (error) {
            handleError(error, {
                context: 'setting the audio track failed',
            })
        }
    }
</script>

{#if currentModal === 'Audio'}
    <SelectModal
        bind:currentModal
        tracks={$videoProperties.audioTracks}
        currentTrack={$videoProperties.currentAudioTrack}
        func={(trackId: number) => {
            setAudioTrack(trackId)
        }}
        bottom="bottom-16"
        left="left-12" />
{/if}

<button
    aria-label="Audio Track"
    onclick={() => {
        if ($videoProperties.audioTracks?.length > 1) {
            if (currentModal === 'Audio') {
                currentModal = null
            } else {
                currentModal = 'Audio'
            }
        }
    }}>
    <AudioTracksIcon
        class={$videoProperties.audioTracks?.length > 1
            ? 'text-white transition-colors  hover:text-primaryColor'
            : 'cursor-default text-neutral-700'} />
</button>
