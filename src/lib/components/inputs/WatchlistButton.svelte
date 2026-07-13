<script lang="ts">
    import { fetchUpNext } from '$lib/db/fetchUpNext'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { updateCachedMedia } from '$lib/functions/cache/updateCachedMedia'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { setMediaStates } from '$lib/functions/states/setMediaStates'
    import { validateUser } from '$lib/functions/user/validateUser'
    import { loadingStates } from '$lib/stores/app'
    import type { Api } from '$lib/types/api'
    import WatchlistIcon from 'virtual:icons/material-symbols/bookmark-add'

    let { media = $bindable(), size = 'text-2xl', background = '' } = $props()

    const setState = async (event: Event): Promise<void> => {
        event.preventDefault()
        event.stopPropagation()

        const preChanges = media

        try {
            const currentUser = validateUser()
            if (!currentUser) return

            media = { ...media, in_watchlist: !media.in_watchlist, watched: false, hidden: false, in_collection: false }

            const resp = await setMediaStates(media, {
                in_watchlist: !preChanges.in_watchlist,
                watched: false,
                hidden: false,
                in_collection: false,
                episode_id: null,
                progress: null,
                rating: null,
            })
            if (!resp.success) throw resp.error
            if (resp.success) {
                media = { ...resp.data }
                if (preChanges.watched && media.type === 'tv') {
                    $loadingStates.isEpisodesLoading = true
                    const response = await invokeFunction('api_delete_watched_episodes', {
                        postgresId: currentUser.postgresId,
                        token: currentUser.token,
                        mediaId: media.id,
                    })
                    if (!response.success) throw response.error
                    const episodeGroups = ['default_episodes', ...(resp.data.episode_group_keys || [])]
                    media = {
                        ...resp.data,
                        seasons: resp.data.seasons.map((season: Api.Season) => {
                            const updatedSeason = { ...season }

                            episodeGroups.forEach((groupName) => {
                                const episodes = updatedSeason[groupName]
                                if (Array.isArray(episodes)) {
                                    updatedSeason[groupName] = episodes.map((episode: Api.Episode) => ({
                                        ...episode,
                                        watched: false,
                                    }))
                                }
                            })

                            return updatedSeason
                        }),
                    }
                }
                updateCachedMedia(media)
                if (media.type === 'tv' && preChanges.watched) await fetchUpNext()
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
    aria-label="Set Watchlist State">
    <WatchlistIcon
        class="{size} rounded hover:bg-primaryColor/50 {background} {media.in_watchlist
            ? 'text-purple-500'
            : 'text-textColor'}" />
</button>
