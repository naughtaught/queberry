<script lang="ts">
    import { loadVideo } from '$lib/functions/video/loadVideo'
    import { loadingStates } from '$lib/stores/app'
    import PlayIcon from 'virtual:icons/material-symbols/play-circle'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'

    let { media = $bindable(), size = 'text-2xl', background = '' } = $props()
</script>

<button
    disabled={$loadingStates.isPlayButtonLoading}
    onclick={(event) => {
        event.preventDefault()
        loadVideo(media, null, null, null)
    }}
    class="flex items-center rounded"
    aria-label="Play">
    {#if $loadingStates.isPlayButtonLoading}
        <LoadingSpinner />
    {:else}
        <PlayIcon class="{size} rounded {background} text-textColor hover:text-primaryColor" />
    {/if}
</button>
