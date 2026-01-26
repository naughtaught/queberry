<script>
    import { page } from '$app/state'

    const { path, name } = $props()
</script>

<button aria-label="Header button" class="flex cursor-pointer hover:scale-125" style="--wails-draggable:no-drag">
    <svg
        class="shrink-0 {page.url.pathname.includes('details') ? 'text-detailsPageTextColor' : 'text-textColor'}"
        xmlns="http://www.w3.org/2000/svg"
        height="20"
        viewBox="0 -960 960 960"
        width="20"
        ><path class:fill-primaryColor={page.url.pathname === `/${name}`} fill="currentColor" d={path} /></svg>
</button>
