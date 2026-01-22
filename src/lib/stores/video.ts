import { get, readable, writable } from 'svelte/store'
import { type Api } from '$lib'

const defaultMetaData = readable<Api.Metadata>({
    title: '',
    duration: 0,
})

export const videoMetadata = writable<Api.Metadata>(get(defaultMetaData))

const defaultvideoState = readable<Api.VideoState>({
    currentTime: 0,
    cacheTime: 0,
    cacheSpeed: 0,
    isBuffering: true,
    bufferingPercent: 0,
    isPaused: false,
})

export const videoState = writable<Api.VideoState>(get(defaultvideoState))
