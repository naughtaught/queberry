import type { Api } from '$lib/types/api'

export const checkIfCompletelyWatched = (media: Api.MediaItem): boolean => {
    if (media.type !== 'tv' || !media.seasons) return false
    if (!media.seasons.seasons || !Array.isArray(media.seasons.seasons)) return false

    const preferredEpisodeKey = media.episode_group_name ? `${media.episode_group_name}` : 'default_episodes'
    const now = new Date()

    return media.seasons.seasons
        .filter((season: Api.Season) => season.season_num !== 0)
        .every((season: Api.Season) => {
            let episodes = season[preferredEpisodeKey]

            if (!Array.isArray(episodes)) episodes = season.default_episodes

            if (!Array.isArray(episodes)) return false

            const releasedEpisodes = episodes.filter((episode) => {
                if (!episode.air_date) return false
                return new Date(episode.air_date) <= now
            })

            if (releasedEpisodes.length === 0) return true

            return releasedEpisodes.every((episode) => episode.watched === true)
        })
}
