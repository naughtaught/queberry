import type { Api } from '$lib/types/api'

export const formatAverageRating = (media: Api.MediaItem): number | null => {
    const averageRating = media?.ratings?.find((x) => x.source === 'average')?.rating

    if (!averageRating) return null

    return (averageRating / 100) * 10
}
