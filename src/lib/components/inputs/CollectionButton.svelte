<script lang="ts">
    import { MediaStateBaseButton } from '$lib'

    let { item = $bindable(), label, text = '', hasBackground = false } = $props()

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

<MediaStateBaseButton
    path="M240-144v-600q0-30 21-51t51-21h336q30 0 51 21t21 51v600l-240-96-240 96Z"
    {label}
    {text}
    {hasBackground}
    func={(event: Event) => {
        setState(event)
    }} />
