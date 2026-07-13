<script lang="ts">
    import { fetchSeasonData } from '$lib/db/fetchSeasons'
    import { fetchUpNext } from '$lib/db/fetchUpNext'
    import { updateCachedMedia } from '$lib/functions/cache/updateCachedMedia'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { setEpisodeWatchedState } from '$lib/functions/states/setEpisodeWatchedState'
    import { setMediaStates } from '$lib/functions/states/setMediaStates'
    import { loadingStates } from '$lib/stores/app'
    import type { Api } from '$lib/types/api'

    import WatchedIcon from 'virtual:icons/material-symbols/check-circle-outline'

    let { media = $bindable(), size = 'text-2xl', background = '' } = $props()

    const setState = async (event: Event): Promise<void> => {
        event.preventDefault()
        event.stopPropagation()

        const preChanges = media

        try {
            media = { ...media, watched: !media.watched, hidden: false, in_watchlist: false }

            const resp = await setMediaStates(preChanges, {
                watched: !preChanges.watched,
                hidden: false,
                in_watchlist: false,
                progress: null,
                episode_id: null,
                rating: preChanges.rating,
                in_collection: preChanges.in_collection,
            })

            if (!resp.success) throw resp.error
            if (resp.success) {
                updateCachedMedia(media)
                if (media.watched && media.type === 'tv') {
                    const seasonResp = await fetchSeasonData(media.id)
                    if (!seasonResp.success) throw seasonResp.error

                    resp.data.seasons = seasonResp.data

                    media = { ...resp.data }

                    $loadingStates.isEpisodesLoading = true

                    const allEpisodes = (media.seasons?.seasons ?? []).flatMap(
                        (season: Api.Season) => season.default_episodes,
                    )

                    const response = await setEpisodeWatchedState(allEpisodes, media, true)
                    if (!response.success) throw response.error
                    media = { ...response.data.media }
                }

                if (!media.watched && media.type === 'tv') {
                    $loadingStates.isEpisodesLoading = true
                    const episodesToUpdate = media.episode_group_name
                        ? `${media.episode_group_name}`
                        : 'default_episodes'
                    const allEpisodes = (media.seasons?.seasons ?? [])
                        .filter((season: Api.Season) => season.season_num !== 0)
                        .flatMap((season: Api.Season) => season[episodesToUpdate] || [])
                    const response = await setEpisodeWatchedState(allEpisodes, media, false)
                    if (!response.success) throw response.error
                    media = { ...response.data.media }
                }
                updateCachedMedia(media)

                if (media.type === 'tv') await fetchUpNext()
            }
        } catch (error) {
            media = { ...preChanges }
            handleError(error)
        } finally {
            $loadingStates.isEpisodesLoading = false
        }
    }
</script>

<button
    onclick={(event: Event) => {
        setState(event)
    }}
    class="flex items-center rounded"
    aria-label="Set Watched State">
    <WatchedIcon
        class="{size} rounded {background} hover:bg-primaryColor/50 {media.watched
            ? 'text-green-500'
            : 'text-textColor'}" />
</button>
