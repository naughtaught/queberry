<script lang="ts">
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { AUDIO_CHANNELS, videoProperties } from '$lib/stores/video'
    import AudioChannelsIcon from 'virtual:icons/mdi/surround-sound'
    import SelectModal from '$lib/components/videoplayer/SelectModal.svelte'

    let { currentModal = $bindable() } = $props()

    const setAudioChannels = async (channel: string): Promise<void> => {
        try {
            const response = await invokeFunction('set_audio_channel', {
                value: channel,
            })

            if (!response.success) throw response.error

            $videoProperties.audioChannel = response.data.value
        } catch (error) {
            handleError(error, {
                context: 'setting the audio channel failed',
            })
        }
    }
</script>

{#if currentModal === 'AudioChannels'}
    <SelectModal
        bind:currentModal
        tracks={$AUDIO_CHANNELS}
        currentTrack={$videoProperties.audioChannel}
        bottom="bottom-16"
        left="left-32"
        func={(track: string) => {
            setAudioChannels(track)
        }} />
{/if}

<button
    aria-label="Set Audio Channels"
    onclick={() => {
        if (currentModal === 'AudioChannels') {
            currentModal = null
        } else {
            currentModal = 'AudioChannels'
        }
    }}>
    <AudioChannelsIcon class="text-white transition-colors hover:text-primaryColor" />
</button>
