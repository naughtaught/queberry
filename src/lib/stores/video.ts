import { get, readable, writable } from 'svelte/store'
import { type Api, type Video } from '$lib'

export const defaultMetaData = readable<Video.VideoMetadata>({
    duration: 0,
})

export const videoMetadata = writable<Video.VideoMetadata>(get(defaultMetaData))

export const defaultvideoState = readable<Api.VideoState>({
    currentTime: 0,
    cacheTime: 0,
    cacheSpeed: 0,
    isBuffering: true,
    bufferingPercent: 0,
    isPaused: false,
})

export const videoState = writable<Api.VideoState>(get(defaultvideoState))
