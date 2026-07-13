<script lang="ts">
    let { value = $bindable(), func } = $props()
</script>

<div class="flex rounded bg-slate-800">
    <button
        class="h-9 min-w-20 px-4 py-2 text-center text-sm font-semibold tracking-wide text-white uppercase shadow-lg transition-colors"
        class:bg-primaryColor={!value}
        class:rounded-l={!value}
        onclick={() => {
            value = !value
            func()
        }}>
        Off
    </button>
    <button
        class="h-9 min-w-20 px-4 py-2 text-center text-sm font-semibold tracking-wide text-white uppercase shadow-lg transition-colors"
        class:bg-primaryColor={value}
        class:rounded-r={value}
        onclick={() => {
            value = !value
            func()
        }}>
        On
    </button>
</div>
