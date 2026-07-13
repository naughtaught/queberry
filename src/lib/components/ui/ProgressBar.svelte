<script lang="ts">
    import CancelIcon from 'virtual:icons/healthicons/cancel-24px'
    import { downloadsInProgress } from '$lib/stores/plugins'
    import { deleteTransfer } from '$lib/functions/plugins/deleteTransfer'
    import { createError, handleError } from '$lib/functions/errors/errorHandling'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { user } from '$lib/stores/user'

    const { progressData, type } = $props()

    const deleteId = $derived(type === 'transfer' ? progressData.info_hash : progressData.link?.uuid)
    const displayProgress = $derived(progressData?.progress ?? 0)
    const displaySpeed = $derived(progressData?.speed)
    const displayStatus = $derived(progressData?.status ?? 'queued')

    const handleDelete = async (): Promise<void> => {
        if (!deleteId) return

        if (type === 'transfer') {
            await deleteTransfer(deleteId)
        } else {
            await deleteDownload(deleteId)
        }
    }

    const formatSpeed = (speed: number): string => {
        if (speed < 1) {
            return `${speed.toFixed(2)}Mbps`
        } else if (speed < 10) {
            return `${speed.toFixed(1)}Mbps`
        } else if (speed >= 1000) {
            return `${(speed / 1000).toFixed(1)}Gbps`
        } else {
            return `${Math.round(speed)}Mbps`
        }
    }

    const deleteDownload = async (id: string): Promise<void> => {
        try {
            if (!$user) throw createError('Missing User', 401, { log: false })

            await invokeFunction('cancel_download', { uuid: id, userId: $user.id })
            downloadsInProgress.update((x) => x.filter((entry) => entry.link.uuid !== id))
        } catch (error) {
            handleError(error)
        }
    }
</script>

<div class="relative min-h-9.5 w-full items-center space-y-1 pt-1">
    <p class="flex items-center justify-between text-[10px] font-bold tracking-widest text-slate-500 uppercase">
        {type === 'transfer' ? 'Transferring' : 'Downloading'}
        <span>
            <button
                onclick={handleDelete}
                class="flex items-center hover:scale-105 hover:cursor-pointer"
                aria-label="Cancel {type}">
                <CancelIcon class="text-xs text-red-500" />
            </button>
        </span>
    </p>

    {#if progressData}
        <div
            class="relative flex w-full max-w-lg space-x-5 overflow-hidden rounded-full border border-black bg-slate-300">
            <div
                class="h-3 rounded-full {displayStatus === 'error' || displayStatus === 'failed'
                    ? 'bg-red-500'
                    : 'bg-primaryColor'}"
                style="width: {displayProgress.toFixed(2)}%;">
            </div>
            <div
                class="absolute top-1/2 left-1/2 flex -translate-x-1/2 -translate-y-1/2 transform items-center text-xs font-bold whitespace-nowrap text-black">
                <span class="mr-2 text-black">{displayProgress.toFixed(2)}%</span>
                {#if displaySpeed !== undefined && displayStatus === 'downloading'}
                    <span class="mr-2 whitespace-nowrap text-black">
                        {formatSpeed(displaySpeed)}
                    </span>
                {/if}
            </div>
        </div>
    {:else}
        <div
            class="relative flex w-full max-w-lg space-x-5 overflow-hidden rounded-full border border-black bg-slate-300">
            <div class="h-3 w-0 rounded-full bg-primaryColor"></div>
            <div
                class="absolute top-1/2 left-1/2 flex -translate-x-1/2 -translate-y-1/2 transform items-center text-xs font-bold whitespace-nowrap text-black">
                <span class="text-black">Initializing...</span>
            </div>
        </div>
    {/if}
</div>
