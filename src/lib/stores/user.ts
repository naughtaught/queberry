import { writable } from 'svelte/store'
import { type User } from '$lib'

export const settings = writable<User.Settings>()

export const defaultSettings = writable<User.Settings>({
    id: 0,
    userId: 0,
    preferredTheme: 'Default',
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
