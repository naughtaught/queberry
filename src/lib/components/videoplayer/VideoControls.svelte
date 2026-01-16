<script lang="ts">
    import PlayButton from '$lib/components/videoplayer/PlayButton.svelte'

    let { isPaused = $bindable() } = $props()
</script>

<div class="fixed bottom-0 left-0 flex h-28 w-full flex-col bg-black px-4">
    <div class="flex w-full items-center justify-between">
        <div class="flex w-2/4 items-center justify-center gap-6">
            <PlayButton {isPaused} />
        </div>
    </div>
</div>
