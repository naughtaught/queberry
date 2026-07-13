<script lang="ts">
    import { navigatePlaylist } from '$lib/functions/video/navigatePlaylist'
    import { videoProperties } from '$lib/stores/video'
    import NextIcon from 'virtual:icons/material-symbols/skip-next'

    const hasNextPlaylistItem = $derived($videoProperties.playlistPosition < $videoProperties.playlistCount - 1)

    const gotoNextPlaylistItem = async (): Promise<void> => {
        await navigatePlaylist('next')
    }
</script>

<button onclick={gotoNextPlaylistItem} disabled={!hasNextPlaylistItem}>
    <NextIcon
        class="text-3xl {hasNextPlaylistItem
            ? 'text-white transition-colors hover:text-primaryColor'
            : 'cursor-default! text-neutral-700'}" />
</button>
