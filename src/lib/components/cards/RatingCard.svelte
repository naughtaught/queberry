<script lang="ts">
    import { formatRatingCounts } from '$lib/functions/ui/formatRatingCounts'
    import { getRatingColor } from '$lib/functions/ui/getRatingColor'

    const { rating } = $props()

    const color = $derived(getRatingColor(rating.rating, 'text'))

    const icon = $derived.by(() => {
        if (rating.source === 'tomatometer') {
            return rating >= 60 ? 'tomatometer' : 'tomatometer_splat'
        } else if (rating.source === 'popcornmeter') {
            return rating >= 60 ? 'popcornmeter' : 'popcornmeter_spill'
        } else {
            return rating.source
        }
    })
</script>

<div class="relative flex items-center">
    <img class="max-h-7" width="24" src="/ratings/{rating.source}.svg" alt="{icon} logo" />
    <div class="ml-1 flex flex-col">
        <p class="font-outline {color}">{rating.rating}</p>
        {#if rating.count}
            <p class="font-outline text-left text-xs text-slate-400">{formatRatingCounts(rating.count)}</p>
        {/if}
    </div>
</div>
