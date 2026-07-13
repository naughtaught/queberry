import type { Api } from '$lib/types/api'

export const formatRuntime = (media: Api.MediaItem): string | null => {
    if (!media.runtime) return null

    const minutes = media.runtime % 60
    const hours = Math.floor(media.runtime / 60)

    return `${hours}h ${minutes}m`
}
