<script lang="ts">
    import { resolve } from '$app/paths'
    import { getImagePath } from '$lib/functions/ui/getImagePath'
    import { posterSize } from '$lib/functions/ui/posterSize'
    import { detailsMedia } from '$lib/stores/pages'
    import { settings } from '$lib/stores/user'
    import CardPlayButton from '$lib/components/inputs/CardPlayButton.svelte'
    import FavouriteButton from '$lib/components/inputs/FavouriteButton.svelte'
    import HideButton from '$lib/components/inputs/HideButton.svelte'
    import UserRating from '$lib/components/inputs/UserRating.svelte'
    import WatchedButton from '$lib/components/inputs/WatchedButton.svelte'
    import WatchlistButton from '$lib/components/inputs/WatchlistButton.svelte'
    import CardRating from '$lib/components/ui/CardRating.svelte'

    let { media, showEpisodes = false, updateDetails = true } = $props()

    const imagescaling = $derived($settings.imageScaling)
    const scale = $derived(Math.max(0.5, Math.min(2, (imagescaling ?? 100) / 100)))
    const minHpx = $derived(300 * scale)
    const maxHvh = $derived(25 * scale)
    const minWvw = $derived(maxHvh * (2 / 3))
    const avgRating = $derived(media?.ratings?.find((r: { source: string }) => r.source === 'average')?.rating ?? null)
    const totalAiredEpisodes = $derived.by(() => {
        if (media?.type === 'tv' && showEpisodes) {
            const defaultOrNoGroup = media.episode_group_name === 'Default' || !media.episode_group_name
            if (defaultOrNoGroup) {
                return media.episode_counts?.['default']
            }
            const key = media.episode_group_name.toLowerCase().replace(/\s+/g, '_').replace('_episodes', '')
            return media.episode_counts?.[key] ?? media.episode_counts?.['default']
        }
        return null
    })
    const posterPath = $derived(getImagePath(media?.poster, posterSize(imagescaling, 'carousel')))
</script>

<div
    class="group flex aspect-2/3 h-full shadow-xl"
    style={`min-height: ${minHpx}px; min-width: ${minWvw}vh; max-height: ${maxHvh}vh;`}>
    <a
        data-sveltekit-preload-data="off"
        class="h-full w-full"
        onclick={() => {
            if (updateDetails) $detailsMedia = media
        }}
        href={resolve(`/details/?id=${media.id}&type=${media.type}`, {})}>
        <div class="relative h-full w-full">
            {#if avgRating}
                <div class="absolute -top-3 -left-2 hidden h-16 w-16 group-hover:flex">
                    <CardRating rating={avgRating} />
                </div>
            {/if}

            <div class="absolute top-1 right-1 z-10 hidden group-hover:flex">
                <CardPlayButton {media} background="bg-gray-700/50" size="text-3xl" />
            </div>
            {#if totalAiredEpisodes && showEpisodes}
                <div class="absolute top-1 right-2 z-0 text-xl font-bold group-hover:hidden">
                    <span class="text-detailsPage font-outline"
                        >{media.watched_episodes}&nbsp;/&nbsp;{totalAiredEpisodes}</span>
                </div>
            {/if}
            <img
                class:glow-shadow={showEpisodes ? media?.has_unwatched_finale : media?.has_released_season_finale}
                class="aspect-2/3 h-full w-full rounded-lg object-cover shadow-lg"
                src={posterPath ? posterPath : '/images/poster-placeholder.png'}
                alt={media.title} />

            {#if $settings.enableUserRatings}
                <div class="absolute bottom-7 left-0 mb-5 ml-1 hidden w-[96%] px-2 group-hover:flex">
                    <UserRating bind:media textColor="text-textColor" baseOpacity={50} />
                </div>
            {/if}
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
