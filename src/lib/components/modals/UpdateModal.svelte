<script lang="ts">
    import BaseModal from '$lib/components/modals/BaseModal.svelte'
    import { appData } from '$lib/stores/app'
    import Markdown from 'svelte-markdown'

    const onConfirm = (): void => {
        if ($appData.updateResolver) $appData.updateResolver(true)
    }

    const handleKeypress = (e: KeyboardEvent): void => {
        if ($appData.showUpdateModal) {
            if (e.key === 'Escape') onCancel()
            if (e.key === 'Enter') onConfirm()
        }
    }

    const onCancel = (): void => {
        if ($appData.updateResolver) $appData.updateResolver(false)
        $appData.showUpdateModal = false
    }
</script>

<svelte:window onkeydown={handleKeypress} />

<BaseModal onClose={onCancel}>
    <div class="w-full max-w-md min-w-96 transform rounded-lg p-6" tabindex="-1">
        {#if $appData.isDownloading}
            <h2 class="mb-4 text-xl font-semibold">Downloading Update...</h2>
            <div class="mb-4">
                <div class="h-2 w-full rounded-full bg-gray-700">
                    <div
                        class="h-2 rounded-full bg-primaryColor transition-all duration-300"
                        style="width: {$appData.downloadProgress || 0}%">
                    </div>
                </div>
                <p class="mt-2 text-center text-sm text-gray-400">
                    {Math.round($appData.downloadProgress || 0)}%
                </p>
            </div>
        {:else if $appData.isInstalling}
            <h2 class="mb-4 text-xl font-semibold">Installing Update...</h2>
            <p class="mb-6">The application will restart now.</p>
        {:else}
            <h2 id="modal-title" class="mb-12 text-3xl font-semibold">Update Available</h2>
            {#if $appData.pendingUpdate?.notes}
                <p class="prose max-w-none prose-invert">
                    <Markdown source={$appData.pendingUpdate.notes} />
                </p>
            {/if}
            <div class="mt-12 flex justify-end space-x-4">
                <button
                    onclick={onCancel}
                    class="px-6 py-3.5 font-medium text-slate-400 transition-colors hover:text-slate-200">
                    Cancel
                </button>
                <button
                    onclick={onConfirm}
                    class="max-w-96 flex-1 rounded-lg bg-slate-800 px-6 py-3.5 font-bold text-white shadow-lg transition-all hover:text-primaryColor">
                    Confirm
                </button>
            </div>
        {/if}
    </div>
</BaseModal>
