<script lang="ts">
    import AudioChannels from '$lib/components/videoplayer/AudioChannels.svelte'
    import AudioTrackButton from '$lib/components/videoplayer/AudioTrackButton.svelte'
    import NextPlaylistItemButton from '$lib/components/videoplayer/NextPlaylistItemButton.svelte'
    import PlayButton from '$lib/components/videoplayer/PlayButton.svelte'
    import PreviousPlaylistItemButton from '$lib/components/videoplayer/PreviousPlaylistItemButton.svelte'
    import SeekBar from '$lib/components/videoplayer/SeekBar.svelte'
    import SeekButton from '$lib/components/videoplayer/SeekButton.svelte'
    import SubtitleTrackButton from '$lib/components/videoplayer/SubtitleTrackButton.svelte'
    import VideoSettingsButton from '$lib/components/videoplayer/VideoSettingsButton.svelte'
    import Volume from '$lib/components/videoplayer/Volume.svelte'

    let { currentModal = $bindable(), previousVolume = $bindable() } = $props()
</script>

<div class="fixed bottom-0 left-0 flex h-28 w-full flex-col bg-black px-4">
    <div class="flex w-full justify-center">
        <SeekBar />
    </div>
    <div class="flex w-full items-center justify-between">
        <div class="flex w-1/4 justify-start gap-4">
            <VideoSettingsButton bind:currentModal />
            <AudioTrackButton bind:currentModal />
            <SubtitleTrackButton bind:currentModal />
            <AudioChannels bind:currentModal />
        </div>
        <div class="flex w-2/4 items-center justify-center gap-4">
            <div>
                <PreviousPlaylistItemButton />
                <SeekButton direction="back" />
            </div>
            <PlayButton />
            <div>
                <SeekButton direction="forward" />
                <NextPlaylistItemButton />
            </div>
        </div>
        <div class="flex w-1/4 items-center justify-end gap-2">
            <Volume bind:previousVolume />
        </div>
    </div>
</div>
