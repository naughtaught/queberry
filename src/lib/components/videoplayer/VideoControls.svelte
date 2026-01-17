<script lang="ts">
    import { PlayButton, SeekButton } from '$lib'

    let { isPaused = $bindable(), currentTime = $bindable() } = $props()
</script>

<div class="fixed bottom-0 left-0 flex h-28 w-full flex-col bg-black px-4">
    <div class="flex w-full items-center justify-between">
        <div class="flex w-2/4 items-center justify-center gap-6">
            <SeekButton direction="back" {currentTime} />
            <PlayButton {isPaused} />
            <SeekButton direction="forward" {currentTime} />
        </div>
    </div>
</div>
