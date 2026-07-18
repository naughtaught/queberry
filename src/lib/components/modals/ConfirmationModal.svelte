<script lang="ts">
    import BaseModal from '$lib/components/modals/BaseModal.svelte'

    let { config = $bindable() } = $props()

    const onConfirm = (): void => {
        config.action()
        config.isOpen = false
    }

    const handleKeypress = (e: KeyboardEvent): void => {
        if (config.isOpen) {
            if (e.key === 'Escape') onCancel()
            if (e.key === 'Enter') onConfirm()
        }
    }

    const onCancel = (): void => {
        config.isOpen = false
    }
</script>

<svelte:window onkeydown={handleKeypress} />

{#if config.isOpen}
    <BaseModal onClose={onCancel}>
        <div class="w-full max-w-md transform rounded-lg p-6" tabindex="-1">
            <h2 id="modal-title" class="mb-4 text-xl font-semibold">
                {config.title}
            </h2>
            <p class="mb-6 whitespace-pre-line">
                {config.message}
            </p>

            <div class="flex justify-end space-x-4">
                <button
                    onclick={onCancel}
                    class="px-6 py-3.5 font-medium text-slate-400 transition-colors hover:text-slate-200">
                    Cancel
                </button>
                <button
                    onclick={onConfirm}
                    class="flex-1 rounded-lg bg-slate-800 px-6 py-3.5 font-bold text-white shadow-lg transition-all hover:text-primaryColor">
                    Confirm
                </button>
            </div>
        </div>
    </BaseModal>
{/if}
