<script lang="ts">
    let { label, min = $bindable(), max = $bindable(), step = 1 } = $props()
</script>

<label class="flex flex-col text-sm" for="">
    {label}
    <div class="mt-1 space-x-1">
        <input
            type="number"
            placeholder="Min Ratings"
            bind:value={min}
            {min}
            {max}
            {step}
            class="w-24 rounded border border-slate-700 bg-slate-800/50 px-2 text-xs text-textColor"
            aria-label="Minimum Ratings Count" />
        <span class="text-sm text-slate-100 opacity-60">-</span>
        <input
            type="number"
            placeholder="Max Ratings"
            bind:value={max}
            {min}
            {max}
            {step}
            class="w-24 rounded border border-slate-700 bg-slate-800/50 px-2 text-xs text-textColor"
            aria-label="Maximum Ratings Count" />
    </div>
</label>
