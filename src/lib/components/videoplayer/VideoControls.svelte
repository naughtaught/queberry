<script lang="ts">
    import { PlayButton, SeekBar, SeekButton, Volume, AudioChannels, SubtitleButton } from '$lib'

    let currentModal = $state(null)
</script>

<div class="fixed bottom-0 left-0 flex h-28 w-full flex-col bg-black px-4">
    <div class="mb-2 flex w-full justify-center">
        <SeekBar />
    </div>
    <div class="flex w-full items-center justify-between">
        <div class="flex w-1/4 justify-start gap-4">
            <SubtitleButton bind:currentModal />
            <AudioChannels bind:currentModal />
        </div>
        <div class="flex w-2/4 items-center justify-center gap-6">
            <SeekButton direction="back" />
            <PlayButton />
            <SeekButton direction="forward" />
        </div>
        <div class="flex w-1/4 items-center justify-end gap-2">
            <Volume />
        </div>
    </div>
</div>
