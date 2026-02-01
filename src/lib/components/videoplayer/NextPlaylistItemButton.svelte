<script lang="ts">
    import { invokeFunction, videoMetadata } from '$lib'
    import NextIcon from 'virtual:icons/material-symbols/skip-next'

    const hasNextPlaylistItem = $derived($videoMetadata.playlistPosition < $videoMetadata.playlistCount - 1)

    const gotoNextPlaylistItem = async (): Promise<void> => {
        const response = await invokeFunction('next_playlist_item', {})

        if (response.success) console.log(response)
    }
</script>

<button onclick={gotoNextPlaylistItem} disabled={!hasNextPlaylistItem}>
    <NextIcon
        class="text-3xl {hasNextPlaylistItem
            ? 'text-white transition-colors hover:text-neutral-400'
            : 'cursor-default text-neutral-700'}" />
</button>
