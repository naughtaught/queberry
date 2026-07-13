<script lang="ts">
    import { createError, handleError } from '$lib/functions/errors/errorHandling'
    import type { Plugins } from '$lib/types/plugins'
    import { onMount } from 'svelte'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { settings } from '$lib/stores/user'
    import { get } from 'svelte/store'
    import { enabledResolverPlugins, excludedTransferStatuses, transfersInProgress } from '$lib/stores/plugins'
    import { checkMethodApi } from '$lib/functions/plugins/checkMethodApi'

    let { isTransferListOpen = $bindable() } = $props()

    let loading = $state(true)
    let transfers: Plugins.TransferData[] = $state([])
    let transfersToDelete: Plugins.TransferData[] = $state([])
    const defaultResolver = $derived(get(settings).defaultResolver)
    const resolver = $derived(get(enabledResolverPlugins).find((x) => x.id === defaultResolver))

    const fetchTransfers = async (): Promise<void> => {
        if (!resolver) throw createError('No resolver found', 404, { log: false })

        loading = true
        transfersToDelete = []
        transfers = []

        try {
            checkMethodApi(resolver, 'GetTorrentList')

            const response = await invokeFunction('call_plugin_method', {
                pluginName: resolver.id,
                methodName: 'GetTorrentList',
                args: [resolver.apikey ?? null],
            })

            if (!response.success) throw response.error

            const listedTransfers = response.data.filter(
                (x: { status: string }) =>
                    !$excludedTransferStatuses.some((excluded) => x.status === excluded || x.status.includes(excluded)),
            )

            transfers = [...listedTransfers]
        } catch (error) {
            handleError(error)
        } finally {
            loading = false
        }
    }

    const deleteTransfers = async (): Promise<void> => {
        if (!resolver) {
            handleError('No resolver found')
            return
        }

        loading = true

        try {
            for await (const transfer of transfersToDelete) {
                try {
                    checkMethodApi(resolver, 'CancelTransfer')

                    const response = await invokeFunction('call_plugin_method', {
                        pluginName: resolver.id,
                        methodName: 'CancelTransfer',
                        args: [resolver.apikey ?? null, transfer.id],
                    })

                    if (!response.success) {
                        handleError(response.error || 'Failed to cancel transfer')
                        continue
                    }

                    const key = Object.keys($transfersInProgress).find(
                        (hash) => $transfersInProgress[hash].transferId === +transfer.id,
                    )

                    if (key) delete $transfersInProgress[key]

                    const index = transfers.findIndex((t) => t.id === transfer.id)

                    if (index !== -1) transfers.splice(index, 1)
                } catch (error) {
                    handleError(error)
                }
            }
        } catch (error) {
            handleError(error)
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
        await fetchTransfers()
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
                        <label class="relative flex cursor-pointer items-center" for={transfer.id}>
                            <input
                                type="checkbox"
                                checked={transfersToDelete.some((item) => {
                                    return item.id === transfer.id
                                })}
                                onchange={(e) => {
                                    const target = e.target as HTMLInputElement | null

                                    if (target && target.checked) {
                                        transfersToDelete = [...transfersToDelete, transfer]
                                    } else if (target) {
                                        transfersToDelete = transfersToDelete.filter(
                                            (item) => !(item.id === transfer.id),
                                        )
                                    }
                                }}
                                class="peer h-4 w-4 cursor-pointer appearance-none rounded border border-gray-300 shadow transition-all checked:border-black checked:bg-primaryColor hover:shadow-md"
                                id={transfer.id} />
                            <span
                                class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 transform text-black opacity-0 peer-checked:opacity-100">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="h-3.5 w-3.5"
                                    viewBox="0 0 20 20"
                                    fill="currentColor"
                                    stroke="currentColor"
                                    stroke-width="1">
                                    <path
                                        fill-rule="evenodd"
                                        d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                        clip-rule="evenodd"></path>
                                </svg>
                            </span>
                        </label>
                        <label class="text-text ml-2 flex cursor-pointer gap-x-4 text-sm" for={transfer.id}
                            ><span>{transfer.progress}%</span><span>{transfer.status}</span><span
                                >{transfer.filename}</span
                            ></label>
                    </div>
                {/each}
            {:else}
                <p class="text-text ml-2 cursor-pointer text-sm">No Transfers Found.</p>
            {/if}
        </div>
    {/if}
    <div class="mx-auto flex w-fit items-center justify-center gap-x-5 py-5">
        <button
            disabled={transfers.length === 0}
            onclick={deleteTransfers}
            class="{transfers.length === 0
                ? ' cursor-default! text-slate-500'
                : 'text-textColor hover:text-primaryColor'} flex-1 rounded-lg bg-slate-800 px-6 py-3.5 font-bold shadow-lg transition-all">
            Delete Selected
        </button>
        <button
            onclick={() => {
                isTransferListOpen = false
            }}
            class="px-6 py-3.5 font-medium text-slate-400 transition-colors hover:text-slate-200">
            Cancel
        </button>
    </div>
</div>
