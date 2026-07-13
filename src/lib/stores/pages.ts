import type { Api } from '$lib/types/api'
import type { App } from '$lib/types/app'
import { get, readable, writable } from 'svelte/store'

export const VALID_ROUTES = ['discover', 'watchlist', 'favourites']

export const PAGINATION_SIZE = readable(50)

export const cursorId = writable(null)

export const activeGridViewSideBarItem = writable<Api.MediaItem | null>()

export const detailsMedia = writable<Api.MediaItem | null>(null)

export const defaultFilters = writable<Api.Filters>({
    minYear: 1903,
    maxYear: new Date().getFullYear() + 1,
    minAvgRating: 0,
    maxAvgRating: 100,
    minLetterboxdRating: 0,
    maxLetterboxdRating: 100,
    minMetacritic: 0,
    maxMetacritic: 100,
    minTomatometer: 0,
    maxTomatometer: 100,
    minPopcornmeter: 0,
    maxPopcornmeter: 100,
    minImdbRating: 0,
    maxImdbRating: 100,
    minCount: 0,
    maxCount: null,
    genres: [],
    excludeGenres: [],
    includeWatched: false,
    includeHidden: false,
    includeWatchlisted: false,
    includeFavourites: false,
    onlyFavourites: false,
    onlyWatchlisted: false,
    onlyWatched: false,
    onlyHidden: false,
    sort: 'total_count',
    ascending: false,
    type: 'movies',
    languages: [],
    excludeLanguages: [],
    countries: [],
    excludeCountries: [],
    contentRatings: [],
    genders: [],
    status: [],
    tags: [],
    excludeTags: [],
})

export const currentFilters = writable<Api.Filters>(get(defaultFilters))

export const defaultShuffleSettings: App.ShuffleSettings = {
    excludeMediaIds: [],
    playedEpisodeIds: [],
    continuousPlay: false,
    randomEpisodes: false,
}

export const shuffleSettings = writable(defaultShuffleSettings)

export const DEFAULT_CACHE = readable<App.Cache>({
    discover: {
        media: [],
        lastUpdated: null,
        paginationPage: 0,
        filters: get(defaultFilters),
    },
    watchlist: {
        media: [],
        lastUpdated: null,
        paginationPage: 0,
        filters: get(defaultFilters),
    },
    favourites: {
        media: [],
        lastUpdated: null,
        paginationPage: 0,
        filters: get(defaultFilters),
    },
    details: {
        media: [],
        lastUpdated: null,
        paginationPage: 0,
        filters: get(defaultFilters),
    },
    trendingMovies: {
        media: [],
        lastUpdated: null,
        paginationPage: 0,
        filters: get(defaultFilters),
    },
    trendingTv: {
        media: [],
        lastUpdated: null,
        paginationPage: 0,
        filters: get(defaultFilters),
    },
    upNext: {
        media: [],
        lastUpdated: null,
        paginationPage: 0,
        filters: get(defaultFilters),
    },
    recentReleases: {
        media: [],
        lastUpdated: null,
        paginationPage: 0,
        filters: get(defaultFilters),
    },
})

export const cache = writable<App.Cache>(get(DEFAULT_CACHE))

export const showHomeHidden = writable(false)
