<script lang="ts">
    import { handleError, invokeFunction } from '$lib'

    const { change, label, emit } = $props()

    const emitEvent = async (value: boolean): Promise<void> => {
        const response = await invokeFunction(emit, {
            value,
        })
        if (!response.success) handleError(response.error!)
    }
</script>

{#if change}
    <button
        aria-label={label}
        class="flex items-center justify-center rounded-md bg-gray-100 px-3 py-1 text-sm text-gray-600 transition-colors duration-200 hover:cursor-pointer hover:bg-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700"
        onclick={() => {
            emitEvent(true)
        }}>
        <svg height="18px" viewBox="0 -960 960 960" width="18px" fill="currentColor">
            <path d="M440-120v-320H120v-80h320v-320h80v320h320v80H520v320h-80Z" />
        </svg>
    </button>
{:else if !change}
    <button
        aria-label={label}
        class="flex items-center justify-center rounded-md bg-gray-100 px-3 py-1 text-sm text-gray-600 transition-colors duration-200 hover:cursor-pointer hover:bg-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700"
        onclick={() => {
            emitEvent(false)
        }}>
        <svg height="18px" viewBox="0 -960 960 960" width="18px" fill="currentColor">
            <path d="M200-440v-80h560v80H200Z" />
        </svg>
    </button>
{/if}
