<script lang="ts">
    import { videoMetadata, SelectModal, invokeFunction } from '$lib'
    import AudioTracksIcon from 'virtual:icons/fa6-solid/language'

    let { currentModal = $bindable() } = $props()

    const setAudioTrack = async (trackId: number): Promise<void> => {
        const response = await invokeFunction('set_audio_track', {
            value: trackId,
        })
        if (response.success) $videoMetadata.currentAudioTrack = response.data.value
    }
</script>

{#if currentModal === 'Audio'}
    <SelectModal
        bind:currentModal
        tracks={$videoMetadata.audioTracks}
        currentTrack={$videoMetadata.currentAudioTrack}
        func={(trackId: number) => {
            setAudioTrack(trackId)
        }}
        bottom="bottom-16"
        left="left-12" />
{/if}

<button
    aria-label="Audio Track"
    onclick={() => {
        if ($videoMetadata.audioTracks?.length > 1) {
            if (currentModal === 'Audio') {
                currentModal = null
            } else {
                currentModal = 'Audio'
            }
        }
    }}>
    <AudioTracksIcon
        class={$videoMetadata.audioTracks?.length >= 1
            ? 'text-white transition-colors  hover:text-neutral-400'
            : 'text-neutral-700'} />
</button>
