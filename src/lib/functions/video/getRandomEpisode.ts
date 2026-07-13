import { shuffleSettings } from '$lib/stores/pages'
import type { Api } from '$lib/types/api'
import { get } from 'svelte/store'

export const getRandomEpisode = (seasons: Api.SeasonData): { seasonNumber: number; episode: Api.Episode } | null => {
    const playedIds = get(shuffleSettings).playedEpisodeIds || []

    const available = seasons.seasons
        .filter((season) => season.default_episodes.some((episode) => episode.is_finale))
        .flatMap((season) =>
            season.default_episodes
                .filter((episode) => !playedIds.includes(episode.episode_id))
                .map((episode) => ({
                    seasonNumber: season.season_num,
                    episode,
                })),
        )

    if (available.length === 0) return null

    return available[Math.floor(Math.random() * available.length)]
}
