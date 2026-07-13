<script lang="ts">
    import PlayIcon from 'virtual:icons/material-symbols/play-circle'
    import MenuIcon from 'virtual:icons/material-symbols/menu'
    import SourceIcon from 'virtual:icons/ic/round-source'

    import { onMount } from 'svelte'
    import SeasonWatchedButton from '$lib/components/inputs/SeasonWatchedButton.svelte'
    import { getSelectedSeasonsEpisodes } from '$lib/functions/utility/getSelectedSeasonsEpisodes'
    import { getImagePath } from '$lib/functions/ui/getImagePath'
    import { loadingStates, scrollPositions } from '$lib/stores/app'
    import EpisodeGroupSelect from '$lib/components/inputs/EpisodeGroupSelect.svelte'
    import SeasonSelect from '$lib/components/inputs/SeasonSelect.svelte'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import EpisodeCard from '$lib/components/cards/EpisodeCard.svelte'
    import MenuModal from '$lib/components/modals/MenuModal.svelte'
    import { loadVideo } from '$lib/functions/video/loadVideo'

    let {
        media = $bindable(),
        selectedSeason = $bindable(),
        selectedEpisode = $bindable(),
        isSourcesOpen = $bindable(),
        showWatchedEpisodes = $bindable(),
    } = $props()

    const { episode_group_keys } = $derived(media.seasons ?? null)
    let currentEpisodeInfo = $derived(null)
    let episodeContainer: HTMLElement | null = $state(null)
    let selectedEpisodeGroup = $derived(media.episode_group_name)
    let isMenuOpen = $state(false)
    let selectedSeasonEpisodes = $derived.by(() => {
        let episodes = getSelectedSeasonsEpisodes(selectedSeason, media)

        if (!showWatchedEpisodes) episodes = episodes.filter((x) => !x.watched)

        return episodes
    })
    const posterPath = $derived.by(() => {
        if (media.type === 'movie') {
            return getImagePath(media.poster, 'original')
        } else {
            const poster = getImagePath(selectedSeason?.poster, 'original') ?? getImagePath(media.poster, 'original')
            return poster
        }
    })
    let currentSeasonNumber = $state(null)

    const handleSourcesClick = (): void => {
        $loadingStates.isSourcesLoading = true
        isSourcesOpen = true
    }

    $effect(() => {
        if (media.type === 'tv' && currentSeasonNumber !== selectedSeason?.season_num) {
            if (!episodeContainer) return

            currentSeasonNumber = selectedSeason?.season_num
            episodeContainer.scrollTop = 0
        }
    })

    onMount(() => {
        if (episodeContainer) episodeContainer.scrollTop = $scrollPositions.episodeContainer.y
    })
</script>

<aside class="glass-panel relative z-0 flex h-screen w-90 shrink-0 flex-col">
    <div class="relative {media.type === 'tv' ? 'h-[35%]' : 'h-full'} w-full">
        <div class="absolute inset-0">
            <img class="h-full w-full object-cover object-top" src={posterPath} alt="" />
            <div class="absolute inset-0 bg-linear-to-r from-black/80 via-transparent to-black/80"></div>
        </div>
        <div class="absolute inset-0 bg-linear-to-t from-backgroundColor via-backgroundColor/40 to-transparent"></div>
        {#if media.type === 'tv'}
            <div class="absolute bottom-3 left-2">
                <h2 class="font-outline text-xl leading-tight font-black tracking-widest text-textColor">
                    {#if selectedSeason}
                        {selectedSeason?.name ?? `Season ${selectedSeason?.season_num}`}
                    {/if}
                </h2>
                <div class="mt-2 flex items-center gap-4 text-sm font-medium text-slate-300"></div>
            </div>
        {/if}
    </div>
    {#if selectedSeason}
        {#if selectedSeason?.blurb}
            <div class="max-h-[26%] space-y-4 overflow-y-auto px-2">
                <h3 class="text-[10px] font-bold tracking-[0.2em] text-slate-500 uppercase">The Story</h3>
                <p class="text-sm leading-relaxed font-normal text-slate-300">
                    {selectedSeason?.blurb}
                </p>
            </div>
        {/if}
        <div class="mt-2">
            {#if episode_group_keys.length > 0}
                <EpisodeGroupSelect
                    {episode_group_keys}
                    {showWatchedEpisodes}
                    bind:media
                    bind:selectedEpisodeGroup
                    bind:selectedSeason
                    bind:selectedEpisode
                    bind:selectedSeasonEpisodes />
            {/if}
            {#if selectedSeason}
                <SeasonSelect
                    {selectedEpisodeGroup}
                    {showWatchedEpisodes}
                    {media}
                    bind:selectedSeason
                    bind:selectedEpisode />
            {/if}
        </div>
        <h3 class="my-4 px-2 text-[10px] font-bold tracking-[0.2em] text-slate-500 uppercase">
            Episodes <span class="float-right">
                <SeasonWatchedButton {showWatchedEpisodes} bind:media bind:selectedSeasonEpisodes bind:selectedSeason />
            </span>
        </h3>
        <div class="flex-1 space-y-3 overflow-y-auto" bind:this={episodeContainer}>
            {#if $loadingStates.isEpisodesLoading}
                <LoadingSpinner />
            {:else}
                {#each selectedSeasonEpisodes as episode (episode.episode_id)}
                    <EpisodeCard {episode} bind:selectedEpisode bind:currentEpisodeInfo bind:media />
                {/each}
            {/if}
        </div>
    {:else if media.seasons?.seasons?.length > 0}
        <button
            onclick={() => {
                showWatchedEpisodes = true
            }}
            class="h-1/2">Show Watched Episodes?</button>
        <div class="grow"></div>
    {:else}
        {#if media.type === 'tv'}<p>No Episodes Found.</p>{/if}
        <div class="grow"></div>
    {/if}
    {#if isMenuOpen}
        <MenuModal bind:isMenuOpen bind:showWatchedEpisodes {selectedSeason} {selectedEpisode} {media} />
    {/if}
    <div class="border-t border-white/5 bg-white/5 p-4">
        <div class="flex gap-3">
            <button
                disabled={$loadingStates.isPlayButtonLoading}
                onclick={() => {
                    loadVideo(media, null, selectedSeason?.season_num ?? null, selectedEpisode ?? null)
                }}
                class="group flex flex-1 items-center justify-center gap-2 rounded-xl bg-slate-800 py-3 font-bold text-textColor shadow-lg shadow-backgroundColor hover:text-primaryColor">
                <PlayIcon class="text-xl group-hover:text-primaryColor " />
                {#if $loadingStates.isPlayButtonLoading}<LoadingSpinner />{:else}Play{/if}
            </button>
            <button
                disabled={!selectedEpisode && media.type !== 'movie'}
                onclick={handleSourcesClick}
                class="{!selectedEpisode && media.type !== 'movie'
                    ? 'cursor-default text-neutral-500'
                    : 'group cursor-pointer text-textColor hover:text-primaryColor'}  flex flex-1 items-center justify-center gap-2 rounded-xl bg-slate-800 py-3 font-bold shadow-lg shadow-backgroundColor">
                <SourceIcon class="text-xl group-hover:text-primaryColor " />
                Sources
            </button>
            <button
                onclick={() => {
                    isMenuOpen = !isMenuOpen
                }}
                id="menu-button"
                class="group flex w-12 items-center justify-center gap-2 rounded-xl bg-slate-800 font-bold text-textColor shadow-lg shadow-backgroundColor hover:text-primaryColor">
                <MenuIcon class="text-xl group-hover:text-primaryColor " />
            </button>
        </div>
    </div>
</aside>
