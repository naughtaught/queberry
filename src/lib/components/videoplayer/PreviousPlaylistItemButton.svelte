<script lang="ts">
    import { navigatePlaylist } from '$lib/functions/video/navigatePlaylist'
    import { videoProperties } from '$lib/stores/video'
    import PreviousIcon from 'virtual:icons/material-symbols/skip-previous'

    const hasPreviousPlaylistItem = $derived($videoProperties.playlistPosition > 0)

    const gotoNextPlaylistItem = async (): Promise<void> => {
        await navigatePlaylist('previous')
    }
</script>

<button onclick={gotoNextPlaylistItem} disabled={!hasPreviousPlaylistItem}>
    <PreviousIcon
        class="text-3xl {hasPreviousPlaylistItem
            ? 'text-white transition-colors hover:text-primaryColor'
            : 'cursor-auto text-neutral-700'}" />
</button>
