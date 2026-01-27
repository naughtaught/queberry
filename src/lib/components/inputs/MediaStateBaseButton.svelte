<script lang="ts">
    const { path, func, label, text = '', hasBackground = false, height = '25', width = '25' } = $props()

    const background = $derived(hasBackground ? 'flex items-center rounded bg-gray-500/50 p-2 w-[40px]' : '')
</script>

<button onclick={func} class="hover:cursor-pointer {background}  group" aria-label={label}>
    <svg
        class="group-hover:scale-105"
        class:mr-1={text}
        xmlns="http://www.w3.org/2000/svg"
        {height}
        viewBox="0 -960 960 960"
        {width}><path stroke-width="20" stroke="black" d={path} /></svg>
    {text}
</button>
