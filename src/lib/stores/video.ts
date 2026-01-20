import { get, readable, writable } from 'svelte/store'
import type { Api, Video } from '$lib'

export const defaultMetaData = readable<Video.VideoMetadata>({
    duration: 0,
})

export const videoMetadata = writable<Video.VideoMetadata>(get(defaultMetaData))

export const defaultvideoState = readable<Api.VideoState>({
    current_time: 0,
    cache_time: 0,
    cache_speed: 0,
    is_buffering: true,
    buffering_percent: 0,
})

export const videoState = writable<Api.VideoState>(get(defaultvideoState))
