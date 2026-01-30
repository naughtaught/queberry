<script lang="ts">
    import CollectionIcon from 'virtual:icons/material-symbols/bookmark-add'

    let { item = $bindable(), label, text = '', textColor = 'text-textColor' } = $props()

    // TODO
    const setState = (event: Event): void => {
        event.preventDefault()
        // const origin = item
        // item = {
        //     ...item,
        //     watchlist: false,
        //     collection: !item.collection,
        // }
        // onupdate(item)
        // try {
        //     await setCollectionState(item, origin.collection)
        // } catch (error) {
        //     setErrorMessage({
        //         displayMessage: 'Error setting collection state',
        //         writeMessage: `[CollectionButton - setState]: ${error}`,
        //     })
        //     item = { ...origin }
        //     return
        // }
        // if (origin.collection) await setWatchlistState(item, true)
        // await updateLocalMediaStates(item, 'collection', item.collection)
        // if ($modals.search && page.url.pathname === '/details') {
        //     const urlParams = new URLSearchParams(page.url.search)
        //     const currentPageId = urlParams.get('id')
        //     if (currentPageId === item.id.toString()) await invalidateAll()
        // }
        // if (page.url.pathname !== '/details') await preloadData(`/details`)
        // if (page.url.pathname === '/collection') {
        //     item.collection ? $offsetAdjustment-- : $offsetAdjustment++
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
    <CollectionIcon class="{textColor} text-2xl transition-colors hover:text-purple-700" />
    {text}
</button>
