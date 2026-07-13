import { goto } from '$app/navigation'
import { loadingStates, parentalControlsAreEnabled } from '$lib/stores/app'
import { restrictedContent, user } from '$lib/stores/user'
import type { Api } from '$lib/types/api'
import type { Plugins } from '$lib/types/plugins'
import { get } from 'svelte/store'
import { getImagePath } from '$lib/functions/ui/getImagePath'
import type { Video } from '$lib/types/video'
import { DEFAULT_VIDEO_METADATA, shouldCancelVideoLoad, videoMetadata } from '$lib/stores/video'
import { fetchSeasonData } from '$lib/db/fetchSeasons'
import { selectSeason } from '$lib/functions/utility/selectSeason'
import { getFirstUnwatchedEpisode } from '$lib/functions/utility/getFirstUnwatchedEpisode'
import { fetchSources } from '$lib/functions/plugins/fetchSources'
import { getLanguageCodeByName } from '$lib/functions/utility/parseLanguage'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { createError, handleError } from '$lib/functions/errors/errorHandling'
import { checkIfCompletelyWatched } from '$lib/functions/utility/checkIfCompletelyWatched'
import { shuffleSettings } from '$lib/stores/pages'
import { getRandomEpisode } from '$lib/functions/video/getRandomEpisode'
import { fetchVideoFromSources } from '$lib/functions/video/fetchVideoFromSources'
import { fetchLocalMedia } from '$lib/functions/video/fetchLocalMedia'

const TIMEOUTS = {
    SEASON_DATA: 10000,
    SOURCES_FETCH: 35000,
    VIDEO_URL_FETCH: 35000,
    INVOKE_FUNCTION: 20000,
    TOTAL_OPERATION: 200000,
}

const checkCancellation = (): void => {
    if (get(shouldCancelVideoLoad)) throw new DOMException('Operation cancelled by user', 'AbortError')
}

const withTimeout = async <T>(promise: Promise<T>, timeoutMs: number, cancelFlag: () => boolean): Promise<T> => {
    let timeoutId: NodeJS.Timeout | undefined = undefined

    const timeoutPromise = new Promise<never>((_, reject) => {
        timeoutId = setTimeout(() => {
            reject(new Error(`Operation timed out after ${timeoutMs}ms`))
        }, timeoutMs)
    })

    const cancelCheckPromise = new Promise<never>((_, reject) => {
        const checkInterval = setInterval(() => {
            if (cancelFlag()) {
                clearInterval(checkInterval)
                reject(new DOMException('Operation cancelled by user', 'AbortError'))
            }
        }, 100)

        promise.finally(() => clearInterval(checkInterval))
        timeoutPromise.finally(() => clearInterval(checkInterval))
    })

    try {
        const result = await Promise.race([promise, timeoutPromise, cancelCheckPromise])
        clearTimeout(timeoutId)
        return result as T
    } catch (error) {
        clearTimeout(timeoutId)
        throw error
    }
}

export const loadVideo = async (
    media: Api.MediaItem,
    sources: Plugins.IndexerSource[] | null,
    seasonNumber: number | null = null,
    episode: Api.Episode | null = null,
    targeted = false,
    collectionItems: Api.CollectionItem[] = [],
): Promise<void> => {
    shouldCancelVideoLoad.set(false)

    loadingStates.update((states) => ({
        ...states,
        isPlayButtonLoading: true,
    }))

    const currentUser = get(user)

    let seasonNum = seasonNumber
    let episodeData = episode
    let episodeNum = episodeData?.episode_num ?? null
    let season = null
    let mediaSources = sources
    let showWatched = false
    let localMedia = false

    const isCancelled = (): boolean => get(shouldCancelVideoLoad)
    let overallTimeoutId: NodeJS.Timeout | undefined = undefined

    if (get(parentalControlsAreEnabled)) {
        const { tv, movies } = get(restrictedContent)

        if (!media.content_rating) {
            throw createError('Parental Lock: Content restricted due to unknown content rating', 403, {
                log: false,
            })
        }

        if (tv.includes(media.content_rating) || movies.includes(media.content_rating)) {
            throw createError('Parental Lock: Content restricted due to content rating', 403, {
                log: false,
            })
        }
    }

    try {
        const overallTimeoutPromise = new Promise<never>((_, reject) => {
            overallTimeoutId = setTimeout(() => {
                reject(new Error(`Overall operation timed out after ${TIMEOUTS.TOTAL_OPERATION}ms`))
            }, TIMEOUTS.TOTAL_OPERATION)
        })

        await Promise.race([
            (async () => {
                checkCancellation()

                if (!currentUser) {
                    throw createError('Missing user', 401, {
                        log: false,
                    })
                }

                if (!targeted) {
                    loadingStates.set({
                        ...get(loadingStates),
                        isVideoLoading: true,
                    })
                }

                const backdrop = media.backdrop ? getImagePath(media.backdrop) : null

                const playlistItem: Video.PlaylistItem = {
                    videoUrl: null,
                    filename: null,
                    files: [],
                    backdrop,
                    seasonNumber,
                    episode: episodeData,
                    language: null,
                    infohash: null,
                    resolver: null,
                    playlistIndex: 0,
                    media,
                    collectionItems,
                    collectionIndex: 0,
                }

                let metadata = {
                    ...playlistItem,
                    media,
                    playlist: [playlistItem],
                }

                videoMetadata.set(metadata)

                if (media.type === 'tv') {
                    if (!media.seasons) {
                        checkCancellation()

                        const resp = await withTimeout(fetchSeasonData(media.id), TIMEOUTS.SEASON_DATA, isCancelled)
                        if (!resp.success) throw resp.error
                        media.seasons = resp.data
                    }

                    if (get(shuffleSettings).randomEpisodes) {
                        if (!media.seasons) return
                        const result = getRandomEpisode(media.seasons)
                        if (result) {
                            seasonNum = result.seasonNumber
                            episodeData = result.episode
                            episodeNum = episodeData.episode_num
                        }
                    } else {
                        const isShowCompletelyWatched = checkIfCompletelyWatched(media)
                        if (isShowCompletelyWatched) showWatched = true

                        season = selectSeason(media, showWatched)

                        if (season && !episodeData) {
                            episodeData = getFirstUnwatchedEpisode(season, media, showWatched)
                        }

                        seasonNum = seasonNumber ? seasonNumber : (season?.season_num ?? 1)
                        episodeNum = episodeNum ? episodeNum : (episodeData?.episode_num ?? 1)

                        media.progress =
                            media.continue_watching_episode_id === episodeData?.episode_id ? media.progress : null
                    }

                    playlistItem.seasonNumber = seasonNum ?? null
                    playlistItem.episode = episodeData ?? null
                }

                if (!mediaSources || mediaSources.length === 0) {
                    checkCancellation()

                    const imdbId = episode?.imdb_id ? episode.imdb_id : media.imdb_id

                    const localResults = await fetchLocalMedia(
                        imdbId,
                        media.title,
                        media.released,
                        media.type,
                        seasonNum,
                        episodeNum,
                    )

                    if (localResults.length > 0) {
                        mediaSources = localResults
                        localMedia = true
                    } else {
                        const resp = await withTimeout(
                            fetchSources(
                                imdbId,
                                media.title,
                                media.released,
                                media.type,
                                seasonNum,
                                episodeNum,
                                episodeData?.episode_id ?? null,
                                true,
                            ),
                            TIMEOUTS.SOURCES_FETCH,
                            isCancelled,
                        )

                        if (!resp.success) throw resp.error

                        if (resp.success && resp.data.length > 0) {
                            mediaSources = resp.data
                        } else if (resp.success && resp.data.length === 0) {
                            throw createError(`No sources found for ${media.imdb_id}`, 404, {
                                log: false,
                            })
                        }
                    }
                }

                if (!mediaSources) {
                    throw createError(`No sources found for ${media.imdb_id}`, 404, {
                        log: false,
                    })
                }

                const originalSeasonNumber = episode?.original_season_num ? episode.original_season_num : seasonNum
                const originalEpisodeNumber = episode?.original_episode_num ? episode.original_episode_num : episodeNum
                const imdbId = episode?.imdb_id ?? media.imdb_id

                let videoData
                if (localMedia || mediaSources[0].source === 'Local Media') {
                    videoData = {
                        videoUrl: mediaSources[0].filePath,
                        filename: mediaSources[0].filename,
                        files: [],
                        infohash: null,
                        resolver: 'Local Media',
                    }
                } else {
                    videoData = await withTimeout(
                        fetchVideoFromSources(
                            {
                                imdbId,
                                title: media.title,
                                released: media.released,
                                type: media.type,
                                seasonNumber: originalSeasonNumber,
                                episodeNumber: originalEpisodeNumber,
                                episodeId: episodeData?.episode_id ?? null,
                                skipErrors: !targeted,
                            },
                            mediaSources,
                        ),
                        TIMEOUTS.VIDEO_URL_FETCH,
                        isCancelled,
                    )
                }

                Object.assign(playlistItem, videoData)

                if (!playlistItem.videoUrl) {
                    throw createError(`No Video File Found`, 400, {
                        log: false,
                    })
                }

                if (targeted) {
                    loadingStates.update((states) => ({
                        ...states,
                        isVideoLoading: true,
                    }))
                }

                checkCancellation()

                if (media.language) {
                    const languageCode = getLanguageCodeByName(media.language)
                    playlistItem.language = languageCode
                }

                metadata = {
                    ...playlistItem,
                    media,
                    playlist: [playlistItem],
                }

                videoMetadata.set(metadata)

                const resp = await withTimeout(
                    invokeFunction('load_video', {
                        value: {
                            url: playlistItem.videoUrl,
                            userId: currentUser.id,
                            videoLanguage: playlistItem.language,
                            progress: media.progress ?? null,
                        },
                    }),
                    TIMEOUTS.INVOKE_FUNCTION,
                    isCancelled,
                )
                if (!resp.success) throw resp.error

                checkCancellation()

                goto(`/video`)
            })(),
            overallTimeoutPromise,
        ])
    } catch (error) {
        videoMetadata.set(get(DEFAULT_VIDEO_METADATA))

        loadingStates.update((states) => ({
            ...states,
            isVideoLoading: false,
        }))

        shouldCancelVideoLoad.set(false)

        if (error instanceof Error && error.message.includes('Transfer Started.')) return

        if (error instanceof DOMException && error.name === 'AbortError') return

        if (error instanceof Error && error.message.includes('timed out')) {
            handleError(error, {
                context: 'loading a video timed out',
            })
            return
        }

        handleError(error, {
            context: 'loading a video failed',
        })
    } finally {
        if (overallTimeoutId) clearTimeout(overallTimeoutId)

        loadingStates.update((states) => ({
            ...states,
            isPlayButtonLoading: false,
        }))
    }
}
