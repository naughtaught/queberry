<script lang="ts">
    import { afterNavigate, beforeNavigate } from '$app/navigation'
    import { navigating } from '$app/state'
    import CarouselCard from '$lib/components/cards/CarouselCard.svelte'
    import HomeMenu from '$lib/components/modals/HomeMenu.svelte'
    import Carousel from '$lib/components/ui/Carousel.svelte'
    import { loadingStates, modals, scrollPositions } from '$lib/stores/app'
    import { cache, showHomeHidden } from '$lib/stores/pages'
    import MenuIcon from 'virtual:icons/material-symbols/menu'

    let sectionElement: HTMLElement | null = $state(null)

    const upNext = $derived(
        $cache.upNext.media.sort((a, b) => {
            if (a.has_unwatched_finale !== b.has_unwatched_finale) return a.has_unwatched_finale ? -1 : 1

            return (a.watched_episodes ?? 0) - (b.watched_episodes ?? 0)
        }),
    )
    const recentReleases = $derived(
        $showHomeHidden
            ? $cache.recentReleases.media
            : $cache.recentReleases.media.filter((x) => !x.watched && !x.hidden),
    )
    const trendingMovies = $derived(
        $showHomeHidden
            ? $cache.trendingMovies.media
            : $cache.trendingMovies.media.filter((x) => !x.watched && !x.hidden),
    )
    const trendingTv = $derived(
        $showHomeHidden ? $cache.trendingTv.media : $cache.trendingTv.media.filter((x) => !x.watched && !x.hidden),
    )

    afterNavigate((url) => {
        if (navigating.type === 'popstate' || url.from?.route.id === '/video') {
            if (sectionElement) sectionElement.scrollTop = $scrollPositions.home.y
        }
    })

    beforeNavigate(() => {
        if (sectionElement) $scrollPositions.home.y = sectionElement.scrollTop
    })

</script>

{#if recentReleases?.length > 0 || trendingMovies?.length > 0 || trendingTv?.length > 0}
    <div
        id="home-menu-button"
        class="fixed top-12 right-2 z-30 flex h-10 -translate-y-1/2 flex-col items-center rounded-md bg-backgroundColor px-2 py-2">
        <button aria-label="Toggle menu" onclick={() => ($modals.homeMenu = !$modals.homeMenu)}>
            <MenuIcon class="text-md text-textColor transition-colors hover:text-primaryColor " />
        </button>
    </div>
{/if}

{#if $modals.homeMenu}
    <HomeMenu />
{/if}

<section bind:this={sectionElement} class="flex h-screen w-full max-w-full flex-col space-y-10 overflow-y-auto p-10">
    {#if upNext?.length > 0 || $loadingStates.isUpNextLoading}
        <Carousel title="Next Episodes" loadingState="isUpNextLoading">
            {#each upNext as media (media.id)}
                <div class="min-w-50 shrink-0">
                    <CarouselCard {media} showEpisodes={true} />
                </div>
            {/each}
        </Carousel>
    {/if}
    {#if recentReleases?.length > 0 || $loadingStates.isRecentReleasesLoading}
        <Carousel title="Recently Released Movies" loadingState="isRecentReleasesLoading">
            {#each recentReleases as media (media.id)}
                <div class="min-w-50 shrink-0">
                    <CarouselCard {media} />
                </div>
            {/each}
        </Carousel>
    {/if}
    {#if trendingMovies?.length > 0 || $loadingStates.isTrendingLoading}
        <Carousel title="Trending Movies" loadingState="isTrendingLoading">
            {#each trendingMovies as media (media.id)}
                <div class="min-w-50 shrink-0">
                    <CarouselCard {media} />
                </div>
            {/each}
        </Carousel>
    {/if}
    {#if trendingTv?.length > 0 || $loadingStates.isTrendingLoading}
        <Carousel title="Trending Television" loadingState="isTrendingLoading">
            {#each trendingTv as media (media.id)}
                <div class="min-w-50 shrink-0">
                    <CarouselCard {media} />
                </div>
            {/each}
        </Carousel>
    {/if}
</section>
