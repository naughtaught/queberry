<script lang="ts">
    import '../app.css'

    import { Header, scrollYPosition } from '$lib'
    import { page } from '$app/state'
    import { afterNavigate, beforeNavigate } from '$app/navigation'

    const { children } = $props()

    let scrollY: number = $state(0)

    beforeNavigate(() => {
        if (page.url.pathname !== '/video') $scrollYPosition = scrollY
    })

    afterNavigate(() => {
        if ($scrollYPosition > 0) {
            setTimeout(() => {
                window.scrollTo(0, $scrollYPosition)
            }, 0)
        }
    })
</script>

<svelte:window bind:scrollY />

{#if !page.url.pathname.includes('video')}
    <div class="fixed top-0 left-0 z-50 w-full">
        <Header />
    </div>
{/if}

{@render children()}
