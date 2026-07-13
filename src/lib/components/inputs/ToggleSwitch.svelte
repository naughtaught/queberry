<script lang="ts">
    let { options, value = $bindable(), func } = $props()
</script>

<div class="grid rounded bg-slate-800" style="grid-template-columns: repeat({options.length}, 1fr);">
    {#each options as option, index (index)}
        <button
            class="h-9 min-w-20 px-4 py-2 text-center text-sm font-semibold tracking-wide text-white uppercase shadow-lg transition-colors"
            class:bg-primaryColor={option.toLowerCase() === value.toLowerCase()}
            class:rounded-l={option === options[0]}
            class:rounded-r={option === options.at(-1)}
            onclick={() => {
                value = option
                func()
            }}>
            {option}
        </button>
    {/each}
</div>
