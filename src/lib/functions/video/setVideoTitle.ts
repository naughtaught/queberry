import { videoMetadata } from '$lib/stores/video'
import { get } from 'svelte/store'

export const setVideoTitle = (): string => {
    const metaData = get(videoMetadata)

    if (!metaData) return ''

    if (metaData.media?.type === 'tv') {
        const seasonNumber = metaData.seasonNumber ? `S${String(metaData.seasonNumber).padStart(2, '0')}` : ''
        const episodeNumber = metaData.episode?.episode_num
            ? `E${String(metaData.episode.episode_num).padStart(2, '0')}`
            : ''
        return `${metaData.media.title} (${metaData.media?.released}) ${seasonNumber}${episodeNumber} ${metaData.episode?.name ?? ''} | ${metaData.filename}`
    } else {
        return `${metaData.media?.title} (${metaData.media?.released}) | ${metaData.filename}`
    }
}
