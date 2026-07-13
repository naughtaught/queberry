<script lang="ts">
    import { resolve } from '$app/paths'
    import { fetchMediaItem } from '$lib/db/fetchMediaItem'
    import { createError, handleError } from '$lib/functions/errors/errorHandling'
    import { getImagePath } from '$lib/functions/ui/getImagePath'
    import { loadVideo } from '$lib/functions/video/loadVideo'
    import { loadingStates } from '$lib/stores/app'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'

    const { related } = $props()

    let currentIndex = $derived(related[0])
    const backdropPath = $derived(getImagePath(currentIndex.backdrop, 'original'))

    const playItem = async (): Promise<void> => {
        try {
            $loadingStates.isPlayButtonLoading = true
            const mediaItemResp = await fetchMediaItem(currentIndex.media_id, currentIndex.type)

            if (!mediaItemResp.success) {
                if (mediaItemResp.error.message.toLowerCase().includes('no data')) {
                    const newType = currentIndex.type === 'movie' ? 'tv' : 'movie'

                    const resp = await fetchMediaItem(currentIndex.media_id, newType)
                    if (!resp.success) throw resp.error
                    if (!resp.data)
                        throw createError('Missing media data', 500, {
                            log: false,
                        })

                    loadVideo(resp.data, null, null, null, false)
                } else {
                    throw mediaItemResp.error
                }
            } else {
                if (!mediaItemResp.data)
                    throw createError('Missing media data', 500, {
                        log: false,
                    })

                loadVideo(mediaItemResp.data, null, null, null, false)
            }
        } catch (error) {
            $loadingStates.isPlayButtonLoading = false
            handleError(error)
        }
    }
</script>

<section class="flex w-full">
    <div
        class="relative ml-2 h-84 w-full overflow-hidden rounded bg-cover bg-top pt-2 pl-4"
        data-alt="Backdrop Image of {currentIndex.title}"
        style="background-image: url({backdropPath})">
        <div class="flex h-full flex-col">
            <div>
                <h2
                    class="font-outline line-clamp-2 max-w-1/2 text-3xl leading-tight font-black tracking-tighter text-textColor">
                    {currentIndex.title} ({currentIndex.released})
                </h2>
                <p class="font-outline flex items-center text-xs font-medium text-slate-300 uppercase">
                    {currentIndex.type}
                </p>
                <div class="mt-5 space-y-1">
                    <h3 class="font-outline text-[10px] font-bold tracking-[0.2em] text-slate-300 uppercase">
                        The Story
                    </h3>
                    <p class="font-outline line-clamp-4 w-1/3 text-sm leading-relaxed font-normal text-slate-100">
                        {currentIndex.blurb}
                    </p>
                </div>
            </div>

            <div class="mt-auto flex w-1/3 space-x-8 pb-4">
                <button
                    disabled={$loadingStates.isPlayButtonLoading}
                    onclick={playItem}
                    class="flex min-w-17 flex-1 items-center justify-center gap-2 rounded-xl bg-slate-800/60 py-1 font-bold shadow-lg shadow-backgroundColor hover:text-primaryColor">
                    {#if $loadingStates.isPlayButtonLoading}<LoadingSpinner />{:else}Play{/if}
                </button>
                <a
                    data-sveltekit-preload-data="tap"
                    class="flex flex-1 items-center justify-center gap-2 rounded-xl bg-slate-800/60 px-2 py-1 font-bold shadow-lg shadow-backgroundColor hover:text-primaryColor"
                    href={resolve(`/details/?id=${currentIndex.media_id}&type=${currentIndex.type}`, {})}>
                    Details
                </a>
            </div>
        </div>

        <div class="absolute right-0 bottom-6 flex h-70 w-1/2 gap-x-3 overflow-x-auto pr-2">
            {#each related as media (media.media_id)}
                <button
                    onclick={() => (currentIndex = media)}
                    class="aspect-2/3 h-full rounded pb-1 shadow-2xl shadow-backgroundColor">
                    <img
                        loading="lazy"
                        class="h-full rounded border-2 border-black object-cover"
                        tabindex="-1"
                        src={getImagePath(media.poster, 'w342')}
                        alt={currentIndex?.title ?? 'Media poster'} />
                </button>
            {/each}
        </div>
    </div>
</section>
