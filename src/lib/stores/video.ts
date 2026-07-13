import type { Video } from '$lib/types/video'
import { get, readable, writable } from 'svelte/store'

export const SEEK_AMOUNT = readable(30)

export const DEFAULT_VIDEO_PROPERTIES = readable<Video.VideoProperties>({
    duration: 0,
    audioChannel: '5.1',
    subtitleTracks: [],
    currentSubtitleTrack: null,
    audioTracks: [],
    currentAudioTrack: null,
    avSync: 0,
    playlistPosition: 0,
    playlistCount: 1,
    availableShaders: [],
    activeShaders: [],
    subtitleSync: 0,
})

export const videoProperties = writable<Video.VideoProperties>(get(DEFAULT_VIDEO_PROPERTIES))

export const DEFAULT_VIDEO_STATE = readable<Video.VideoState>({
    currentTime: 0,
    cacheTime: 0,
    cacheSpeed: 0,
    isBuffering: true,
    bufferingPercent: 0,
    isPaused: false,
})

export const videoState = writable<Video.VideoState>(get(DEFAULT_VIDEO_STATE))

export const AUDIO_CHANNELS = readable([
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

export const SPEAKER_LAYOUTS_WITH_CENTER_SPEAKER = ['3.0', '3.1', '5.0', '5.1', '6.0', '6.1', '7.1']

export const defaultSessionSettings = writable<Video.SessionSettings>({
    centerSpeakerLevel: 0,
    volume: 30,
    subtitleScaling: 1,
    subtitlePos: null,
    disableIntroTiming: false,
    disablePreviewTiming: false,
    disableRecapTiming: false,
    disableCreditTiming: false,
    disableAllTimings: false,
    isIntroTimingUpdated: false,
    isRecapTimingUpdated: false,
    isPreviewTimingUpdated: false,
    isCreditTimingUpdated: false,
})

export const sessionSettings = writable<Video.SessionSettings>(get(defaultSessionSettings))

export const keyboardShortcuts = writable([
    {
        id: 'togglePlay',
        name: 'Play/Pause',
        code: 'Space',
        shiftKey: false,
    },
    {
        id: 'fullscreen',
        name: 'Toggle Fullscreen',
        code: 'KeyF',
        shiftKey: false,
    },
    {
        id: 'mute',
        name: 'Mute/Unmute',
        code: 'KeyM',
        shiftKey: false,
    },
    {
        id: 'forward',
        name: 'Forward 30s',
        code: 'ArrowRight',
        shiftKey: false,
    },
    {
        id: 'rewind',
        name: 'Rewind 30s',
        code: 'ArrowLeft',
        shiftKey: false,
    },
    {
        id: 'playlistNext',
        name: 'Next Playlist Item',
        code: 'ArrowUp',
        shiftKey: false,
    },
    {
        id: 'playlistPrevious',
        name: 'Previous Playlist Item',
        code: 'ArrowDown',
        shiftKey: false,
    },
    {
        id: 'close',
        name: 'Exit Video',
        code: 'Escape',
        shiftKey: false,
    },
    {
        id: 'volumeUp',
        name: 'Volume Up',
        code: 'Equal',
        shiftKey: false,
    },
    {
        id: 'volumeDown',
        name: 'Volume Down',
        code: 'Minus',
        shiftKey: false,
    },
])

export const DEFAULT_VIDEO_METADATA = readable<Video.Metadata>({
    videoUrl: null,
    filename: null,
    files: [],
    backdrop: null,
    language: null,
    infohash: null,
    resolver: null,
    seasonNumber: null,
    episode: null,
    playlistIndex: 0,
    playlist: [],
    media: null,
    collectionIndex: 0,
    collectionItems: [],
})

export const videoMetadata = writable<Video.Metadata>(get(DEFAULT_VIDEO_METADATA))

export const shouldCancelVideoLoad = writable(false)
