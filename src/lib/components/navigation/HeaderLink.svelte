<script lang="ts">
    import { page } from '$app/state'

    let { path, name } = $props()

    let currentPage = $derived(name === 'Home' ? '/' : `/${name.toLowerCase()}`)
</script>

<div
    style="--wails-draggable:no-drag"
    class="after:from-primary relative flex cursor-pointer items-center pr-1 after:absolute after:-bottom-[1.5px] after:left-0 after:h-0.5 after:w-0 after:bg-linear-to-r after:to-pink-500 after:transition-all after:duration-100"
    class:hover:after:w-full={page.url.pathname !== currentPage}
    class:after:w-full={page.url.pathname === currentPage}>
    <svg xmlns="http://www.w3.org/2000/svg" height="20" viewBox="0 -960 960 960" width="20">
        <path class:fill-primary={page.url.pathname === currentPage} d={path} fill="currentColor" />
    </svg>
    <p
        class="textShadow hidden md:block {page.url.pathname.includes('details')
            ? 'text-detailsPage'
            : 'text-text'}">
        {name}
    </p>
</div>
