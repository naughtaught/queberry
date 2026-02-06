<script lang="ts">
    import { handleError, invokeFunction, videoState } from '$lib'
    import PauseIcon from 'virtual:icons/material-symbols/pause'
    import PlayIcon from 'virtual:icons/material-symbols/play-arrow'

    const togglePlay = async (): Promise<void> => {
        try {
            const response = await invokeFunction('toggle_play', {
                value: $videoState.isPaused,
            })
            if (response.error) throw response.error
            $videoState.isPaused = response.data.value
        } catch (error) {
            handleError(error, {
                context: 'toggling play state failed',
            })
        }
    }
</script>

<button onclick={togglePlay}>
    {#if $videoState.isPaused}
        <PlayIcon class="text-5xl text-white transition-colors hover:text-neutral-400" />
    {:else}
        <PauseIcon class="text-5xl text-white transition-colors hover:text-neutral-400" />
    {/if}
</button>
