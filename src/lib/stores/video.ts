import { get, readable, writable } from 'svelte/store'
import { type Api } from '$lib'

export const defaultVideoMetadata = readable<Api.Metadata>({
    title: '',
    duration: 0,
    audioChannel: '5.1',
    subtitleTracks: [],
    currentSubtitleTrack: null,
    audioTracks: [],
    currentAudioTrack: null,
    avSync: 0,
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

export const audioChannelOptions = readable([
    { value: '2.0', name: 'Stereo (2.0)' },
    { value: '2.1', name: '2.1' },
    { value: '3.0', name: '3.0 (L+R+C)' },
    { value: '3.1', name: '3.1 (L+R+C+Sub)' },
    { value: '4.0', name: 'Quadraphonic (4.0)' },
    { value: '4.1', name: '4.1' },
    { value: '5.0', name: '5.0 Surround' },
    { value: '5.1', name: '5.1 Surround' },
    { value: '6.0', name: '6.0 Surround' },
    { value: '6.1', name: '6.1 Surround' },
    { value: '7.1', name: '7.1 Surround' },
    { value: 'auto', name: 'Auto (Original)' },
])

export const speakerLayoutsWithCenter = ['3.0', '3.1', '5.0', '5.1', '6.0', '6.1', '7.1']

export const defaultSessionSettings = readable({
    centerSpeakerLevel: 0,
})

export const sessionSettings = writable(get(defaultSessionSettings))
