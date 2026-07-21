import type { Api } from '$lib/types/api'

export const getSelectedSeasonsEpisodes = (season: Api.Season, media: Api.MediaItem): Api.Episode[] => {
    if (!season) return []

    const episodeKey = media?.episode_group_name
        ? `${media.episode_group_name.toLowerCase().replace(/\s+/g, '_')}_episodes`
        : 'default_episodes'

    const episodes = season[episodeKey as keyof Api.Season]

    return Array.isArray(episodes) ? (episodes as Api.Episode[]) : season.default_episodes
}
