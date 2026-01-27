<script lang="ts">
    import { MediaStateBaseButton } from '$lib'

    let { item = $bindable(), label, text = '', hasBackground = false } = $props()

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

<MediaStateBaseButton
    path="M280-120q-33 0-56.5-23.5T200-200v-520h-40v-80h200v-40h240v40h200v80h-40v520q0 33-23.5 56.5T680-120H280Zm400-600H280v520h400v-520ZM360-280h80v-360h-80v360Zm160 0h80v-360h-80v360ZM280-720v520-520Z"
    {label}
    {text}
    {hasBackground}
    func={(event: Event) => {
        setState(event)
    }} />
