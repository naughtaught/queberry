<script lang="ts">
    import { invokeFunction, videoMetadata } from '$lib'
    import PreviousIcon from 'virtual:icons/material-symbols/skip-previous'

    const hasPreviousPlaylistItem = $derived($videoMetadata.playlistPosition > 0)

    const gotoNextPlaylistItem = async (): Promise<void> => {
        await invokeFunction('previous_playlist_item', {})
    }
</script>

<button onclick={gotoNextPlaylistItem} disabled={!hasPreviousPlaylistItem}>
    <PreviousIcon
        class="text-3xl {hasPreviousPlaylistItem
            ? 'text-white transition-colors hover:text-neutral-400'
            : 'cursor-auto text-neutral-700'}" />
</button>
