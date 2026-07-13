<script lang="ts">
    import BaseModal from '$lib/components/modals/BaseModal.svelte'
    import type { Api } from '$lib/types/api'
    import { getImagePath } from '$lib/functions/ui/getImagePath'
    import { posterSize } from '$lib/functions/ui/posterSize'
    import { loadingStates } from '$lib/stores/app'
    import { fetchMediaItem } from '$lib/db/fetchMediaItem'
    import { createError, handleError } from '$lib/functions/errors/errorHandling'
    import { loadVideo } from '$lib/functions/video/loadVideo'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'

    let { isCollectionModalOpen = $bindable(), unfilteredCollectionItems } = $props()

    const collectionItems: Api.CollectionItem[] = $derived(unfilteredCollectionItems)
    let collectionPlaylist: Api.CollectionItem[] = $derived([])

    const onCancel = (): void => {
        isCollectionModalOpen = false
    }

    const handleClick = (item: Api.CollectionItem): void => {
        if (!collectionPlaylist.includes(item)) {
            collectionPlaylist = [...collectionPlaylist, item]
        } else {
            collectionPlaylist = collectionPlaylist.filter((x) => x !== item)
        }
    }

    const playPlaylist = async (): Promise<void> => {
        try {
            $loadingStates.isPlayButtonLoading = true
            const mediaItemResp = await fetchMediaItem(collectionPlaylist[0].media_id, collectionPlaylist[0].type)

            if (!mediaItemResp.success) {
                if (mediaItemResp.error.message.toLowerCase().includes('no data')) {
                    const newType = collectionPlaylist[0].type === 'movie' ? 'tv' : 'movie'

                    const resp = await fetchMediaItem(collectionPlaylist[0].media_id, newType)
                    if (!resp.success) throw resp.error
                    if (!resp.data)
                        throw createError('Missing media data', 500, {
                            log: false,
                        })

                    loadVideo(resp.data, null, null, null, false, collectionPlaylist)
                } else {
                    throw mediaItemResp.error
                }
            } else {
                if (!mediaItemResp.data)
                    throw createError('Missing media data', 500, {
                        log: false,
                    })

                loadVideo(mediaItemResp.data, null, null, null, false, collectionPlaylist)
            }
        } catch (error) {
            $loadingStates.isPlayButtonLoading = false
            handleError(error)
        }
    }

    const findPlaylistIndex = (item: Api.CollectionItem): number => {
        return collectionPlaylist.findIndex((x) => x === item) + 1
    }
</script>

<BaseModal onClose={onCancel}>
    <div class="relative mt-2 flex w-225 flex-col rounded">
        <div class="text-text sticky top-0 py-2 text-center font-medium">
            <h1>Create Playlist</h1>
        </div>
        <div class="mb-5 flex w-full justify-between px-5 text-sm font-medium text-textColor">
            <div class="flex items-center">
                {#if collectionItems.length > 0}
                    {#if collectionItems.length !== collectionPlaylist.length}
                        <button onclick={() => (collectionPlaylist = collectionItems)}>Select all</button>
                    {:else}
                        <button onclick={() => (collectionPlaylist = [])}>Deselect all</button>
                    {/if}
                {/if}
            </div>
        </div>
        <div class="mt-5 flex max-h-[70vh] min-h-48 flex-col space-y-4 overflow-auto px-5">
            {#each collectionItems as item (item.media_id)}
                <div class="relative inline-flex items-center">
                    {#if collectionPlaylist.includes(item)}
                        <button class="absolute top-1 left-3.5">{findPlaylistIndex(item)}</button>
                    {/if}
                    <button
                        type="button"
                        class="flex w-full items-center p-2 text-left text-sm font-semibold tracking-wider text-slate-300 uppercase hover:text-primaryColor"
                        onclick={() => handleClick(item)}>
                        <div class="relative mr-2 flex items-center">
                            {#if collectionPlaylist.includes(item)}
                                <div
                                    class="flex h-5 w-5 items-center justify-center rounded border border-slate-700 bg-primaryColor text-black shadow">
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
                                </div>
                            {:else}
                                <div class="h-5 w-5 rounded border border-gray-400 shadow"></div>
                            {/if}
                        </div>
                        <div class="flex space-x-4">
                            <div class="h-36 w-24 shrink-0 overflow-hidden rounded-md shadow-lg">
                                <img
                                    alt="Search Result Poster"
                                    class="h-full w-full object-cover"
                                    src={getImagePath(item?.poster, posterSize(1, 'carousel'))
                                        ? getImagePath(item?.poster, posterSize(1, 'carousel'))
                                        : '/images/poster-placeholder.png'} />
                            </div>
                            <div class="flex flex-col">
                                <p>{item.title} ({item.released})</p>
                                <div class="min-h-0 flex-1 space-y-4">
                                    <p class="text-xs text-slate-500">{item.type}</p>
                                    <p class="text-xs text-slate-300">{item.blurb}</p>
                                </div>
                            </div>
                        </div>
                    </button>
                </div>
            {/each}
        </div>
        <div class="mx-auto flex w-fit items-center justify-center gap-x-5 py-5">
            <button
                disabled={collectionPlaylist.length === 0 || $loadingStates.isPlayButtonLoading}
                onclick={playPlaylist}
                class="{collectionPlaylist.length === 0
                    ? ' cursor-default! text-slate-500'
                    : 'text-textColor hover:text-primaryColor'} flex-1 rounded-lg bg-slate-800 px-6 py-3.5 font-bold shadow-lg transition-all">
                {#if $loadingStates.isPlayButtonLoading}<LoadingSpinner />{:else}Play Selected{/if}
            </button>
            <button
                onclick={() => {
                    isCollectionModalOpen = false
                }}
                class="px-6 py-3.5 font-medium text-slate-400 transition-colors hover:text-slate-200">
                Cancel
            </button>
        </div>
    </div>
</BaseModal>
