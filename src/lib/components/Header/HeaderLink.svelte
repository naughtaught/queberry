<script lang="ts">
    import { page } from '$app/state'

    const { name, children } = $props()

    const currentPage = $derived(name === 'Home' ? '/' : `/${name.toLowerCase()}`)
</script>

<div
    style="--wails-draggable:no-drag"
    class="relative flex cursor-pointer items-center pr-1 after:absolute after:-bottom-[1.5px] after:left-0 after:h-0.5 after:w-0 after:bg-primaryColor after:transition-all after:duration-100"
    class:hover:after:w-full={page.url.pathname !== currentPage}
    class:after:w-full={page.url.pathname === currentPage}>
    {@render children?.()}
    <p
        class="textShadow ml-1 hidden md:block {page.url.pathname.includes('details')
            ? 'text-detailsPageTextColor'
            : 'text-textColor'}">
        {name}
    </p>
</div>
