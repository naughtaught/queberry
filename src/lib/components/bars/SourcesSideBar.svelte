<script lang="ts">
    import PlayIcon from 'virtual:icons/material-symbols/play-circle'
    import MenuIcon from 'virtual:icons/material-symbols/menu'
    import SeasonIcon from 'virtual:icons/material-symbols/tv-with-assistant-outline'

    import { onMount } from 'svelte'
    import SourceCard from '$lib/components/cards/SourceCard.svelte'
    import { getImagePath } from '$lib/functions/ui/getImagePath'
    import type { Plugins } from '$lib/types/plugins'
    import { indexerSources } from '$lib/stores/plugins'
    import { fetchSources } from '$lib/functions/plugins/fetchSources'
    import { loadingStates } from '$lib/stores/app'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import MenuModal from '$lib/components/modals/MenuModal.svelte'
    import { loadVideo } from '$lib/functions/video/loadVideo'

    let { media, selectedSeason, selectedEpisode, isSourcesOpen = $bindable(), selectedSource = $bindable() } = $props()

    let isMenuOpen = $state(false)

    const posterPath = $derived.by(() => {
        if (media.type === 'movie') {
            return getImagePath(media.poster, 'original')
        } else {
            return getImagePath(selectedSeason?.poster, 'original')
        }
    })

    const getSourceKey = (): string =>
        media.type === 'tv' ? (selectedEpisode?.episode_id ?? media.imdb_id) : media.imdb_id

    let sources: Plugins.IndexerSource[] = $derived.by(() => {
        const key = getSourceKey()
        const existingSource = $indexerSources.find((source) => source[key])
        return (existingSource?.[key] ?? []) as Plugins.IndexerSource[]
    })

    onMount(async () => {
        await fetchSources(
            media.imdb_id,
            media.title,
            media.released,
            media.type,
            selectedSeason?.season_num,
            selectedEpisode?.episode_num,
            selectedEpisode?.episode_id,
            false,
        )
    })
</script>

<aside class="glass-panel z-0 flex h-screen w-90 shrink-0 flex-col">
    <div class="relative min-h-[35%] w-full">
        <img class="h-full w-full object-cover object-top" src={posterPath} alt="" />
        <div class="absolute inset-0 bg-linear-to-t from-backgroundColor via-backgroundColor/40 to-transparent"></div>
        {#if media.type === 'tv'}
            <div class="absolute bottom-3 left-2">
                <h2 class="font-outline text-xl leading-tight font-black tracking-widest text-textColor">
                    Season {selectedSeason.season_num} Episode {selectedEpisode.episode_num}
                </h2>
                <div class="mt-2 flex items-center gap-4 text-sm font-medium text-slate-300"></div>
            </div>
        {/if}
    </div>
    {#if $loadingStates.isSourcesLoading}
        <div class="flex flex-1 items-center justify-center">
            <LoadingSpinner />
        </div>
    {:else}
        <div class="my-2 ml-2 space-y-4 overflow-x-hidden overflow-y-auto">
            {#each sources ?? [] as source (source)}
                <SourceCard {source} {media} {selectedSeason} {selectedEpisode} bind:selectedSource />
            {/each}
        </div>
    {/if}
    <div class="grow"></div>
    {#if isMenuOpen}
        <MenuModal bind:isMenuOpen bind:sources {selectedSeason} {selectedEpisode} {media} />
    {/if}
    <div class="border-t border-white/5 bg-white/5 p-4">
        <div class="flex gap-3">
            <button
                disabled={$loadingStates.isPlayButtonLoading}
                onclick={() => {
                    loadVideo(media, null, selectedSeason?.season_num ?? null, selectedEpisode ?? null)
                }}
                class="group flex flex-1 items-center justify-center gap-2 rounded-xl bg-slate-800 py-3 font-bold text-textColor shadow-lg shadow-backgroundColor hover:text-primaryColor">
                {#if $loadingStates.isPlayButtonLoading}<LoadingSpinner />{:else}
                    <PlayIcon class="text-xl  " /> Play{/if}
            </button>
            <button
                onclick={() => {
                    isSourcesOpen = false
                }}
                disabled={$loadingStates.isPlayButtonLoading}
                class=" group flex flex-1 items-center justify-center gap-2 rounded-xl bg-slate-800 py-3 font-bold text-textColor shadow-lg shadow-backgroundColor hover:text-primaryColor disabled:cursor-default disabled:hover:text-textColor">
                <SeasonIcon class="text-xl " />
                {#if media?.type === 'tv'}Season{:else}Details{/if}
            </button>
            <button
                onclick={() => {
                    isMenuOpen = !isMenuOpen
                }}
                disabled={$loadingStates.isPlayButtonLoading}
                id="menu-button"
                class="group flex w-12 items-center justify-center gap-2 rounded-xl bg-slate-800 font-bold text-textColor shadow-lg shadow-backgroundColor hover:text-primaryColor disabled:cursor-default disabled:hover:text-textColor">
                <MenuIcon class="text-xl" />
            </button>
        </div>
    </div>
</aside>
