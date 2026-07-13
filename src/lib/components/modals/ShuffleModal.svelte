<script lang="ts">
    import BaseModal from '$lib/components/modals/BaseModal.svelte'
    import { loadingStates, modals } from '$lib/stores/app'
    import ShuffleIcon from 'virtual:icons/zondicons/shuffle'
    import Checkbox from '$lib/components/inputs/Checkbox.svelte'
    import { currentFilters, defaultShuffleSettings, shuffleSettings } from '$lib/stores/pages'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import { onMount } from 'svelte'
    import { loadVideo } from '$lib/functions/video/loadVideo'
    import type { Api } from '$lib/types/api'
    import { fetchRandom } from '$lib/db/fetchRandom'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { SvelteSet } from 'svelte/reactivity'
    import { parseCredits } from '$lib/functions/utility/parseCredits'
    import { getImagePath } from '$lib/functions/ui/getImagePath'
    import { posterSize } from '$lib/functions/ui/posterSize'
    import StarIcon from 'virtual:icons/material-symbols/star'
    import ResetIcon from 'virtual:icons/ri/reset-left-fill'
    import { page } from '$app/state'
    import TrailerModal from '$lib/components/modals/TrailerModal.svelte'
    import TrailerIcon from 'virtual:icons/material-symbols/smart-display'
    import { abbreviateName } from '$lib/functions/utility/abbreviateName'

    let media: Api.MediaItem | null = $derived(null as Api.MediaItem | null)
    let isLoading = $state(true)
    let canShuffle = $state(true)
    let isTrailerOpen = $state(false)
    const avgRating = $derived(media?.ratings?.find((r: { source: string }) => r.source === 'average')?.rating ?? null)
    const posterPath = $derived.by(() => {
        const currentMedia = media
        return getImagePath(currentMedia?.poster, posterSize(1, 'grid'))
    })
    const { cast, directors, creators } = $derived.by(() => {
        if (media) return parseCredits(media, 3)
        return {
            cast: [] as Api.CastMember[],
            directors: [] as Api.CastMember[],
            creators: [] as Api.CastMember[],
        }
    })
    const uniqueCrew = $derived.by(() => {
        const seen = new SvelteSet()
        return directors.concat(creators, cast).filter((c) => {
            if (seen.has(c.name)) return false
            seen.add(c.name)
            return true
        })
    })

    const playMedia = (): void => {
        if (!media) return
        try {
            loadVideo(media, null, null, null)
        } catch (error) {
            handleError(error)
        } finally {
            $modals.shuffle = false
        }
    }

    const getMedia = async (): Promise<void> => {
        if (media) $shuffleSettings.excludeMediaIds = [...$shuffleSettings.excludeMediaIds, media.id]
        isLoading = true
        try {
            const resp = await fetchRandom()
            if (!resp.success) throw resp.error
            if (!resp.data) {
                media = null
                canShuffle = false
            }
            if (resp.data) media = resp.data
        } catch (error) {
            handleError(error)
        } finally {
            isLoading = false
        }
    }

    onMount(() => {
        media = null
        $shuffleSettings = { ...defaultShuffleSettings }
        canShuffle = true
        getMedia()
    })

    const reset = (): void => {
        media = null
        $shuffleSettings = { ...defaultShuffleSettings }
        canShuffle = true
    }
</script>

<BaseModal
    onClose={() => {
        $modals.shuffle = false
    }}>
    {#if media && isTrailerOpen}
        <div>
            <TrailerModal
                {media}
                func={() => {
                    isTrailerOpen = false
                }} />
        </div>
    {/if}

    <header class="flex items-center justify-between border-b border-slate-700/50 px-8 py-5">
        <div class="flex w-full items-center justify-between gap-3">
            <h2 class="text-2xl font-bold tracking-tight">Shuffle Play</h2>
            <button
                onclick={() => {
                    reset()
                    getMedia()
                }}
                class="group flex items-center text-xs font-medium tracking-widest text-slate-400 uppercase hover:text-primaryColor"
                >Reset<span>
                    <ResetIcon class="group ml-1 text-slate-500 group-hover:text-primaryColor" />
                </span></button>
        </div>
    </header>
    <div class="flex min-h-75 w-[70vw] flex-col space-y-5 px-8 py-5">
        {#if media && !isLoading}
            <div class="flex min-h-75 w-full gap-6">
                <div class="h-60 w-44 shrink-0 overflow-hidden rounded-md shadow-lg">
                    <img
                        alt="Search Result Poster"
                        class="h-full w-full object-cover"
                        src={posterPath ? posterPath : '/images/poster-placeholder.png'} />
                </div>
                <div class="flex-1">
                    <h3 class="mb-1 text-xl font-bold">{media.title}</h3>
                    <div class="flex items-center gap-3">
                        <span class="text-sm tracking-widest text-slate-400 uppercase">{media.released}</span>
                        <span class="h-1 w-1 rounded-full bg-slate-600"></span>
                        <span class="text-sm tracking-widest text-slate-400 uppercase">{media.genres}</span>
                    </div>
                    <div class="mt-3">
                        <p class="text-sm text-textColor">{media.blurb}</p>
                    </div>
                    <div class="mt-4 pt-4">
                        <div class="flex space-x-4 overflow-x-auto pb-2">
                            {#each uniqueCrew as crew, i (i)}
                                <div class="flex shrink-0 flex-col items-center">
                                    <div class="size-18 overflow-hidden rounded-lg border border-white/5 bg-white/5">
                                        <img
                                            alt="Headshot of {crew.name}"
                                            class="h-full w-full object-cover"
                                            src={crew.image
                                                ? `https://image.tmdb.org/t/p/w185/${crew.image}`
                                                : '/images/person-placeholder.png'} />
                                    </div>
                                    <div class="mt-1">
                                        <p class="max-w-18 truncate text-xs font-bold text-textColor">
                                            {abbreviateName(crew.name)}
                                        </p>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    </div>
                </div>
                <div class="flex flex-col justify-between gap-y-4">
                    <div
                        class="flex h-5 items-center gap-1.5 rounded-full border border-primaryColor/20 bg-primaryColor/10 px-3 py-1">
                        <span class="font-outline text-sm text-amber-400"><StarIcon class="text-md" /></span>
                        <span class="text-sm font-bold text-textColor">{avgRating ? avgRating / 10 : '-'}</span>
                    </div>
                    <button
                        onclick={() => {
                            isTrailerOpen = true
                        }}
                        disabled={!media?.trailer}
                        class="{media?.trailer
                            ? 'text-textColor hover:text-primaryColor'
                            : 'cursor-default! text-neutral-700'} group flex items-center justify-center gap-2 rounded-lg bg-slate-800 px-2 py-2 text-xs font-bold shadow-lg shadow-backgroundColor">
                        <TrailerIcon
                            class="text-xs {media?.trailer
                                ? 'text-textColor group-hover:text-primaryColor'
                                : 'cursor-default! text-neutral-700'} " />
                        Trailer
                    </button>
                </div>
            </div>
        {:else if isLoading}
            <div class="flex min-h-75 items-center justify-center">
                <LoadingSpinner />
            </div>
        {:else}
            <div class="min-h-75">
                <p>No matching media found.</p>
            </div>
        {/if}
    </div>
    <footer class="flex items-center justify-between border-t border-slate-700/50 bg-slate-900/40 px-8 py-5">
        <div class=""></div>
        <div class="flex w-full justify-between">
            <div class="flex space-x-5">
                <button
                    disabled={!canShuffle}
                    class={!canShuffle ? ' cursor-default! text-slate-500' : 'text-textColor '}
                    onclick={getMedia}
                    ><ShuffleIcon class="{!canShuffle ? ' ' : ' hover:text-primaryColor'} mr-4 text-2xl " />
                </button>
                {#if $currentFilters.type === 'movies'}
                    <Checkbox label="Continuous Play?" bind:checked={$shuffleSettings.continuousPlay} />
                {:else if page.url.pathname === '/favourites'}
                    <Checkbox label="Random Episodes?" bind:checked={$shuffleSettings.randomEpisodes} />
                {/if}
            </div>
            <div>
                <button
                    onclick={() => ($modals.shuffle = false)}
                    class="rounded-lg px-8 py-3 font-bold text-slate-300 transition-all hover:bg-slate-800"
                    >Cancel</button>
                <button
                    disabled={$loadingStates.isPlayButtonLoading}
                    onclick={playMedia}
                    class="{$loadingStates.isPlayButtonLoading
                        ? ' cursor-default! text-slate-500'
                        : 'text-textColor hover:text-primaryColor'} flex-1 rounded-lg bg-slate-800 px-6 py-3.5 font-bold shadow-lg transition-all">
                    {#if $loadingStates.isPlayButtonLoading}<LoadingSpinner />{:else}Play{/if}
                </button>
            </div>
        </div>
    </footer>
</BaseModal>
