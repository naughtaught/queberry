<script lang="ts">
    import { invokeFunction, videoProperties } from '$lib'
    import NextIcon from 'virtual:icons/material-symbols/skip-next'

    const hasNextPlaylistItem = $derived($videoProperties.playlistPosition < $videoProperties.playlistCount - 1)

    const gotoNextPlaylistItem = async (): Promise<void> => {
        await invokeFunction('next_playlist_item', {})
    }
</script>

<button onclick={gotoNextPlaylistItem} disabled={!hasNextPlaylistItem}>
    <NextIcon
        class="text-3xl {hasNextPlaylistItem
            ? 'text-white transition-colors hover:text-neutral-400'
            : 'cursor-default text-neutral-700'}" />
</button>
