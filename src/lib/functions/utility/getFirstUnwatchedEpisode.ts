import type { Api } from '$lib/types/api'

export const getFirstUnwatchedEpisode = (
    selectedSeason: Api.Season,
    media: Api.MediaItem,
    showWatched = false,
): Api.Episode | null => {
    if (!selectedSeason) return null

    const transformedGroupName = media?.episode_group_name
        ? media.episode_group_name.toLowerCase().replace(/\s+/g, '_')
        : null

    const episodeKey = transformedGroupName ? `${transformedGroupName}_episodes` : 'default_episodes'

    const episodes =
        (selectedSeason[episodeKey as keyof Api.Season] as Api.Episode[] | undefined) ?? selectedSeason.default_episodes

    if (showWatched) return episodes[0]

    return episodes?.find((episode) => !episode.watched) ?? null
}
