<script lang="ts">
    import { handleError } from '$lib/functions/errors/errorHandling'
    import type { Plugins } from '$lib/types/plugins'
    import { onMount } from 'svelte'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import { settings } from '$lib/stores/user'
    import { get } from 'svelte/store'
    import { enabledResolverPlugins, transfersInProgress } from '$lib/stores/plugins'
    import { fetchTransfers } from '$lib/functions/plugins/fetchTransfers'
    import Checkbox from '../inputs/Checkbox.svelte'
    import { deleteTransfer } from '$lib/functions/plugins/deleteTransfer'

    let { isTransferListOpen = $bindable() } = $props()

    let loading = $state(true)
    let transfers: Plugins.TransferData[] = $state([])
    let transfersToDelete: Plugins.TransferData[] = $state([])
    const defaultResolver = $derived(get(settings).defaultResolver)
    const resolver = $derived(get(enabledResolverPlugins).find((x) => x.id === defaultResolver))

    const deleteTransfers = async (): Promise<void> => {
        if (!resolver) {
            handleError('No resolver found')
            return
        }

        loading = true

        try {
            for (const transfer of transfersToDelete) {
                await deleteTransfer(transfer.infohash)

                if (!$transfersInProgress[transfer.infohash]) {
                    const index = transfers.findIndex((t) => t.id === transfer.id)
                    if (index !== -1) transfers.splice(index, 1)
                }
            }
        } catch (error) {
            handleError(error, { display: false, log: false })
        } finally {
            loading = false
            isTransferListOpen = false
        }
    }

    const selectAll = (): void => {
        transfersToDelete = []
        transfers.forEach((transfer) => {
            transfersToDelete.push(transfer)
        })
    }

    onMount(async () => {
        if (resolver) {
            const allFiles = await fetchTransfers(resolver)
            transfers = allFiles.filter((x) => x.status !== 'cached')
        }
        loading = false
    })
</script>

<div class="relative mt-2 flex w-225 flex-col rounded">
    <div class="text-text sticky top-0 py-2 text-center font-medium">
        <h1>Select Transfers to Cancel</h1>
    </div>
    <div class="mb-5 flex w-full justify-between px-5 text-sm font-medium text-textColor">
        <div class="flex items-center">
            {#if transfers.length > 0}
                {#if transfers.length !== transfersToDelete.length}
                    <button onclick={selectAll}>Select all</button>
                {:else}
                    <button onclick={() => (transfersToDelete = [])}>Deselect all</button>
                {/if}
            {/if}
        </div>
    </div>
    {#if loading}
        <div class="mt-5 flex min-h-48 items-center justify-center">
            <LoadingSpinner />
        </div>
    {:else}
        <div class="mt-5 flex min-h-48 flex-col space-y-4 overflow-auto px-5">
            {#if transfers.length > 0}
                {#each transfers as transfer (transfer.id)}
                    <div class="inline-flex items-center">
                        <Checkbox
                            id={transfer.id}
                            checked={transfersToDelete.some((item) => item.id === transfer.id)}
                            func={(e: Event) => {
                                const target = e.target as HTMLInputElement | null
                                if (target && target.checked) {
                                    transfersToDelete = [...transfersToDelete, transfer]
                                } else if (target) {
                                    transfersToDelete = transfersToDelete.filter((item) => !(item.id === transfer.id))
                                }
                            }} />
                        <label class="text-text ml-2 flex cursor-pointer gap-x-4 text-sm" for={transfer.id}>
                            <span>{transfer.progress}%</span>
                            <span>{transfer.status}</span>
                            <span>{transfer.filename}</span>
                        </label>
                    </div>
                {/each}
            {:else}
                <p class="text-text ml-2 cursor-pointer text-sm">No Transfers Found.</p>
            {/if}
        </div>
    {/if}
    <div class="mx-auto flex w-fit items-center justify-center gap-x-5 py-5">
        <button
            onclick={() => {
                isTransferListOpen = false
            }}
            class="px-6 py-3.5 font-medium text-slate-400 transition-colors hover:text-slate-200">
            Cancel
        </button>
        <button
            disabled={transfersToDelete.length === 0 || loading}
            onclick={deleteTransfers}
            class="{transfersToDelete.length === 0 || loading
                ? ' cursor-default! text-slate-500'
                : 'text-textColor hover:text-primaryColor'} flex-1 rounded-lg bg-slate-800 px-6 py-3.5 font-bold shadow-lg transition-all">
            {#if loading}
                <LoadingSpinner />
            {:else}
                Delete
            {/if}
        </button>
    </div>
</div>
