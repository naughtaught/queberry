<script lang="ts">
    import { afterNavigate } from '$app/navigation'
    import { resolve } from '$app/paths'
    import { page } from '$app/state'
    import { getImagePath } from '$lib/functions/ui/getImagePath'
    import { posterSize } from '$lib/functions/ui/posterSize'
    import { activeGridViewSideBarItem, detailsMedia } from '$lib/stores/pages'
    import { settings } from '$lib/stores/user'
    import type { Api } from '$lib/types/api'
    import FavouriteButton from '$lib/components/inputs/FavouriteButton.svelte'
    import HideButton from '$lib/components/inputs/HideButton.svelte'
    import WatchedButton from '$lib/components/inputs/WatchedButton.svelte'
    import WatchlistButton from '$lib/components/inputs/WatchlistButton.svelte'
    import PaginationDiv from '$lib/components/utility/PaginationDiv.svelte'

    let { media, index, dataLength, mediaData = $bindable() } = $props()
    let x = $state(0)
    const imagescaling = $derived($settings.imageScaling)
    const minPosterHeightPixels = 210
    const maxPosterViewHeight = $derived.by(() => {
        if (x >= 1440) return 34
        if (x >= 1080) return 33
        return 30
    })
    const scale = $derived(Math.max(0.5, Math.min(2, (imagescaling ?? 100) / 100)))
    const minHpx = $derived(minPosterHeightPixels * scale)
    const maxHvh = $derived(maxPosterViewHeight * scale)
    const minWvw = $derived(maxHvh * (2 / 3))
    const posterPath = $derived(getImagePath(media?.poster, posterSize(imagescaling, 'grid')))
    const hoverTime = 1000
    let hoverTimer: ReturnType<typeof setTimeout> | null = $state(null)

    afterNavigate(() => {
        if (hoverTimer !== null) {
            clearTimeout(hoverTimer)
            hoverTimer = null
        }
    })

    function handleMouseEnter(mediaItem: Api.MediaItem): void {
        hoverTimer = setTimeout(() => {
            $activeGridViewSideBarItem = mediaItem
        }, hoverTime)
    }

    function handleMouseLeave(): void {
        if (hoverTimer !== null) {
            clearTimeout(hoverTimer)
            hoverTimer = null
        }
    }
</script>

<svelte:window bind:outerHeight={x} />

<div
    role="link"
    tabindex="-1"
    class="group flex aspect-2/3 h-full"
    style={`min-height: ${minHpx}px; min-width: ${minWvw}vh; max-height: ${maxHvh}vh;`}>
    <a
        data-sveltekit-preload-data="off"
        tabindex="-1"
        href={resolve(`/details/?id=${media.id}&type=${media.type}`, {})}
        onclick={() => ($detailsMedia = media)}
        onmouseenter={() => handleMouseEnter(media)}
        onmouseleave={() => handleMouseLeave()}>
        {#if index === dataLength - 20}
            <PaginationDiv bind:mediaData />
        {/if}
        <div class="relative w-full">
            <img
                tabindex="-1"
                class:glow-shadow={media?.has_released_season_finale && page.url.pathname === '/watchlist'}
                class="aspect-2/3 w-full rounded object-cover shadow-lg"
                src={posterPath ? posterPath : '/images/poster-placeholder.png'}
                alt={media?.name ?? 'Media poster'} />
            <div
                class="absolute bottom-0 left-0 z-20 hidden w-full max-w-86 justify-between px-3 pb-3 group-hover:flex">
                <WatchedButton bind:media background="bg-gray-700/40" />
                <HideButton bind:media background="bg-gray-700/40" />
                <WatchlistButton bind:media background="bg-gray-700/40" />
                <FavouriteButton bind:media background="bg-gray-700/40" />
            </div>
        </div>
    </a>
</div>
