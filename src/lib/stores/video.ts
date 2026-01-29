import { get, readable, writable } from 'svelte/store'
import { type Api } from '$lib'

export const seekAmount = readable(30)

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
    { value: '2.0', name: '2.0' },
    { value: '2.1', name: '2.1' },
    { value: '3.0', name: '3.0' },
    { value: '3.1', name: '3.1' },
    { value: '4.0', name: '4.0' },
    { value: '4.1', name: '4.1' },
    { value: '5.0', name: '5.0' },
    { value: '5.1', name: '5.1' },
    { value: '6.0', name: '6.0' },
    { value: '6.1', name: '6.1' },
    { value: '7.1', name: '7.1' },
    { value: 'auto', name: 'Auto' },
])

export const speakerLayoutsWithCenter = ['3.0', '3.1', '5.0', '5.1', '6.0', '6.1', '7.1']

export const defaultSessionSettings = writable({
    centerSpeakerLevel: 0,
    volume: 30,
})

export const sessionSettings = writable(get(defaultSessionSettings))

export const defaultKeyboardShortcuts = readable([
    { id: 'pause', name: 'Play/Pause', key: 'Space' },
    { id: 'fullscreen', name: 'Toggle Fullscreen', key: 'KeyF' },
    { id: 'mute', name: 'Mute/Unmute', key: 'KeyM' },
    { id: 'forward', name: 'Forward 30s', key: 'ArrowRight' },
    { id: 'rewind', name: 'Rewind 30s', key: 'ArrowLeft' },
    // TODO ArrowUp next track
    // TODO ArrowDown previous track
])

export const keyboardShortcuts = writable(get(defaultKeyboardShortcuts))
