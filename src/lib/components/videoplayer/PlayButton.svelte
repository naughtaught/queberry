<script lang="ts">
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { videoState } from '$lib/stores/video'
    import PauseIcon from 'virtual:icons/material-symbols/pause'
    import PlayIcon from 'virtual:icons/material-symbols/play-arrow'

    const togglePlay = async (): Promise<void> => {
        try {
            const response = await invokeFunction('toggle_play', {
                value: $videoState.isPaused,
            })
            if (!response.success) throw response.error
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
        <PlayIcon class="text-5xl text-white transition-colors hover:text-primaryColor" />
    {:else}
        <PauseIcon class="text-5xl text-white transition-colors hover:text-primaryColor" />
    {/if}
</button>
