<script lang="ts">
    import { setVideoTitle } from '$lib/functions/video/setVideoTitle'
    import DragBar from '$lib/components/bars/DragBar.svelte'
    import { shouldCancelVideoLoad, videoMetadata } from '$lib/stores/video'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import ReturnIcon from 'virtual:icons/famicons/return-down-back-sharp'

    const title = $derived.by(() => {
        const tempTitle = setVideoTitle()
        return tempTitle.split('|')[0].trim()
    })
</script>

<DragBar />

{#if $videoMetadata.backdrop === '-1'}
    <div class="fixed inset-0 z-40 flex h-screen w-screen bg-backgroundColor"></div>
    <div class="fixed top-0 left-0 z-40 flex w-full justify-center">
        <h1 class="font-outline mt-4 text-xl text-white shadow-2xl">{title}</h1>
    </div>
{:else if $videoMetadata.backdrop}
    <img
        class="fixed inset-0 z-40 flex h-full w-full bg-backgroundColor brightness-50"
        src={$videoMetadata.backdrop}
        alt="Loading Video Backdrop" />
{/if}

<button
    onclick={() => {
        $shouldCancelVideoLoad = true
    }}
    class="fixed top-10 left-5 z-50 flex">
    <ReturnIcon class="font-outline max-w-16 text-white hover:cursor-pointer hover:text-primaryColor" />
</button>

<div class="fixed inset-0 z-40 flex items-center justify-center">
    <LoadingSpinner size="max-w-8" />
</div>
