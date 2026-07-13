<script lang="ts">
    import { page } from '$app/state'
    import { resolve } from '$app/paths'
    // TODO generic error page
</script>

<div>
    <h1>Generic Error Page</h1>
    <p>Status: {page.status}</p>
    <p>Error: {page.error?.message}</p>

    {#if page.status === 404}
        <p>The page you're looking for couldn't be found.</p>
        <a href={resolve('/', {})}>Back to Home</a>
    {:else}
        <p>Something went wrong.</p>
        <a href={resolve('/', {})}>Back to Home</a>
    {/if}
</div>
