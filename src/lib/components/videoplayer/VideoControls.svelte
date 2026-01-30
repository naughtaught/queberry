<script lang="ts">
    import {
        PlayButton,
        SeekBar,
        SeekButton,
        Volume,
        AudioChannels,
        SubtitleTrackButton,
        AudioTrackButton,
        VideoSettingsButton,
    } from '$lib'

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
        <div class="flex w-2/4 items-center justify-center gap-6">
            <SeekButton direction="back" />
            <PlayButton />
            <SeekButton direction="forward" />
        </div>
        <div class="flex w-1/4 items-center justify-end gap-2">
            <Volume bind:previousVolume />
        </div>
    </div>
</div>
