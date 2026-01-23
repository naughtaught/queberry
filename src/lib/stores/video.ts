import { get, readable, writable } from 'svelte/store'
import { type Api } from '$lib'

export const defaultVideoMetadata = readable<Api.Metadata>({
    title: '',
    duration: 0,
})

export const videoMetadata = writable<Api.Metadata>(get(defaultVideoMetadata))

export const defaultVideoState = readable<Api.VideoState>({
    currentTime: 0,
    cacheTime: 0,
    cacheSpeed: 0,
    isBuffering: true,
    bufferingPercent: 0,
    isPaused: false,
})

export const videoState = writable<Api.VideoState>(get(defaultVideoState))
