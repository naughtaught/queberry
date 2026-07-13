import { videoMetadata } from '$lib/stores/video'
import type { Api } from '$lib/types/api'
import { get } from 'svelte/store'

export const updateLocalTimings = (timingData: {
    intro: Api.TimingRange[]
    recap: Api.TimingRange[]
    credits: Api.TimingRange[]
    preview: Api.TimingRange[]
}): void => {
    const timings_last_updated = new Date().toISOString()
    const metadata = get(videoMetadata)

    if (metadata.media?.type === 'tv') {
        videoMetadata.update((data) => ({
            ...data,
            episode: {
                ...metadata.episode!,
                intro_timings: timingData.intro,
                recap_timings: timingData.recap,
                credits_timings: timingData.credits,
                preview_timings: timingData.preview,
                timings_last_updated,
            },
            playlist: metadata.playlist.map((item) =>
                item.playlistIndex === metadata.playlistIndex
                    ? {
                          ...item,
                          episode: {
                              ...item.episode!,
                              intro_timings: timingData.intro,
                              recap_timings: timingData.recap,
                              credits_timings: timingData.credits,
                              preview_timings: timingData.preview,
                              timings_last_updated,
                          },
                      }
                    : item,
            ),
        }))
    } else if (metadata.media?.type === 'movie') {
        videoMetadata.update((data) => ({
            ...data,
            media: {
                ...metadata.media!,
                intro_timings: timingData.intro,
                recap_timings: timingData.recap,
                credits_timings: timingData.credits,
                preview_timings: timingData.preview,
                timings_last_updated,
            },
            playlist: metadata.playlist.map((item) =>
                item.playlistIndex === metadata.playlistIndex
                    ? {
                          ...item,
                          media: {
                              ...item.media!,
                              intro_timings: timingData.intro,
                              recap_timings: timingData.recap,
                              credits_timings: timingData.credits,
                              preview_timings: timingData.preview,
                              timings_last_updated,
                          },
                      }
                    : item,
            ),
        }))
    }

    // TODO update db? For tv only? Movies would need db restructure, wanted for shuffle/collections though?
}
