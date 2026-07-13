<script lang="ts">
    import { updateCachedMedia } from '$lib/functions/cache/updateCachedMedia'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { setMediaStates } from '$lib/functions/states/setMediaStates'
    import FavouriteIcon from 'virtual:icons/material-symbols/favorite'

    let { media = $bindable(), size = 'text-2xl', background = '' } = $props()

    const setState = async (event: Event): Promise<void> => {
        event.preventDefault()
        event.stopPropagation()

        const preChanges = media

        try {
            media = {
                ...preChanges,
                in_watchlist: false,
                in_collection: !preChanges.in_collection,
            }

            const resp = await setMediaStates(preChanges, {
                in_watchlist: false,
                hidden: preChanges.hidden,
                watched: preChanges.watched,
                in_collection: !preChanges.in_collection,
                progress: preChanges.progress,
                episode_id: preChanges.continue_watching_episode_id ?? null,
                rating: preChanges.rating,
            })
            if (!resp.success) throw resp.error
            if (resp.data) media = { ...resp.data }
            updateCachedMedia(media)
        } catch (error) {
            media = { ...preChanges }
            handleError(error)
        }
    }
</script>

<button
    onclick={(event: Event) => {
        setState(event)
    }}
    class="flex items-center rounded"
    aria-label="Set Favourite State">
    <FavouriteIcon
        class="{size} rounded hover:bg-primaryColor/50 {background} {media.in_collection
            ? 'text-rose-500'
            : 'text-textColor'}" />
</button>
