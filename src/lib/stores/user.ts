import { get, readable, writable } from 'svelte/store'
import { type Api } from '$lib'

const defaultSettings = readable<Api.UserSettings>({
    id: 0,
    userId: 0,
    preferredTheme: 'default',
    isLightMode: false,
    imageScaling: 100,
    autoplay: true,
    volume: 30,
    completionPercent: 80,
    durationDisplay: 'Duration',
    preferredAudioLanguage: 'Source',
    preferredSubtitleLanguage: 'en',
    subtitleDisplay: 'Auto',
    audioChannels: '2.0',
    defaultResolver: 'torbox',
    fileSizeLimit: 20,
    disabledPlugins: [],
    seasonCompletionRequired: true,
    enableUserRatings: true,
    openAppFullscreen: false,
    updateNotification: true,
    onscreenKeyboardEnabled: false,
    excludedResolutions: [],
    excludedVideoFormats: [],
    excludedAudioOptions: [],
    excludedSources: [],
    indexerSortCriteria: [],
    downloadRateLimit: 0,
    screensaverTimeout: 30,
})

export const settings = writable<Api.UserSettings>(get(defaultSettings))
