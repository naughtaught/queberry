<script lang="ts">
    import { getFirstUnwatchedEpisode } from '$lib/functions/utility/getFirstUnwatchedEpisode'
    import { clickOutside } from '$lib/functions/utility/useClickOutside'
    import type { Api } from '$lib/types/api'
    import { onDestroy } from 'svelte'

    let {
        media,
        selectedSeason = $bindable(),
        selectedEpisode = $bindable(),
        showWatchedEpisodes,
        selectedEpisodeGroup,
    } = $props()

    let isOpen = $state(false)
    let dropdownElement: HTMLElement | null = $state(null)

    const seasons = $derived.by(() => {
        let allSeasons = media.seasons.seasons
        const preferredEpisodeKey = selectedEpisodeGroup ? `${selectedEpisodeGroup}` : 'default_episodes'

        if (selectedEpisodeGroup !== 'Default') {
            allSeasons = media.seasons.seasons.filter((season: { season_num: number }) => season.season_num !== 0)
        } else {
            allSeasons.sort((a: { season_num: number }, b: { season_num: number }) => a.season_num - b.season_num)
        }

        if (showWatchedEpisodes) return allSeasons

        return allSeasons.filter((season: Api.Season) => {
            const episodes = (season[preferredEpisodeKey] ?? season.default_episodes) as Api.Episode[]
            return episodes?.some((episode: Api.Episode) => !episode.watched)
        })
    })

    const checkIfScrollbar = (): boolean => {
        if (dropdownElement) {
            return dropdownElement.scrollHeight > dropdownElement.clientHeight
        }
        return false
    }

    const hasScrollbar = $derived(checkIfScrollbar())

    const handleMenuClick = (): void => {
        isOpen = !isOpen
    }

    const handleSeasonClick = (event: MouseEvent | KeyboardEvent, season: Api.Season): void => {
        event.stopPropagation()
        selectedSeason = season
        selectedEpisode = getFirstUnwatchedEpisode(selectedSeason, media, showWatchedEpisodes) ?? 1
        isOpen = false
    }

    onDestroy(() => {
        isOpen = false
    })
</script>

{#if media.seasons.seasons.length > 1}
    <button
        use:clickOutside
        onclickOutside={() => {
            isOpen = false
        }}
        onclick={handleMenuClick}
        class="relative w-full border-y border-white/5 py-2 text-center text-[10px] font-bold tracking-[0.2em] text-slate-500 uppercase">
        {#if selectedSeason.season_num === 0}Specials{:else}Season {selectedSeason.season_num}{/if}
        {#if isOpen}
            <div
                bind:this={dropdownElement}
                class="{hasScrollbar
                    ? 'pl-2'
                    : ''} absolute top-full left-0 z-20 mt-0.5 h-fit max-h-[20vh] w-full space-y-3 overflow-y-auto border-b border-white/5 bg-backgroundColor py-6">
                {#each seasons as season (season.season_id)}
                    <div
                        tabindex="0"
                        onclick={(event) => handleSeasonClick(event, season)}
                        onkeydown={(event) => handleSeasonClick(event, season)}
                        role="button"
                        class="w-full text-center">
                        {#if season.season_num === 0}Specials{:else}Season {season.season_num}{/if}
                    </div>
                {/each}
            </div>
        {/if}
    </button>
{/if}
