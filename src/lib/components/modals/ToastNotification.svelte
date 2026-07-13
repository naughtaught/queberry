<script lang="ts">
    import { TOAST_DURATION, toastNotification } from '$lib/stores/app'
    import { onDestroy } from 'svelte'
    import { fade } from 'svelte/transition'
    import CloseIcon from 'virtual:icons/material-symbols/close'

    let progress = $state(100)
    let intervalId: ReturnType<typeof setInterval> | null = null
    let unsubscribe: (() => void) | null = null

    const getTypeClasses = (type: string | null): { bg: string; bar: string; text: string } => {
        const lowerType = type?.toLowerCase()
        switch (lowerType) {
            case 'error':
                return { bg: 'bg-red-200', bar: 'bg-red-800', text: 'text-slate-900' }
            case 'success':
                return { bg: 'bg-green-200', bar: 'bg-green-800', text: 'text-slate-900' }
            case 'warning':
                return { bg: 'bg-yellow-200', bar: 'bg-yellow-800', text: 'text-slate-900' }
            default:
                return { bg: 'bg-gray-200', bar: 'bg-gray-800', text: 'text-slate-900' }
        }
    }

    unsubscribe = toastNotification.subscribe((notification) => {
        if (notification.message) {
            progress = 100

            if (intervalId) clearInterval(intervalId)

            const UPDATE_INTERVAL = 30
            const DECREMENT = (100 / TOAST_DURATION) * UPDATE_INTERVAL

            intervalId = setInterval(() => {
                progress = Math.max(0, progress - DECREMENT)

                if (progress <= 0) {
                    if (intervalId) {
                        clearInterval(intervalId)
                        intervalId = null
                    }
                }
            }, UPDATE_INTERVAL)
        } else {
            if (intervalId) {
                clearInterval(intervalId)
                intervalId = null
            }
        }
    })

    onDestroy(() => {
        if (intervalId) clearInterval(intervalId)
        if (unsubscribe) unsubscribe()
    })
</script>

{#if $toastNotification.message}
    {@const typeClasses = getTypeClasses($toastNotification.type)}

    <div
        class="fixed bottom-5 left-1/2 z-50 max-w-200 min-w-80 -translate-x-1/2 transform overflow-hidden rounded-lg shadow-lg {typeClasses.bg}"
        transition:fade={{ duration: 100 }}>
        <button
            class="absolute top-2 right-2 z-10 rounded-full p-1 transition-colors hover:bg-black/10"
            onclick={() => toastNotification.hide()}
            aria-label="Close notification">
            <CloseIcon class="h-4 w-4 {typeClasses.text}" />
        </button>

        <div class="p-4 pr-8">
            {#if $toastNotification.title}
                <p class="font-bold {typeClasses.text}">{$toastNotification.title}</p>
            {/if}
            <p class={typeClasses.text}>{$toastNotification.message}</p>
        </div>

        <div class="h-1 transition-all duration-30 {typeClasses.bar}" style="width: {progress}%"></div>
    </div>
{/if}
