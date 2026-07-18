import { writable } from 'svelte/store'
import type { App } from '$lib/types/app'

export const TV_RATINGS = ['TV-Y', 'TV-Y7', 'TV-G', 'TV-PG', 'TV-14', 'TV-MA']

export const MOVIE_RATINGS = ['G', 'PG', 'PG-13', 'R', 'NC-17']

export const SKIP_ERRORS = ['infringing', 'no suitable', 'no matching', 'episode not found']

export const MIN_COMPLETION_TIME = 300

export const PRIORITY_LANGAUGES = [
    'English',
    'French',
    'German',
    'Hindi',
    'Chinese',
    'Japanese',
    'Italian',
    'Russian',
    'Arabic',
    'Korean',
]
export const PRIORITY_COUNTRIES = [
    'United States',
    'United Kingdom',
    'France',
    'Japan',
    'Germany',
    'South Korea',
    'Canada',
    'Australia',
    'New Zealand',
]

export const GENRE_LIST = [
    'Action',
    'Adventure',
    'Animation',
    'Anime',
    'Comedy',
    'Crime',
    'Documentary',
    'Drama',
    'Family',
    'Fantasy',
    'History',
    'Horror',
    'Kids',
    'Musical',
    'Mystery',
    'Reality',
    'Romance',
    'Sci-Fi',
    'Soap',
    'Sport',
    'Thriller',
    'War',
    'Western',
]

export const GENDER_LIST = ['Female', 'Male', 'Non-binary']

export const parentalControlsAreEnabled = writable(false)

export const primaryUser = writable<number | null>(null)

export const appData = writable<App.AppData>({
    currentVersion: '0.0.1',
    currentNotes: null,
    updateAvailable: false,
    pendingUpdate: null,
    showUpdateModal: false,
    updateResolver: null,
    isDownloading: false,
    downloadProgress: 0,
    downloadContentLength: undefined,
    isInstalling: false,
})

export const directories = writable<{ movies: string | null; tv: string | null }>({
    movies: null,
    tv: null,
})

export const isAppFullscreen = writable(false)

export const scrollPositions = writable<App.ScrollPositions>({
    home: {
        x: 0,
        y: 0,
    },
    discover: {
        x: 0,
        y: 0,
    },
    watchlist: {
        x: 0,
        y: 0,
    },
    favourites: {
        x: 0,
        y: 0,
    },
    episodeContainer: {
        x: 0,
        y: 0,
    },
})

export const previousPage = writable<string>('/')

export const STATUS_TYPES = ['Ended', 'Released', 'Returning Series']

export const modals = writable({
    user: false,
    trailer: false,
    filters: false,
    homeMenu: false,
    transfer: false,
    search: false,
    shuffle: false,
    download: false,
})

export const loadingStates = writable({
    isGridViewLoading: true,
    isGridViewLoadingMore: false,
    isSourcesLoading: false,
    isEpisodesLoading: false,
    isVideoLoading: false,
    isUpNextLoading: false,
    isRecentReleasesLoading: false,
    isTrendingLoading: false,
    isPlayButtonLoading: false,
    isLocalMediaLoading: false,
})

export const RESOLUTION_ORDER = ['2160p', '1080p', '720p', '480p', 'Unknown']

export const QUALITY_ORDER = ['remux', 'bluray', 'web-dl', 'dvd', 'telesync', 'cam']

export const AUDIO_OPTIONS = ['DTS', 'DTSHD', 'TrueHD', 'Atmos']

export const VIDEO_OPTIONS = [
    'Remux',
    'BluRay',
    'Web-DL',
    'DVD',
    'Screener',
    'Telesync',
    'Cam',
    'Unknown',
    '3D',
    'HDR',
    'Dolby Vision',
]

export const MIN_PASSWORD_LENGTH = 6
export const MAX_PASSWORD_LENGTH = 30

export const MIN_PIN_LENGTH = 4
export const MAX_PIN_LENGTH = 8

export const passwordVisibility = writable<boolean>(false)

export const SCREENSAVER_TIMEOUTS = [
    { name: 'Never', value: 0 },
    { name: '10 minutes', value: 10 },
    { name: '15 minutes', value: 15 },
    { name: '30 minutes', value: 30 },
    { name: '60 minutes', value: 60 },
]

export const TOAST_DURATION = 6000

const createToastNotification = (): App.ToastNotificationStore => {
    let timeoutId: NodeJS.Timeout | null = null
    const QUEUE: App.QueuedToast[] = []
    let isShowing = false

    const { subscribe, set } = writable<App.ToastNotification>({
        title: null,
        message: null,
        type: null,
    })

    const showNext = (): void => {
        if (QUEUE.length === 0) {
            isShowing = false
            return
        }

        isShowing = true
        const next = QUEUE.shift()!

        set({
            title: next.notification.title ?? null,
            message: next.notification.message ?? null,
            type: next.notification.type ?? null,
        })

        if (timeoutId) clearTimeout(timeoutId)

        timeoutId = setTimeout(() => {
            set({ title: null, message: null, type: null })
            timeoutId = null
            showNext()
        }, TOAST_DURATION)
    }

    return {
        subscribe,
        show: (notification) => {
            QUEUE.push({
                notification: {
                    title: notification.title ?? null,
                    message: notification.message ?? null,
                    type: notification.type as App.ToastNotification['type'],
                },
            })
            if (!isShowing) showNext()
        },
        hide: () => {
            if (timeoutId) {
                clearTimeout(timeoutId)
                timeoutId = null
            }

            set({ title: null, message: null, type: null })

            showNext()
        },
    }
}

export const toastNotification = createToastNotification()
