<script lang="ts">
    import { fetchUpNext } from '$lib/db/fetchUpNext'
    import { updateCachedMedia } from '$lib/functions/cache/updateCachedMedia'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { setEpisodeWatchedState } from '$lib/functions/states/setEpisodeWatchedState'
    import { setMediaStates } from '$lib/functions/states/setMediaStates'
    import type { Api } from '$lib/types/api'
    import WatchedIcon from 'virtual:icons/material-symbols/check-circle-outline'

    let { episode = $bindable(), media = $bindable() } = $props()

    const handleClick = async (event: MouseEvent | KeyboardEvent): Promise<void> => {
        event.stopPropagation()

        try {
            const response = await setEpisodeWatchedState(episode, media, !episode.watched, true)
            if (!response.success) throw response.error

            if (response.success) {
                episode = { ...response.data.episode }
                media = { ...response.data.media }
                if (episode.watched) {
                    if (!media.watched) {
                        const resp = await setMediaStates(media, { watched: true, in_watchlist: false, hidden: false })
                        if (!resp.success) throw resp.error
                        if (resp.data) media = { ...resp.data }
                    }
                } else {
                    const hasWatchedEpisodes = media.seasons.seasons.some((season: Api.Season) =>
                        season.default_episodes.some((e) => e.watched === true),
                    )
                    if (!hasWatchedEpisodes) {
                        const resp = await setMediaStates(media, {
                            watched: false,
                            progress: null,
                            episode_id: null,
                            rating: null,
                        })
                        if (!resp.success) throw resp.error
                        if (resp.data) media = { ...resp.data }
                    }
                }
                updateCachedMedia(media)

                await fetchUpNext()
            }
        } catch (error) {
            handleError(error)
        }
    }
</script>

<button onclick={(event) => handleClick(event)} class="rounded bg-black/30 p-1 hover:bg-primaryColor/40">
    <WatchedIcon class="text-2xl  {episode.watched ? 'text-green-500 ' : 'text-slate-200 '} " />
</button>
