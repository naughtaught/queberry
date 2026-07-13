<script lang="ts">
    import { getImagePath } from '$lib/functions/ui/getImagePath'
    import { posterSize } from '$lib/functions/ui/posterSize'
    import StarIcon from 'virtual:icons/material-symbols/star'
    import { resolve } from '$app/paths'
    import { detailsMedia } from '$lib/stores/pages'
    import { modals } from '$lib/stores/app'
    import { parseCredits } from '$lib/functions/utility/parseCredits'
    import { SvelteSet } from 'svelte/reactivity'
    import WatchedButton from '$lib/components/inputs/WatchedButton.svelte'
    import HideButton from '$lib/components/inputs/HideButton.svelte'
    import WatchlistButton from '$lib/components/inputs/WatchlistButton.svelte'
    import FavouriteButton from '$lib/components/inputs/FavouriteButton.svelte'
    import { abbreviateName } from '$lib/functions/utility/abbreviateName'

    let { result = $bindable() } = $props()

    const posterPath = $derived(getImagePath(result?.poster, posterSize(1, 'carousel')))
    const { cast, directors, creators } = $derived(parseCredits(result, 3))
    const avgRating = $derived(result?.ratings?.find((r: { source: string }) => r.source === 'average')?.rating ?? null)
    const uniqueCrew = $derived.by(() => {
        const seen = new SvelteSet()
        return directors.concat(creators, cast).filter((c) => {
            if (seen.has(c.name)) return false
            seen.add(c.name)
            return true
        })
    })
</script>

<a
    data-sveltekit-preload-data="off"
    onclick={() => {
        $detailsMedia = result
        $modals.search = false
    }}
    href={resolve(`/details/?id=${result.id}&type=${result.type}`, {})}
    class="group flex cursor-pointer flex-col items-center rounded-lg p-4 transition-all hover:bg-white/5">
    <div class="flex w-full gap-6">
        <div class="h-44 w-32 shrink-0 overflow-hidden rounded-md shadow-lg">
            <img
                alt="Search Result Poster"
                class="h-full w-full object-cover"
                src={posterPath ? posterPath : '/images/poster-placeholder.png'} />
        </div>
        <div class="flex-1">
            <h3 class="mb-1 text-xl font-bold">{result.title}</h3>
            <div class="flex items-center gap-3">
                <span class="text-sm tracking-widest text-slate-400 uppercase">{result.released}</span>
                <span class="h-1 w-1 rounded-full bg-slate-600"></span>
                <span class="truncate text-sm tracking-widest text-slate-400 uppercase">
                    {result.genres.slice(0, 5).join(', ')}{result.genres.length > 5 ? '...' : ''}
                </span>
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
        <div class="flex flex-col gap-y-4">
            <div
                class="flex h-5 items-center gap-1.5 rounded-full border border-primaryColor/20 bg-primaryColor/10 px-3 py-1">
                <span class="font-outline text-sm text-amber-400"><StarIcon class="text-md" /></span>
                <span class="text-sm font-bold text-textColor">{avgRating ? avgRating / 10 : '-'}</span>
            </div>
            <div class="flex w-full flex-col items-end space-y-4">
                <WatchedButton bind:media={result} />
                <HideButton bind:media={result} />
                <WatchlistButton bind:media={result} />
                <FavouriteButton bind:media={result} />
            </div>
        </div>
    </div>
</a>
