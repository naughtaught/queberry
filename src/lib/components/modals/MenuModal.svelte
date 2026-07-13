<script lang="ts">
    import { clickOutside } from '$lib/functions/utility/useClickOutside'
    import { areSourceFiltersEnabled } from '$lib/stores/user'
    import Checkbox from '$lib/components/inputs/Checkbox.svelte'
    import { indexerSources } from '$lib/stores/plugins'
    import { loadingStates } from '$lib/stores/app'
    import { fetchSources } from '$lib/functions/plugins/fetchSources'

    let {
        isMenuOpen = $bindable(),
        sources = $bindable(),
        showWatchedEpisodes = $bindable(null),
        media,
        selectedSeason,
        selectedEpisode,
    } = $props()

    const handleSourceFilters = async (): Promise<void> => {
        $loadingStates.isSourcesLoading = true

        const key = media.type === 'tv' ? (selectedEpisode?.episode_id ?? media.imdb_id) : media.imdb_id
        const existingSource = $indexerSources.find((source) => source[key])
        if (existingSource) delete existingSource[key]

        const resp = await fetchSources(
            media.imdb_id,
            media.title,
            media.released,
            media.type,
            selectedSeason?.season_num,
            selectedEpisode?.episode_num,
            selectedEpisode?.episode_id,
            false
        )

        if (resp.success && resp.data.length > 0) {
            sources = resp.data
        } else {
            sources = []
        }

        $loadingStates.isSourcesLoading = false
    }
</script>

<div
    use:clickOutside={{ ignore: '#menu-button' }}
    onclickOutside={() => {
        isMenuOpen = false
    }}
    class="glass-panel fixed bottom-12 z-10 mb-8 flex h-1/2 w-full rounded-t-xl border border-slate-200/10 shadow-2xl">
    <div class="relative">
        <div class="space-y-5 p-4 text-white">
            <h2 class="text-xl font-bold">Menu</h2>
            <div class="flex flex-col gap-y-3">
                {#if media.type === 'tv'}
                    <Checkbox label="Include Watched Episodes" bind:checked={showWatchedEpisodes} func={() => {}} />
                {/if}
                <Checkbox
                    label="Source Filters"
                    bind:checked={$areSourceFiltersEnabled}
                    func={() => handleSourceFilters()} />
            </div>
        </div>
    </div>
</div>
