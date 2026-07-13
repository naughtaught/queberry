import { videoMetadata } from '$lib/stores/video'
import type { Video } from '$lib/types/video'

export const updateVideoMetadata = (metadata: Video.Metadata, playlistItem: Video.PlaylistItem): void => {
    const newMetadata = {
        ...metadata,
        playlist: [...metadata.playlist, playlistItem],
    }

    videoMetadata.set(newMetadata)
}
