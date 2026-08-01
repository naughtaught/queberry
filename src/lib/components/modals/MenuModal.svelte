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
        isReportModalOpen = $bindable(),
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
            false,
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
    use:clickOutside={{ ignore: '#menu-button, #report-modal' }}
    onclickOutside={() => {
        isMenuOpen = false
    }}
    class="glass-panel fixed bottom-12 z-10 mb-8 flex h-1/2 w-full rounded-t-xl border border-slate-200/10 shadow-2xl">
    <div class="relative flex w-full flex-col">
        <div class="flex-1 space-y-5 p-4 text-white">
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
        <div class="flex w-full items-center justify-between p-4">
            <div>
                <p class="text-sm font-bold">Report</p>
                <p class="text-xs">Content Issue</p>
            </div>
            <button
                onclick={() => {
                    isReportModalOpen = true
                }}
                class="rounded-lg border border-white/5 bg-slate-800 px-6 py-2 text-xs font-bold transition-colors hover:bg-white/10"
                >Report</button>
        </div>
    </div>
</div>
