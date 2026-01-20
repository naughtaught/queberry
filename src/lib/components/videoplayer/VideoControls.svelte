<script lang="ts">
    import { PlayButton, SeekBar, SeekButton } from '$lib'

    let { isPaused = $bindable() } = $props()
</script>

<div class="fixed bottom-0 left-0 flex h-28 w-full flex-col bg-black px-4">
    <div class="mb-2 flex w-full justify-center">
        <SeekBar />
    </div>
    <div class="flex w-full items-center justify-between">
        <div class="flex w-2/4 items-center justify-center gap-6">
            <SeekButton direction="back" />
            <PlayButton {isPaused} />
            <SeekButton direction="forward" />
        </div>
    </div>
</div>
