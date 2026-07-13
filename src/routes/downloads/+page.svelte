<script lang="ts">
    import { downloadsInProgress } from '$lib/stores/plugins'
    import DownloadCard from '$lib/components/cards/DownloadCard.svelte'
</script>

<section class="relative h-screen w-full max-w-full flex-col space-y-10 overflow-y-auto p-16">
    <div class="absolute bottom-0 left-0 -z-10 h-75 w-75 rounded-full bg-blue-900/10 blur-[100px]"></div>
    <header class="mb-12">
        <h2 class="font-headline text-on-surface text-5xl font-black tracking-tighter">Downloads</h2>
    </header>
    <div class="grid w-full max-w-5xl grid-cols-1 gap-6">
        {#each $downloadsInProgress as download (download.link.uuid)}
            <DownloadCard {download} />
        {:else}
            <p>No downloads found.</p>
        {/each}
    </div>
</section>
