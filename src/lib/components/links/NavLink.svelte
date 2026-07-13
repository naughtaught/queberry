<script lang="ts">
    import { resolve } from '$app/paths'
    import { page } from '$app/state'
    import ShuffleIcon from 'virtual:icons/zondicons/shuffle'
    import FiltersIcon from 'virtual:icons/carbon/global-filters'
    import TvIcon from 'virtual:icons/mynaui/tv'
    import MovieIcon from 'virtual:icons/material-symbols/movie-outline'
    import { loadingStates, modals, parentalControlsAreEnabled } from '$lib/stores/app'
    import { restrictedContent } from '$lib/stores/user'
    import { activeGridViewSideBarItem, cache, currentFilters, defaultFilters } from '$lib/stores/pages'
    import { isValidRoute } from '$lib/functions/utility/isValidRoute'
    import { fetchData } from '$lib/db/fetchData'
    import { handleError } from '$lib/functions/errors/errorHandling'

    const { children, text, href } = $props()

    const hasMenu = ['/discover', '/watchlist', '/favourites']

    const isMenuOpen = $derived(page.url.pathname === href && hasMenu.includes(href))

    const displayShuffle = $derived.by(() => {
        if (page.url.pathname === '/favourites') {
            if ($currentFilters.type === 'movies') return $cache.favourites.media.length > 1
            if ($currentFilters.type === 'tv') return $cache.favourites.media.length >= 1
        }

        if (page.url.pathname === '/watchlist') {
            if ($currentFilters.type === 'movies') return $cache.watchlist.media.length > 1
            if ($currentFilters.type === 'tv') return $cache.watchlist.media.length >= 1
        }
    })

    const switchType = async (type: 'movies' | 'tv'): Promise<void> => {
        $loadingStates.isGridViewLoading = true

        let newContentRatings: string[] = []

        if ($parentalControlsAreEnabled) {
            if (type === 'movies') {
                newContentRatings = $restrictedContent.movies
            } else {
                newContentRatings = $restrictedContent.tv
            }
        }

        $currentFilters = {
            ...$defaultFilters,
            type,
            contentRatings: newContentRatings,
        }

        try {
            const sanitizedHref = href?.replace('/', '')

            if (!isValidRoute(sanitizedHref)) return

            if (sanitizedHref === 'watchlist') {
                $currentFilters = {
                    ...$currentFilters,
                    type,
                    includeFavourites: true,
                    includeHidden: true,
                    includeWatched: true,
                    includeWatchlisted: true,
                    onlyWatchlisted: true,
                }
            }
            if (sanitizedHref === 'favourites') {
                $currentFilters = {
                    ...$currentFilters,
                    type,
                    includeFavourites: true,
                    includeHidden: true,
                    includeWatched: true,
                    onlyFavourites: true,
                }
            }

            $cache[sanitizedHref].paginationPage = 0

            const resp = await fetchData()
            if (!resp.success) throw resp.error

            $cache[sanitizedHref].lastUpdated = Date.now()
            $cache[sanitizedHref].media = [...resp.data]
            $cache[sanitizedHref].filters = $currentFilters
            $activeGridViewSideBarItem = $cache[sanitizedHref].media[0]
            const section = document.getElementById('gridViewSection')
            if (section) section.scrollTop = 0
        } catch (err) {
            handleError(err, { context: 'failed to fetch postgres data' })
        }
        $loadingStates.isGridViewLoading = false
    }
</script>

<div role="menu" tabindex="-1" class="relative">
    <a
        data-sveltekit-preload-data="tap"
        class="group flex items-center gap-2 rounded-xl px-4 py-3 text-textColor transition-all"
        href={resolve(href, {})}>
        {@render children?.()}

        <span
            class="relative flex cursor-pointer items-center pr-1 text-sm font-semibold tracking-wide after:absolute after:-bottom-[1.5px] after:left-0 after:h-0.5 after:bg-primaryColor after:transition-all after:duration-100 group-hover:after:w-full"
            class:after:w-full={page.url.pathname === href}>
            {text}
        </span>
    </a>

    {#if isMenuOpen && !$loadingStates.isGridViewLoading}
        <div
            class="absolute top-1/2 left-full z-30 flex h-32 -translate-y-1/2 flex-col items-center justify-between rounded-md bg-backgroundColor px-2 py-2">
            <button aria-label="Show Movies">
                <MovieIcon
                    onclick={() => switchType('movies')}
                    class="text-lg {$currentFilters.type === 'movies' && page.url.pathname === href
                        ? 'text-primaryColor'
                        : 'text-textColor'} transition-colors hover:text-primaryColor" />
            </button>
            <button aria-label="Toggle filter menu" onclick={() => ($modals.filters = true)}>
                <FiltersIcon class="text-lg text-textColor transition-colors hover:text-primaryColor" />
            </button>
            {#if displayShuffle}
                <button aria-label="Toggle shuffle menu" onclick={() => ($modals.shuffle = true)}>
                    <ShuffleIcon class="text-md text-textColor transition-colors hover:text-primaryColor" />
                </button>
            {/if}
            <button aria-label="Show TV">
                <TvIcon
                    onclick={() => switchType('tv')}
                    class="text-lg {$currentFilters.type === 'tv' && page.url.pathname === href
                        ? 'text-primaryColor'
                        : 'text-textColor'} transition-colors hover:text-primaryColor" />
            </button>
        </div>
    {/if}
</div>
