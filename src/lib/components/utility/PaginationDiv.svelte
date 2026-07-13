<script lang="ts">
    import { page } from '$app/state'
    import { fetchData } from '$lib/db/fetchData'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { isValidRoute } from '$lib/functions/utility/isValidRoute'
    import { viewport } from '$lib/functions/utility/useViewport'
    import { loadingStates } from '$lib/stores/app'
    import { cache } from '$lib/stores/pages'

    let { mediaData = $bindable() } = $props()

    let observeViewport = $state(true)

    const onViewportEntry = async (): Promise<void> => {
        if (!observeViewport) return

        const route = page.url.pathname?.replace('/', '')

        if (!isValidRoute(route)) return

        $loadingStates.isGridViewLoadingMore = true

        $cache[route].paginationPage++

        try {
            const resp = await fetchData(false)
            if (!resp.success) throw resp.error

            mediaData = [...mediaData, ...resp.data]

            $cache[route].media = [...$cache[route].media, ...resp.data]
            $cache[route].lastUpdated = Date.now()

            observeViewport = false
        } catch (err) {
            $cache[route].paginationPage--
            handleError(err)
            observeViewport = true
        } finally {
            $loadingStates.isGridViewLoadingMore = false
        }
    }
</script>

{#if observeViewport}
    <div class="m-0 h-0 w-0 p-0 opacity-0" use:viewport onenterViewport={onViewportEntry}></div>
{/if}
