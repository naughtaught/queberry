<script lang="ts">
    import BaseModal from '$lib/components/modals/BaseModal.svelte'
    import { startTransfer } from '$lib/functions/video/startTransfer'
    import { modals } from '$lib/stores/app'
    import TransferListModal from '$lib/components/modals/TransferListModal.svelte'

    const { selectedSource } = $props()

    let isTransferListOpen = $state(false)

    const onConfirm = (): void => {
        startTransfer(selectedSource)
        $modals.transfer = false
    }

    const handleKeypress = (e: KeyboardEvent): void => {
        if ($modals.transfer) {
            if (e.key === 'Escape') onCancel()
            if (e.key === 'Enter') onConfirm()
        }
    }

    const onCancel = (): void => {
        $modals.transfer = false
    }
</script>

<svelte:window onkeydown={handleKeypress} />

<BaseModal onClose={onCancel}>
    {#if !isTransferListOpen}
        <div class="w-full transform rounded-xl p-6" tabindex="-1">
            <h2 id="modal-title" class="mb-4 text-xl font-semibold text-textColor">Transfer</h2>
            <p class="mb-6 text-textColor">Start Transfer?</p>
            <div class="flex justify-end space-x-4">
                <button
                    onclick={onConfirm}
                    class="flex-1 rounded-lg bg-slate-800 px-6 py-3.5 font-bold text-textColor shadow-lg transition-all hover:text-primaryColor">
                    Confirm
                </button>

                <button
                    onclick={() => (isTransferListOpen = true)}
                    class="w-72 flex-1 rounded-lg bg-slate-800 px-6 py-3.5 font-bold text-textColor shadow-lg transition-all hover:text-primaryColor">
                    Current Transfers
                </button>
                <button
                    onclick={onCancel}
                    class="px-6 py-3.5 font-medium text-slate-400 transition-colors hover:text-slate-200">
                    Cancel
                </button>
            </div>
        </div>
    {:else}
        <TransferListModal bind:isTransferListOpen />
    {/if}
</BaseModal>
