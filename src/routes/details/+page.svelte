<script lang="ts">
    import { page } from '$app/state'
    import { onDestroy } from 'svelte'
    import StarIcon from 'virtual:icons/material-symbols/star'
    import TrailerIcon from 'virtual:icons/material-symbols/smart-display'
    import { formatRuntime } from '$lib/functions/ui/formatRuntime.js'
    import { formatAverageRating } from '$lib/functions/ui/formatAverageRating.js'
    import { orderRatings } from '$lib/functions/ui/orderRatings.js'
    import { parseCredits } from '$lib/functions/utility/parseCredits.js'
    import { selectSeason } from '$lib/functions/utility/selectSeason.js'
    import { getImagePath } from '$lib/functions/ui/getImagePath.js'
    import type { Api } from '$lib/types/api.js'
    import { getFirstUnwatchedEpisode } from '$lib/functions/utility/getFirstUnwatchedEpisode.js'
    import { areSourceFiltersEnabled } from '$lib/stores/user.js'
    import { modals, scrollPositions } from '$lib/stores/app.js'
    import TrailerModal from '$lib/components/modals/TrailerModal.svelte'
    import CastCard from '$lib/components/cards/CastCard.svelte'
    import WatchedButton from '$lib/components/inputs/WatchedButton.svelte'
    import HideButton from '$lib/components/inputs/HideButton.svelte'
    import WatchlistButton from '$lib/components/inputs/WatchlistButton.svelte'
    import FavouriteButton from '$lib/components/inputs/FavouriteButton.svelte'
    import RatingCard from '$lib/components/cards/RatingCard.svelte'
    import UserRating from '$lib/components/inputs/UserRating.svelte'
    import Collections from '$lib/components/ui/Collections.svelte'
    import SourcesSideBar from '$lib/components/bars/SourcesSideBar.svelte'
    import DetailsSideBar from '$lib/components/bars/DetailsSideBar.svelte'
    import TransferModal from '$lib/components/modals/TransferModal.svelte'
    import type { Plugins } from '$lib/types/plugins.js'
    import DownloadModal from '$lib/components/modals/DownloadModal.svelte'
    import { fetchRelatedMedia } from '$lib/db/fetchRelatedMedia.js'
    import { cache } from '$lib/stores/pages.js'
    import { handleError } from '$lib/functions/errors/errorHandling.js'
    import RelatedMedia from '$lib/components/ui/RelatedMedia.svelte'
    import { fetchCollections } from '$lib/db/fetchCollections.js'

    const { data } = $props()
    let media = $derived(data.data)
    let relatedFetched = $state(false)
    let collectionsFetched = $state(false)

    const formattedRuntime = $derived(formatRuntime(media))
    const averageRating = $derived(formatAverageRating(media))
    const mediaRatings = $derived(orderRatings(media?.ratings))
    let showWatchedEpisodes = $state(false)
    const { cast, directors, writers, creators } = $derived(parseCredits(media, 10))
    let userSelectedSeason: number | null = $state(null)
    let selectedSeason = $derived(
        media?.type === 'tv' ? selectSeason(media, showWatchedEpisodes, userSelectedSeason) : null,
    )
    const backdropPath = $derived.by(() => {
        if (media?.backdrop) return getImagePath(media.backdrop, 'original')
        return null
    })
    const logoPath = $derived.by(() => {
        if (media?.logo) return getImagePath(media.logo, 'original')
        return null
    })
    let selectedEpisode: Api.Episode | null = $derived(null)
    let isSourcesOpen = $state(false)
    let selectedSource: Plugins.IndexerSource | null = $state(null)

    let collections = $state<Api.Collection[]>([])
    let relatedMedia = $state<Api.Collection[]>([])

    const filteredCollections = $derived.by(() => {
        if (!collections.length) return []
        const updatedCollections = collections.map((collection: Api.Collection) => ({
            ...collection,
            collection_items: collection.collection_items.filter((item) => item.media_id !== page.data.data.id),
        }))

        return updatedCollections.filter((collection: Api.Collection) => collection.collection_items.length !== 0)
    })

    $effect(() => {
        if (selectedSeason) {
            selectedEpisode = getFirstUnwatchedEpisode(selectedSeason, media, showWatchedEpisodes)
        }
    })

    onDestroy(() => {
        $areSourceFiltersEnabled = true
        $scrollPositions.episodeContainer = {
            x: 0,
            y: 0,
        }
    })

    $effect(() => {
        const mediaId = media.id
        const currentMedia = media

        if (!relatedFetched) {
            relatedFetched = true
            fetchRelatedMedia(mediaId)
                .then((resp) => {
                    if (!resp.success) throw resp.error
                    relatedMedia = resp.data
                    const index = $cache.details.media.findIndex((item) => item.id === currentMedia.id)
                    if (index !== -1) {
                        $cache.details.media[index] = currentMedia
                    } else {
                        $cache.details.media = [...$cache.details.media, currentMedia]
                    }
                })
                .catch((error) => handleError(error))
        }

        if (!collectionsFetched) {
            collectionsFetched = true
            fetchCollections(mediaId)
                .then((resp) => {
                    if (!resp.success) throw resp.error
                    collections = resp.data
                    const index = $cache.details.media.findIndex((item) => item.id === currentMedia.id)
                    if (index !== -1) {
                        $cache.details.media[index] = currentMedia
                    } else {
                        $cache.details.media = [...$cache.details.media, currentMedia]
                    }
                })
                .catch((error) => handleError(error))
        }
    })
</script>

{#if $modals.trailer}
    <TrailerModal {media} />
{/if}

{#if $modals.transfer}
    <TransferModal {selectedSource} />
{/if}

{#if $modals.download}
    <DownloadModal {media} source={selectedSource} seasonNumber={userSelectedSeason} episodeData={selectedEpisode} />
{/if}

{#snippet crewList(crewItems: Api.CastMember[], type: string)}
    <div>
        <h3 class="mb-4 text-[10px] font-bold tracking-[0.2em] text-slate-500 uppercase">{type}</h3>
        <div class="flex flex-wrap gap-x-4 gap-y-4">
            {#each crewItems as crew, i (i)}
                <div class="w-75">
                    <CastCard credit={crew} />
                </div>
            {/each}
        </div>
    </div>
{/snippet}

<section class="glass-panel z-0 flex h-screen w-full flex-col">
    <div class="relative h-[45%] min-h-100 w-full shrink-0">
        {#if media.backdrop}
            <div
                class="absolute inset-0 bg-cover bg-top"
                data-alt="Backdrop Image of {media.title}"
                style="background-image: url({backdropPath})">
            </div>
        {/if}
        <div class="absolute inset-0 bg-linear-to-t from-backgroundColor via-backgroundColor/40 to-transparent"></div>
        <div class="absolute right-8 bottom-6 left-8">
            {#if media.logo}
                <img
                    class="relative top-0 mx-auto h-36 max-h-36 max-w-[90%] justify-center"
                    src={logoPath}
                    alt={media.title} />
            {/if}
            <div class="mb-1 flex gap-2">
                <span
                    class="rounded bg-white/10 px-2 py-0.5 text-[10px] font-bold tracking-wider text-textColor/80 uppercase backdrop-blur-md"
                    >{media.genres?.join(', ')}</span>
            </div>

            <h2 class="font-outline text-4xl leading-tight font-black tracking-tighter text-textColor">
                {media.title}
            </h2>

            <div class="mt-2 flex w-full text-sm font-medium text-slate-300">
                <div class="flex w-4/5 items-center gap-8">
                    <span class="flex items-center gap-1"
                        ><span class="font-outline text-sm text-amber-400"><StarIcon class="text-md" /></span
                        >{averageRating?.toFixed(1) ?? '-'}</span>
                    <span class="font-outline">{media.released}</span>
                    <span class="font-outline">{formattedRuntime}</span>
                    <span class="font-outline rounded-sm border border-white/20 px-1 text-[10px]"
                        >{media.content_rating}</span>
                    <span class="font-outline">{media.status}</span>
                    <span class="font-outline">{media.language}</span>
                    <span class="font-outline">{media.country}</span>
                </div>
                <div class="ml-auto flex w-1/3 max-w-75 justify-between">
                    <WatchedButton bind:media size="text-xl" />
                    <HideButton bind:media size="text-xl" />
                    <WatchlistButton bind:media size="text-xl" />
                    <FavouriteButton bind:media size="text-xl" />
                </div>
            </div>
            <div class="mt-3 flex w-full items-center justify-between text-sm font-medium">
                <div class="flex w-1/2 max-w-275 justify-between">
                    {#each mediaRatings as rating (rating.source)}
                        <RatingCard {rating} />
                    {/each}
                </div>

                <div class="flex w-75">
                    <UserRating bind:media textColor="text-textColor" baseOpacity={30} />
                </div>
            </div>
        </div>
    </div>
    <div class="w-48 px-8 pb-2">
        <button
            onclick={() => ($modals.trailer = true)}
            disabled={!media?.trailer}
            class="{media?.trailer
                ? 'text-textColor hover:text-primaryColor'
                : 'cursor-default! text-neutral-700'} group flex items-center justify-center gap-1 rounded bg-slate-800 px-3 py-1 text-xs font-bold shadow-lg shadow-backgroundColor">
            <TrailerIcon
                class="text-xs {media?.trailer
                    ? 'text-textColor group-hover:text-primaryColor'
                    : 'cursor-default! text-neutral-700'} " />
            Trailer
        </button>
    </div>
    <div class="space-y-6 overflow-y-auto px-8 py-6">
        <div class="space-y-2">
            <h3 class="text-center text-sm leading-relaxed font-normal text-slate-300 italic">{media.tagline}</h3>
            <h3 class="text-[10px] font-bold tracking-[0.2em] text-slate-500 uppercase">The Story</h3>
            <p class=" leading-relaxed font-normal text-slate-300">
                {media.blurb}
            </p>
        </div>
        {#if cast?.length > 0}
            {@render crewList(cast, 'Starring')}
        {/if}
        {#if directors?.length > 0}
            {@render crewList(directors, directors?.length > 1 ? 'Directors' : 'Director')}
        {/if}
        {#if writers?.length > 0}
            {@render crewList(writers, writers?.length > 1 ? 'Writers' : 'Writer')}
        {/if}
        {#if creators?.length > 0}
            {@render crewList(creators, creators?.length > 1 ? 'Creators' : 'Creator')}
        {/if}
        {#if filteredCollections?.length > 0}
            <div class="py-6">
                <h3 class="mb-4 text-[10px] font-bold tracking-[0.2em] text-slate-500 uppercase">Collections</h3>
                <div class="flex flex-col">
                    <Collections collections={filteredCollections} unfilteredCollection={collections} />
                </div>
            </div>
        {/if}
        {#if relatedMedia?.length > 0}
            <div class="py-6">
                <h3 class="mb-4 text-[10px] font-bold tracking-[0.2em] text-slate-500 uppercase">Related</h3>
                <div class="flex flex-col">
                    <RelatedMedia related={relatedMedia} />
                </div>
            </div>
        {/if}
    </div>
</section>

{#if isSourcesOpen}
    <SourcesSideBar {media} {selectedSeason} {selectedEpisode} bind:selectedSource bind:isSourcesOpen />
{:else}
    <DetailsSideBar
        bind:showWatchedEpisodes
        bind:media
        bind:selectedSeason
        bind:selectedEpisode
        bind:isSourcesOpen
        bind:userSelectedSeason />
{/if}
