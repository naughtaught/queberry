<script lang="ts">
    import '../app.css'

    import { Header } from '$lib'
    import { page } from '$app/state'

    const { children } = $props()
</script>

{#if !page.url.pathname.includes('video')}
    <div class="fixed top-0 left-0 z-50 w-full">
        <Header />
    </div>
{/if}

{@render children()}
