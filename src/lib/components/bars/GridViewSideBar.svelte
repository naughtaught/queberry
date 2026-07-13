<script lang="ts">
    import TrailerIcon from 'virtual:icons/material-symbols/smart-display'
    import StarIcon from 'virtual:icons/material-symbols/star'
    import PlayIcon from 'virtual:icons/material-symbols/play-circle'
    import { activeGridViewSideBarItem } from '$lib/stores/pages'
    import { getImagePath } from '$lib/functions/ui/getImagePath'
    import { formatRuntime } from '$lib/functions/ui/formatRuntime'
    import { formatAverageRating } from '$lib/functions/ui/formatAverageRating'
    import { orderRatings } from '$lib/functions/ui/orderRatings'
    import { parseCredits } from '$lib/functions/utility/parseCredits'
    import { loadingStates, modals } from '$lib/stores/app'
    import TrailerModal from '$lib/components/modals/TrailerModal.svelte'
    import type { Api } from '$lib/types/api'
    import CastCard from '$lib/components/cards/CastCard.svelte'
    import RatingCard from '$lib/components/cards/RatingCard.svelte'
    import { loadVideo } from '$lib/functions/video/loadVideo'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'

    const media = $derived($activeGridViewSideBarItem!)

    const backdropPath = $derived(getImagePath(media.backdrop, 'original'))

    const formattedRuntime = $derived(formatRuntime(media))
    const averageRating = $derived(formatAverageRating(media))
    const mediaRatings = $derived(orderRatings(media?.ratings))
    const { cast, directors, creators } = $derived(parseCredits(media, 3))
</script>

{#if $modals.trailer}
    <TrailerModal {media} />
{/if}

{#snippet crewList(crewItems: Api.CastMember[], type: string)}
    <div class="space-y-4">
        <h3 class="text-[10px] font-bold tracking-[0.2em] text-slate-500 uppercase">{type}</h3>
        {#each crewItems as crew, i (i)}
            <CastCard credit={crew} />
        {/each}
    </div>
{/snippet}

<aside class="glass-panel z-0 flex h-screen w-105 shrink-0 flex-col">
    <div class="relative h-[45%] w-full">
        <div
            class="absolute inset-0 bg-cover bg-center"
            data-alt="Backdrop Image of {media.title}"
            style="background-image: url({backdropPath})">
        </div>
        <div class="absolute inset-0 bg-linear-to-t from-backgroundColor via-backgroundColor/40 to-transparent"></div>
        <div class="absolute right-8 bottom-6 left-8">
            <div class="mb-1 flex gap-2">
                <span
                    class="rounded bg-white/10 px-2 py-0.5 text-[10px] font-bold tracking-wider text-textColor/80 uppercase backdrop-blur-md"
                    >{media.genres?.join(', ')}</span>
            </div>

            <h2 class="font-outline text-4xl leading-tight font-black tracking-tighter text-textColor">
                {media.title}
            </h2>

            <div class="mt-2 flex items-center gap-4 text-sm font-medium text-slate-300">
                <span class="font-outline flex items-center gap-1"
                    ><span class="font-outline text-sm text-amber-400"><StarIcon class="text-md" /></span
                    >{averageRating?.toFixed(1) ?? '-'}</span>
                <span class="font-outline">{media.released}</span>
                <span class="font-outline">{formattedRuntime}</span>
                <span class="font-outline rounded-sm border border-white/20 px-1 text-[10px]"
                    >{media.content_rating}</span>
            </div>
            <div class="mt-3 flex items-center justify-between gap-4 text-sm font-medium">
                {#each mediaRatings as rating (rating.source)}
                    <RatingCard {rating} />
                {/each}
            </div>
        </div>
    </div>

    <div class="flex-1 space-y-6 overflow-y-auto px-8 py-6">
        <div class="space-y-4">
            <h3 class="text-[10px] font-bold tracking-[0.2em] text-slate-500 uppercase">The Story</h3>
            <p class="text-sm leading-relaxed font-normal text-slate-300">
                {media.blurb}
            </p>
        </div>
        {#if cast?.length > 0}
            {@render crewList(cast, 'Starring')}
        {/if}
        {#if directors?.length > 0}
            {@render crewList(directors, directors?.length > 1 ? 'Directors' : 'Director')}
        {/if}
        {#if creators?.length > 0}
            {@render crewList(creators, creators?.length > 1 ? 'Creators' : 'Creator')}
        {/if}
    </div>
    <div class="border-t border-white/5 bg-white/5 p-4 backdrop-blur-xl">
        <div class="flex gap-3">
            <button
                disabled={$loadingStates.isPlayButtonLoading}
                onclick={() => {
                    loadVideo(media, null)
                }}
                class="group flex flex-1 items-center justify-center gap-2 rounded-xl bg-slate-800 py-3 font-bold text-textColor shadow-lg shadow-backgroundColor hover:text-primaryColor">
                <PlayIcon class="text-xl group-hover:text-primaryColor " />
                {#if $loadingStates.isPlayButtonLoading}<LoadingSpinner />{:else}Play{/if}
            </button>
            <button
                onclick={() => ($modals.trailer = true)}
                disabled={!media?.trailer}
                class="{media?.trailer
                    ? 'text-textColor hover:text-primaryColor'
                    : 'cursor-default! text-neutral-700'} group flex flex-1 items-center justify-center gap-2 rounded-xl bg-slate-800 py-3 font-bold shadow-lg shadow-backgroundColor">
                <TrailerIcon
                    class="text-xl {media?.trailer
                        ? 'text-textColor group-hover:text-primaryColor'
                        : 'cursor-default! text-neutral-700'} " />
                Trailer
            </button>
        </div>
    </div>
</aside>
