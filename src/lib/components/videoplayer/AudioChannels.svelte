<script lang="ts">
    import { audioChannelOptions, videoMetadata, SelectModal, invokeFunction } from '$lib'
    import AudioChannelsIcon from 'virtual:icons/mdi/surround-sound'

    let { currentModal = $bindable() } = $props()

    const setAudioChannels = async (channel: string): Promise<void> => {
        const response = await invokeFunction('set_audio_channel', {
            value: channel,
        })
        if (response.success) $videoMetadata.audioChannel = response.data.value
    }
</script>

{#if currentModal === 'AudioChannels'}
    <SelectModal
        bind:currentModal
        tracks={$audioChannelOptions}
        currentTrack={$videoMetadata.audioChannel}
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
    <AudioChannelsIcon class="text-white transition-colors hover:text-neutral-400" />
</button>
