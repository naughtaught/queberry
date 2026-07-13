import type { Api } from '$lib/types/api'

export const selectSeason = (
    media: Api.MediaItem,
    showWatched = false,
    hasCurrentSeason: number | null = null,
): Api.Season | null => {
    if (media.type !== 'tv' || !media.seasons) return null
    if (!media.seasons.seasons || !Array.isArray(media.seasons.seasons)) return null

    const preferredEpisodeKey = media.episode_group_name ? `${media.episode_group_name}` : 'default_episodes'

    const currentSeason = media.seasons.seasons
        .filter((season: Api.Season) => season.season_num !== 0)
        .find((season: Api.Season) => {
            if (hasCurrentSeason !== null) return season.season_num === hasCurrentSeason

            let episodes = season[preferredEpisodeKey]

            if (!Array.isArray(episodes)) episodes = season.default_episodes

            if (showWatched) {
                return Array.isArray(episodes)
            } else {
                return Array.isArray(episodes) && episodes.some((episode) => !episode.watched)
            }
        })

    return currentSeason ?? null
}
