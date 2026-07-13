<script lang="ts">
    import { page } from '$app/state'
    import { countries } from '$lib/data/countries'
    import { languages } from '$lib/data/languages'
    import { fetchData } from '$lib/db/fetchData'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import {
        GENDER_LIST,
        GENRE_LIST,
        loadingStates,
        modals,
        MOVIE_RATINGS,
        parentalControlsAreEnabled,
        PRIORITY_COUNTRIES,
        PRIORITY_LANGAUGES,
        STATUS_TYPES,
        TV_RATINGS,
    } from '$lib/stores/app'
    import { activeGridViewSideBarItem, cache, currentFilters, defaultFilters } from '$lib/stores/pages'
    import { restrictedContent } from '$lib/stores/user'
    import type { App } from '$lib/types/app'
    import FiltersIcon from 'virtual:icons/carbon/global-filters'
    import ResetIcon from 'virtual:icons/ri/reset-left-fill'
    import BaseModal from '$lib/components/modals/BaseModal.svelte'
    import ToggleSwitch from '$lib/components/inputs/ToggleSwitch.svelte'
    import DoubleHandledSlider from '$lib/components/inputs/DoubleHandledSlider.svelte'
    import MinMaxInputs from '$lib/components/inputs/MinMaxInputs.svelte'
    import GroupCheckboxDropdownWithExclude from '$lib/components/inputs/GroupCheckboxDropdownWithExclude.svelte'
    import GroupCheckboxDropdown from '$lib/components/inputs/GroupCheckboxDropdown.svelte'
    import Select from '$lib/components/inputs/Select.svelte'
    import Checkbox from '$lib/components/inputs/Checkbox.svelte'

    let newFilters = $state($currentFilters)
    let reset = $state(false)
    const sortOptions = $state([
        { name: 'Recently Added', value: 'id' },
        { name: 'Average Rating', value: 'avg_rating' },
        { name: 'Popularity', value: 'total_count' },
        { name: 'Year', value: 'released' },
    ])

    const contentRatings = $derived.by(() => {
        let ratings = newFilters.type === 'movies' ? MOVIE_RATINGS : TV_RATINGS

        if ($parentalControlsAreEnabled) {
            const restrictedRatings = newFilters.type === 'movies' ? $restrictedContent.movies : $restrictedContent.tv

            ratings = ratings.filter((rating) => !restrictedRatings.includes(rating))
        }

        return ratings
    })

    const handleClose = (): void => {
        $modals.filters = false
    }

    const countryOptions = Array.from(
        new Set(
            countries.map((x) => x.name).filter((name): name is string => typeof name === 'string' && name !== null),
        ),
    ).sort((a, b) => {
        const aPriority = PRIORITY_COUNTRIES.indexOf(a)
        const bPriority = PRIORITY_COUNTRIES.indexOf(b)
        if (aPriority !== -1 && bPriority !== -1) {
            return aPriority - bPriority
        }
        if (aPriority !== -1) return -1
        if (bPriority !== -1) return 1
        return a.localeCompare(b)
    })

    const languageOptions = Array.from(
        new Set(
            languages.map((x) => x.name).filter((name): name is string => typeof name === 'string' && name !== null),
        ),
    ).sort((a, b) => {
        const aPriority = PRIORITY_LANGAUGES.indexOf(a)
        const bPriority = PRIORITY_LANGAUGES.indexOf(b)
        if (aPriority !== -1 && bPriority !== -1) {
            return aPriority - bPriority
        }
        if (aPriority !== -1) return -1
        if (bPriority !== -1) return 1
        return a.localeCompare(b)
    })

    const resetForm = (): void => {
        reset = true
        newFilters = $defaultFilters
    }

    const handleClick = async (): Promise<void> => {
        try {
            $loadingStates.isGridViewLoading = true
            $modals.filters = false

            $currentFilters = newFilters

            const response = await fetchData()
            if (!response.success) throw response.error

            const route = page.url.pathname.replace('/', '') as keyof App.Cache
            $cache[route].media = [...response.data]
            $cache[route].paginationPage = 0
            $cache[route].lastUpdated = Date.now()

            $activeGridViewSideBarItem = response.data[0]
        } catch (error) {
            handleError(error)
        } finally {
            $loadingStates.isGridViewLoading = false
        }
    }
</script>

<BaseModal onClose={handleClose}>
    <header class="flex items-center justify-between border-b border-slate-700/50 px-8 py-5">
        <div class="flex items-center gap-3">
            <span class="text-3xl"><FiltersIcon class="mr-4 text-2xl" /></span>
            <h2 class="text-2xl font-bold tracking-tight">Advanced Filters</h2>
        </div>
        <div class="flex items-center gap-6">
            <button
                onclick={resetForm}
                class="group flex items-center text-xs font-medium tracking-widest text-slate-400 uppercase hover:text-primaryColor"
                >Reset<span>
                    <ResetIcon class="group ml-1 text-slate-500 group-hover:text-primaryColor" />
                </span>
            </button>
        </div>
    </header>
    <div class="flex w-[70vw] flex-col space-y-5 px-8 py-5">
        <form class="flex max-h-[80vh] w-full space-x-10 pb-5">
            <div class="flex w-1/2 flex-col space-y-7">
                <div class="flex items-center justify-between">
                    <p class="font-semibold tracking-wide text-slate-100 uppercase opacity-60">Content Type</p>
                    <ToggleSwitch
                        options={['movies', 'tv']}
                        bind:value={newFilters.type}
                        func={() => {
                            if ($parentalControlsAreEnabled) {
                                if (newFilters.type === 'tv') {
                                    newFilters.contentRatings = $restrictedContent.tv
                                } else {
                                    newFilters.contentRatings = $restrictedContent.movies
                                }
                            }
                        }} />
                </div>
                <div class="flex flex-col space-y-7">
                    <DoubleHandledSlider
                        min={1903}
                        max={new Date().getFullYear()}
                        bind:minValue={newFilters.minYear}
                        bind:maxValue={newFilters.maxYear}
                        step={1}
                        {reset}
                        label="Release Year" />
                    <div class="flex space-x-10">
                        <DoubleHandledSlider
                            min={0}
                            max={100}
                            bind:minValue={newFilters.minAvgRating}
                            bind:maxValue={newFilters.maxAvgRating}
                            step={1}
                            {reset}
                            label="Average" />
                        <DoubleHandledSlider
                            min={0}
                            max={100}
                            bind:minValue={newFilters.minMetacritic}
                            bind:maxValue={newFilters.maxMetacritic}
                            step={1}
                            {reset}
                            label="Metacritic" />
                    </div>
                    <div class="flex space-x-10">
                        <DoubleHandledSlider
                            min={0}
                            max={100}
                            bind:minValue={newFilters.minTomatometer}
                            bind:maxValue={newFilters.maxTomatometer}
                            step={1}
                            {reset}
                            label="Tomatometer" />
                        <DoubleHandledSlider
                            min={0}
                            max={100}
                            bind:minValue={newFilters.minPopcornmeter}
                            bind:maxValue={newFilters.maxPopcornmeter}
                            step={1}
                            {reset}
                            label="Popcornmeter" />
                    </div>
                    <div class="flex space-x-10">
                        <DoubleHandledSlider
                            min={0}
                            max={100}
                            bind:minValue={newFilters.minImdbRating}
                            bind:maxValue={newFilters.maxImdbRating}
                            step={1}
                            {reset}
                            label="IMDb" />
                        <DoubleHandledSlider
                            min={0}
                            max={100}
                            bind:minValue={newFilters.minLetterboxdRating}
                            bind:maxValue={newFilters.maxLetterboxdRating}
                            step={1}
                            {reset}
                            label="Letterboxd" />
                    </div>
                </div>
                <div class="mt-3">
                    <MinMaxInputs label="Review Count" bind:min={newFilters.minCount} bind:max={newFilters.maxCount} />
                </div>
            </div>
            <div class="flex w-1/2 flex-col space-y-10">
                <div class="flex flex-col space-y-5">
                    <div class="flex space-x-4">
                        <GroupCheckboxDropdownWithExclude
                            dataArr={GENRE_LIST}
                            bind:includeData={newFilters.genres}
                            bind:excludeData={newFilters.excludeGenres}
                            name="Genres" />
                        <GroupCheckboxDropdownWithExclude
                            dataArr={languageOptions}
                            bind:includeData={newFilters.languages}
                            bind:excludeData={newFilters.excludeLanguages}
                            name="Languages" />
                    </div>
                    <div class="flex space-x-4">
                        <GroupCheckboxDropdownWithExclude
                            dataArr={countryOptions}
                            bind:includeData={newFilters.countries}
                            bind:excludeData={newFilters.excludeCountries}
                            name="Countries" />
                        <GroupCheckboxDropdown
                            dataArr={STATUS_TYPES}
                            searchEnabled={false}
                            bind:includeData={newFilters.status}
                            name="Status" />
                    </div>
                    <div class="flex space-x-4">
                        <GroupCheckboxDropdown
                            dataArr={GENDER_LIST}
                            bind:includeData={newFilters.genders}
                            searchEnabled={false}
                            name="Genders" />
                        <GroupCheckboxDropdown
                            dataArr={contentRatings}
                            bind:includeData={newFilters.contentRatings}
                            searchEnabled={false}
                            name="Exclude Content Ratings" />
                    </div>
                    <div class="flex space-x-4">
                        <Select
                            options={sortOptions}
                            bind:activeOption={newFilters.sort}
                            bind:ascending={newFilters.ascending}
                            name="Sort" />
                    </div>
                </div>

                <div class="flex flex-col space-y-5">
                    <h3 class="text-xs font-bold tracking-widest text-slate-500 uppercase">Library Options</h3>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="flex flex-col space-y-4">
                            <Checkbox
                                label="Include Watched"
                                bind:checked={newFilters.includeWatched}
                                func={() => {
                                    newFilters.onlyWatched = false
                                    newFilters.onlyHidden = false
                                }} />
                            <Checkbox
                                label="Include Watchlisted"
                                bind:checked={newFilters.includeWatchlisted}
                                disabled={page.url.pathname === '/watchlist' ? true : false}
                                func={() => {
                                    newFilters.onlyWatched = false
                                    newFilters.onlyHidden = false
                                }} />
                            <Checkbox
                                label="Only Hidden"
                                disabled={page.url.pathname !== '/discover' ? true : false}
                                bind:checked={newFilters.onlyHidden}
                                func={() => {
                                    newFilters.onlyWatched = false
                                    newFilters.includeWatched = false
                                    newFilters.includeHidden = false
                                    newFilters.includeWatchlisted = false
                                    newFilters.includeFavourites = false
                                }} />
                        </div>
                        <div class="flex flex-col space-y-4">
                            <Checkbox
                                label="Include Hidden"
                                disabled={page.url.pathname !== '/discover' ? true : false}
                                bind:checked={newFilters.includeHidden}
                                func={() => {
                                    newFilters.onlyWatched = false
                                    newFilters.onlyHidden = false
                                }} />
                            <Checkbox
                                label="Include Favourites"
                                disabled={page.url.pathname === '/favourites' ? true : false}
                                bind:checked={newFilters.includeFavourites}
                                func={() => {
                                    newFilters.onlyWatched = false
                                    newFilters.onlyHidden = false
                                }} />
                            <Checkbox
                                label="Only Watched"
                                disabled={page.url.pathname !== '/discover' ? true : false}
                                bind:checked={newFilters.onlyWatched}
                                func={() => {
                                    newFilters.onlyHidden = false
                                    newFilters.includeWatched = false
                                    newFilters.includeHidden = false
                                    newFilters.includeWatchlisted = false
                                    newFilters.includeFavourites = false
                                }} />
                        </div>
                    </div>
                </div>
            </div>
        </form>
    </div>
    <footer class="flex items-center justify-between border-t border-slate-700/50 bg-slate-900/40 px-8 py-5">
        <div class=""></div>
        <div class="flex gap-4">
            <button
                onclick={() => ($modals.filters = false)}
                class="rounded-lg px-8 py-3 font-bold text-slate-300 transition-all hover:bg-slate-800">Cancel</button>
            <button
                onclick={handleClick}
                class="rounded-lg bg-slate-800 px-10 py-3 font-bold text-white hover:bg-primaryColor"
                >Apply Filters</button>
        </div>
    </footer>
</BaseModal>
