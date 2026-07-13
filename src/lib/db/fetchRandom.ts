import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { createError, formatError } from '$lib/functions/errors/errorHandling'
import { validateUser } from '$lib/functions/user/validateUser'
import { currentFilters, shuffleSettings } from '$lib/stores/pages'
import { user } from '$lib/stores/user'
import type { App } from '$lib/types/app'
import { get } from 'svelte/store'

export const fetchRandom = async (): Promise<App.Response> => {
    const filters = get(currentFilters)
    const type = filters.type === 'movies' ? 'movie' : 'tv'

    try {
        const currentUser = validateUser()
        if (!currentUser) throw createError('Missing User', 401, { log: false })

        const response = await invokeFunction('api_fetch_random_media', {
            postgresId: currentUser.postgresId,
            token: currentUser.token,
            excludeMediaIds: get(shuffleSettings).excludeMediaIds,
            filters: {
                mediaType: type,
                page: 1,
                pageSize: 1,
                onlyWatched: filters.onlyWatched,
                includeWatched: filters.includeWatched,
                onlyWatchlisted: filters.onlyWatchlisted,
                onlyFavourites: filters.onlyFavourites,
                onlyHidden: filters.onlyHidden,
                includeHidden: filters.includeHidden,
                includeFavourites: filters.includeFavourites,
                includeWatchlisted: filters.includeWatchlisted,
                minYear: filters.minYear,
                maxYear: filters.maxYear,
                minAvgRating: filters.minAvgRating,
                maxAvgRating: filters.maxAvgRating,
                minLetterboxdRating: filters.minLetterboxdRating,
                maxLetterboxdRating: filters.maxLetterboxdRating,
                minMetacritic: filters.minMetacritic,
                maxMetacritic: filters.maxMetacritic,
                minImdbRating: filters.minImdbRating,
                maxImdbRating: filters.maxImdbRating,
                minTomatometer: filters.minTomatometer,
                maxTomatometer: filters.maxTomatometer,
                minPopcornmeter: filters.minPopcornmeter,
                maxPopcornmeter: filters.maxPopcornmeter,
                minCount: filters.minCount,
                maxCount: filters.maxCount,
                genres: filters.genres,
                excludeGenres: filters.excludeGenres,
                languages: filters.languages,
                excludeLanguages: filters.excludeLanguages,
                countries: filters.countries,
                excludeCountries: filters.excludeCountries,
                status: filters.status,
                contentRatings: filters.contentRatings,
                genders: filters.genders,
                postgresId: get(user)?.postgresId,
                sort: filters.sort,
                ascending: filters.ascending,
            },
        })
        if (!response.success) throw response.error
        return { success: true, data: response.data }
    } catch (error) {
        return formatError(error)
    }
}
