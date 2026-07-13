<script lang="ts">
    import { fetchUpNext } from '$lib/db/fetchUpNext'
    import { updateCachedMedia } from '$lib/functions/cache/updateCachedMedia'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { setMediaStates } from '$lib/functions/states/setMediaStates'
    import DeleteIcon from 'virtual:icons/material-symbols/delete-forever-outline'

    let { media = $bindable(), size = 'text-2xl', background = '' } = $props()

    const setState = async (event: Event): Promise<void> => {
        event.preventDefault()
        event.stopPropagation()

        const preChanges = media

        try {
            media = {
                ...media,
                watched: false,
                hidden: !media.hidden,
                in_watchlist: false,
            }

            const resp = await setMediaStates(preChanges, {
                watched: false,
                hidden: !preChanges.hidden,
                in_watchlist: false,
                in_collection: preChanges.in_collection,
                episode_id: null,
                progress: null,
                rating: preChanges.rating,
            })
            if (!resp.success) throw resp.error
            if (resp.data) media = { ...resp.data }
            updateCachedMedia(media)

            if (media.type === 'tv' && preChanges.watched) await fetchUpNext()
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
    aria-label="Set Hidden State">
    <DeleteIcon
        class="{size} rounded hover:bg-primaryColor/50 {background} {media.hidden
            ? 'text-red-500'
            : 'text-textColor'}" />
</button>
