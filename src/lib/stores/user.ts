import type { Sql } from '$lib/types/sql'
import type { App } from '$lib/types/app'
import { writable } from 'svelte/store'

export const settings = writable<Sql.UserSettings>({
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
    audioChannel: '5.1',
    defaultResolver: null,
    fileSizeLimit: 0,
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
    indexerSortCriteria: [
        { key: 'Resolution', order: 'desc' },
        { key: 'Quality', order: 'desc' },
        { key: 'Size', order: 'desc' },
        { key: 'Seeders', order: 'desc' },
    ],
    downloadRateLimit: 0,
    screensaverTimeout: 30,
    maxConcurrentDownloads: 3,
    skipIntro: false,
    skipRecap: false,
    skipCredits: false,
    skipPreview: false,
    trailerVolume: 30,
})

export const restrictedContent = writable<App.RestrictedContent>({
    tv: [],
    movies: [],
})

export const areSourceFiltersEnabled = writable<boolean>(true)

export const users = writable<Sql.User[]>([])

export const user = writable<Sql.User | null>(null)

export const hashBlacklist = writable<Sql.Blacklist[]>([])
