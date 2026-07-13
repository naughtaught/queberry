import { settings } from '$lib/stores/user'
import { videoMetadata } from '$lib/stores/video'
import type { Api } from '$lib/types/api'
import { get } from 'svelte/store'
import { getSelectedSeasonsEpisodes } from '$lib/functions/utility/getSelectedSeasonsEpisodes'

export const getNextEpisode = (): {
    newSeasonNumber: number | null
    newEpisode: Api.Episode | null
    newEpisodeNumber: number | null
} => {
    const userSettings = get(settings)

    const nullResult = { newSeasonNumber: null, newEpisode: null, newEpisodeNumber: null }

    const metadata = get(videoMetadata)
    if (!metadata.media) return nullResult

    const { episode, seasonNumber: currentSeasonNumber, media } = metadata
    const currentEpisodeNumber = episode?.episode_num
    const seasons = media?.seasons?.seasons

    if (!currentEpisodeNumber || !currentSeasonNumber || !seasons) return nullResult

    const currentSeason = seasons.find((s) => s.season_num === currentSeasonNumber)
    if (!currentSeason) return nullResult

    const currentEpisodes = getSelectedSeasonsEpisodes(currentSeason, metadata.media)
    const nextInSeason = currentEpisodes.find((e) => e.episode_num === currentEpisodeNumber + 1)

    if (nextInSeason) {
        return {
            newSeasonNumber: currentSeasonNumber,
            newEpisode: nextInSeason,
            newEpisodeNumber: nextInSeason.episode_num,
        }
    }

    const nextSeason = seasons.find((s) => s.season_num === currentSeasonNumber + 1)
    if (!nextSeason) return nullResult

    const nextSeasonEpisodes = getSelectedSeasonsEpisodes(nextSeason, metadata.media)
    const firstEpisode = [...nextSeasonEpisodes].sort((a, b) => a.episode_num - b.episode_num)[0]

    if (!firstEpisode) return nullResult

    if (userSettings.seasonCompletionRequired) {
        const doesSeasonHaveAFinale = nextSeasonEpisodes.find((x: Api.Episode) => x.is_finale)
        if (!doesSeasonHaveAFinale) return nullResult
    }

    return {
        newSeasonNumber: currentSeasonNumber + 1,
        newEpisode: firstEpisode,
        newEpisodeNumber: firstEpisode.episode_num,
    }
}
