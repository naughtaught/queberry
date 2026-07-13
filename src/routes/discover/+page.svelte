<script lang="ts">
    import { navigating, page } from '$app/state'
    import GridViewSideBar from '$lib/components/bars/GridViewSideBar.svelte'
    import GridView from '$lib/components/ui/GridView.svelte'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import { isValidRoute } from '$lib/functions/utility/isValidRoute.js'
    import { loadingStates } from '$lib/stores/app.js'
    import { activeGridViewSideBarItem, cache } from '$lib/stores/pages.js'
    import type { Api } from '$lib/types/api'
    import type { App } from '$lib/types/app'
    import { onDestroy, onMount } from 'svelte'

    const { data } = $props()

    let mediaData = $derived<Api.MediaItem[]>($cache['discover'].media)
    let isMounted = $state(false)

    onMount(() => {
        isMounted = true

        if (data.success && Array.isArray(data.data)) {
            if (navigating.type !== 'popstate') $activeGridViewSideBarItem = mediaData[0]
            $cache['discover'].media = [...$cache['discover'].media]
            $activeGridViewSideBarItem = $cache['discover'].media[0]
            $loadingStates.isGridViewLoading = false
        }

        return () => {
            isMounted = false
        }
    })

    $effect(() => {
        if (!isMounted) return

        if (data.success && data.data instanceof Promise) {
            data.data.then(() => {
                if (isMounted) {
                    $cache['discover'].media = [...$cache['discover'].media]
                    $activeGridViewSideBarItem = $cache['discover'].media[0]
                    $loadingStates.isGridViewLoading = false
                }
            })
        }
    })

    onDestroy(() => {
        const route = page.url.pathname?.replace('/', '')
        if (!isValidRoute(route)) return

        $cache[route as keyof App.Cache].lastUpdated = Date.now()
        $loadingStates.isGridViewLoading = true
    })
</script>

<section class="flex w-full flex-col">
    {#if $loadingStates.isGridViewLoading}
        <div class="-ml-39.75 flex min-h-full min-w-full items-center">
            <LoadingSpinner />
        </div>
    {:else if mediaData.length}
        <div class="flex w-full flex-col">
            <div class="flex w-full">
                <div class="flex-1">
                    <GridView bind:mediaData />
                </div>
                {#if $activeGridViewSideBarItem}
                    <GridViewSideBar />
                {/if}
            </div>
        </div>
    {:else}
        <div class="-ml-39.75 min-w-full">
            <p class="mx-auto flex h-screen cursor-default items-center justify-center text-center text-slate-300">
                No media found
            </p>
        </div>
    {/if}
</section>
