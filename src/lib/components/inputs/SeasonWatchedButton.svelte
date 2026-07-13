<script lang="ts">
    import { fetchUpNext } from '$lib/db/fetchUpNext'
    import { updateCachedMedia } from '$lib/functions/cache/updateCachedMedia'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { setEpisodeWatchedState } from '$lib/functions/states/setEpisodeWatchedState'
    import { setMediaStates } from '$lib/functions/states/setMediaStates'
    import { getSelectedSeasonsEpisodes } from '$lib/functions/utility/getSelectedSeasonsEpisodes'
    import { selectSeason } from '$lib/functions/utility/selectSeason'
    import { loadingStates } from '$lib/stores/app'
    import type { Api } from '$lib/types/api'

    let {
        media = $bindable(),
        selectedSeasonEpisodes = $bindable(),
        selectedSeason = $bindable(),
        showWatchedEpisodes,
    } = $props()

    const hasUnwatchedEpisodes = $derived(
        selectedSeasonEpisodes.some((episode: { watched: boolean | null }) => !episode.watched),
    )

    const setSeasonWatchedState = async (): Promise<void> => {
        try {
            $loadingStates.isEpisodesLoading = true

            if (!media.watched) {
                const resp = await setMediaStates(media, {
                    watched: !media.watched,
                    hidden: false,
                    in_watchlist: false,
                    progress: null,
                    episode_id: null,
                    rating: media.rating,
                })
                if (!resp.success) throw resp.error

                media = { ...resp.data }
            }

            const selectedSeasonData = media.seasons?.seasons?.find(
                (season: Api.Season) => season.season_num === selectedSeason.season_num,
            )

            const episodesToUpdate = media.episode_group_name ? `${media.episode_group_name}` : 'default_episodes'

            const isAllWatched = selectedSeasonData[episodesToUpdate].every(
                (episode: { watched: boolean }) => episode.watched,
            )

            const response = await setEpisodeWatchedState(selectedSeasonData[episodesToUpdate], media, !isAllWatched)

            if (!response.success) throw response.error

            media = { ...response.data.media }

            const isAnyWatched =
                media.seasons?.seasons?.some((season: Api.Season) => {
                    return Object.values(season).some((value) => {
                        if (Array.isArray(value) && value.length > 0 && 'watched' in (value[0] || {})) {
                            return value.some((episode: { watched: boolean }) => episode.watched)
                        }
                        return false
                    })
                }) ?? false

            if (!isAnyWatched) {
                const resp = await setMediaStates(media, {
                    watched: false,
                    hidden: false,
                    in_watchlist: false,
                    progress: null,
                    episode_id: null,
                    rating: media.rating,
                })
                if (!resp.success) throw resp.error

                media = { ...resp.data }
            }

            updateCachedMedia(media)

            if (!showWatchedEpisodes) {
                selectedSeason = selectSeason(media, showWatchedEpisodes)
                selectedSeasonEpisodes = getSelectedSeasonsEpisodes(selectedSeason, media)
            }

            await fetchUpNext()
        } catch (error) {
            handleError(error)
        } finally {
            $loadingStates.isEpisodesLoading = false
        }
    }
</script>

<button
    onclick={setSeasonWatchedState}
    class="{hasUnwatchedEpisodes
        ? 'border-white/20 text-slate-500'
        : 'border-green-500/20 text-green-500'} rounded-sm border-2 px-2 text-center text-[8px] font-bold uppercase">
    Watched
</button>
