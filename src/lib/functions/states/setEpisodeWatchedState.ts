import type { App } from '$lib/types/app'
import type { Api } from '$lib/types/api'
import { isEpisodeReleased } from '$lib/functions/utility/isEpisodeReleased'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { createError, formatError } from '$lib/functions/errors/errorHandling'
import { validateUser } from '$lib/functions/user/validateUser'

export const setEpisodeWatchedState = async (
    episode: Api.Episode | Api.Episode[],
    media: Api.MediaItem,
    watched: boolean,
    isManual = false,
): Promise<App.Response> => {
    try {
        let episodeIds

        const currentUser = validateUser()
        if (!currentUser) throw createError('Missing User', 401, { log: false })

        const episodes = Array.isArray(episode) ? episode : [episode]

        if (isManual) {
            episodeIds = episodes.map((ep) => ep.episode_id)
        } else {
            episodeIds = episodes.filter((x) => isEpisodeReleased(x.air_date)).map((ep) => ep.episode_id)
        }

        if (watched) {
            const response = await invokeFunction('api_upsert_watched_episodes', {
                postgresId: currentUser.postgresId,
                token: currentUser.token,
                mediaId: media.id,
                episodeIds,
            })
            if (!response.success) throw response.error
        } else {
            const response = await invokeFunction('api_delete_watched_episode_ids', {
                postgresId: currentUser.postgresId,
                token: currentUser.token,
                mediaId: media.id,
                episodeIds,
            })
            if (!response.success) throw response.error
        }

        const episodeIdSet = new Set(episodeIds)
        const updatedMedia: Api.MediaItem = {
            ...media,
            seasons: {
                ...media.seasons,
                episode_group_keys: media.seasons?.episode_group_keys ?? null,
                seasons: (media.seasons?.seasons ?? []).map((season) => {
                    const updatedSeason = { ...season }
                    const groupKeys = media.seasons?.episode_group_keys ?? []
                    updatedSeason.default_episodes = season.default_episodes.map((ep) =>
                        episodeIdSet.has(ep.episode_id) ? { ...ep, watched } : ep,
                    )
                    for (const key of groupKeys) {
                        const episodeKey = `${key.toLowerCase().replace(/\s+/g, '_')}` as keyof typeof season
                        if (Array.isArray(season[episodeKey])) {
                            updatedSeason[episodeKey] = (season[episodeKey] as Api.Episode[]).map((ep) =>
                                episodeIdSet.has(ep.episode_id) ? { ...ep, watched } : ep,
                            )
                        }
                    }
                    return updatedSeason
                }),
            },
        }
        return {
            success: true,
            data: {
                media: updatedMedia,
                episode: Array.isArray(episode) ? episode.map((ep) => ({ ...ep, watched })) : { ...episode, watched },
            },
        }
    } catch (error) {
        return formatError(error)
    }
}
