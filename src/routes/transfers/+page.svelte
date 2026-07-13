<script lang="ts">
    import TransferCard from '$lib/components/cards/TransferCard.svelte'
    import { startTransferPolling, stopTransferPolling } from '$lib/functions/plugins/transferPolling'
    import { transfersInProgress } from '$lib/stores/plugins'
    import { onDestroy, onMount } from 'svelte'

    onMount(() => {
        const transfers = Object.values($transfersInProgress)
        for (const transfer of transfers) {
            startTransferPolling(transfer)
        }
    })

    onDestroy(() => {
        const transfers = Object.values($transfersInProgress)
        for (const transfer of transfers) {
            stopTransferPolling(transfer)
        }
    })
</script>

<section class="relative h-screen w-full max-w-full flex-col space-y-10 overflow-y-auto p-16">
    <div class="absolute bottom-0 left-0 -z-10 h-75 w-75 rounded-full bg-blue-900/10 blur-[100px]"></div>
    <header class="mb-12">
        <h2 class="font-headline text-on-surface text-5xl font-black tracking-tighter">Transfers</h2>
    </header>
    <div class="grid w-full max-w-5xl grid-cols-1 gap-6">
        {#each Object.entries($transfersInProgress) as [key, transfer] (key)}
            <TransferCard {transfer} />
        {:else}
            <p>No active transfers found.</p>
        {/each}
    </div>
</section>
