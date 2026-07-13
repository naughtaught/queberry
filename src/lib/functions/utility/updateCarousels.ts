import { fetchRecentReleases } from '$lib/db/fetchRecentReleases'
import { fetchTrending } from '$lib/db/fetchTrending'
import { fetchUpNext } from '$lib/db/fetchUpNext'
import { loadingStates } from '$lib/stores/app'
import { cache } from '$lib/stores/pages'
import type { App } from '$lib/types/app'
import { formatError } from '$lib/functions/errors/errorHandling'

export const updateCarousels = async (): Promise<App.Response> => {
    try {
        loadingStates.update((states) => ({
            ...states,
            isRecentReleasesLoading: true,
            isTrendingLoading: true,
        }))

        const [trendingResp, recentReleasesResp] = await Promise.all([
            fetchTrending(),
            fetchRecentReleases(),
            fetchUpNext(),
        ])

        if (!trendingResp.success) throw trendingResp.error
        if (!recentReleasesResp.success) throw recentReleasesResp.error

        cache.update((currentCache) => ({
            ...currentCache,
            trendingMovies: {
                ...currentCache.trendingMovies,
                media: trendingResp.data.movies.sort(
                    (a: { released: number; id: number }, b: { released: number; id: number }) => {
                        const dateA = new Date(a.released)
                        const dateB = new Date(b.released)

                        if (dateA > dateB) return -1
                        if (dateA < dateB) return 1

                        return b.id - a.id
                    },
                ),
            },
            trendingTv: {
                ...currentCache.trendingTv,
                media: trendingResp.data.tv.sort(
                    (a: { released: number; id: number }, b: { released: number; id: number }) => {
                        const dateA = new Date(a.released)
                        const dateB = new Date(b.released)

                        if (dateA > dateB) return -1
                        if (dateA < dateB) return 1

                        return b.id - a.id
                    },
                ),
            },
            recentReleases: {
                ...currentCache.recentReleases,
                media: recentReleasesResp.data,
            },
        }))

        return {
            success: true,
            data: null,
        }
    } catch (error) {
        return formatError(error)
    } finally {
        loadingStates.update((states) => ({
            ...states,
            isRecentReleasesLoading: false,
            isTrendingLoading: false,
        }))
    }
}
