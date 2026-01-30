<script lang="ts">
    import { invokeFunction, videoState } from '$lib'
    import PauseIcon from 'virtual:icons/material-symbols/pause'
    import PlayIcon from 'virtual:icons/material-symbols/play-arrow'

    const togglePlay = async (): Promise<void> => {
        const response = await invokeFunction('toggle_play', {
            value: $videoState.isPaused,
        })
        if (response.success) $videoState.isPaused = response.data.value
    }
</script>

<button onclick={togglePlay}>
    {#if $videoState.isPaused}
        <PlayIcon class="text-5xl text-white transition-colors hover:text-neutral-400" />
    {:else}
        <PauseIcon class="text-5xl text-white transition-colors hover:text-neutral-400" />
    {/if}
</button>
