<script lang="ts">
    import DeleteIcon from 'virtual:icons/material-symbols/delete-forever-outline'

    let { item = $bindable(), label, text = '', textColor = 'text-textColor' } = $props()

    // TODO
    const setState = (event: Event): void => {
        event.preventDefault()
        // const origin = item

        // item = {
        //     ...item,
        //     watched: false,
        //     hidden: !item.hidden,
        //     watchlist: false,
        // }

        // onupdate(item)

        // try {
        //     await setWatchedState(item, 'hidden', origin.hidden)
        // } catch (error) {
        //     item = { ...origin }
        //     return
        // }

        // if (origin.watchlist) await setWatchlistState(item, true)

        // await updateLocalMediaStates(item, 'hidden', item.hidden)

        // if (item.watched || item.hidden || item.watchlist) await removeFromContinueWatching(item.id)

        // if (item.type === 'tv') await getUpNextCarouselData()

        // if (page.url.pathname !== '/details') await preloadData(`/details`)

        // if (adjustOffset && !$filters.includeHidden) {
        //     item.hidden ? $offsetAdjustment++ : ($offsetAdjustment = Math.max(0, $offsetAdjustment - 1))
        // }
    }
</script>

<!-- TODO fill when state is true -->
<button
    onclick={(event: Event) => {
        setState(event)
    }}
    class="group flex items-center rounded"
    aria-label={label}>
    <DeleteIcon class="{textColor} text-2xl transition-colors hover:text-red-500" />
    {text}
</button>
