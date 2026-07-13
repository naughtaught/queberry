<script lang="ts">
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { onMount } from 'svelte'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import BaseModal from '$lib/components/modals/BaseModal.svelte'
    import type { Sql } from '$lib/types/sql'
    import Checkbox from '$lib/components/inputs/Checkbox.svelte'

    let { isMediaManagerOpen = $bindable() } = $props()

    let loading = $state(true)
    let isSaving = $state(false)
    let localMedia: Sql.EditableMediaFields[] = $state([])

    let originalMedia = $state<Sql.EditableMediaFields[]>([])
    let groupedMedia = $state<Record<string, Record<string, Sql.EditableMediaFields[]>>>({})
    let expandedFolders = $state<Record<string, boolean>>({})
    let expandedShows = $state<Record<string, boolean>>({})

    let hasChanges = $derived(
        originalMedia.length > 0 &&
            localMedia.length > 0 &&
            JSON.stringify(localMedia) !== JSON.stringify(originalMedia),
    )

    const toggleFolder = (folder: string): void => {
        expandedFolders[folder] = !expandedFolders[folder]
        expandedFolders = { ...expandedFolders }
    }

    const toggleShow = (showKey: string): void => {
        expandedShows[showKey] = !expandedShows[showKey]
        expandedShows = { ...expandedShows }
    }

    const updateLocalMedia = (index: number, updates: Partial<Sql.EditableMediaFields>): void => {
        if (index !== -1) {
            const savedExpandedFolders = { ...expandedFolders }
            const savedExpandedShows = { ...expandedShows }

            localMedia[index] = { ...localMedia[index], ...updates }
            localMedia = [...localMedia]
            groupedMedia = groupMedia(localMedia)

            expandedFolders = savedExpandedFolders
            expandedShows = savedExpandedShows
        }
    }

    const groupMedia = (
        media: Sql.EditableMediaFields[],
    ): Record<string, Record<string, Sql.EditableMediaFields[]>> => {
        const grouped: Record<string, Record<string, Sql.EditableMediaFields[]>> = {}

        for (const item of media) {
            const parts = item.filePath.split('\\')
            const mainFolderIndex = parts.indexOf('Media') + 1
            const mainFolder = parts[mainFolderIndex] || 'Other'
            const showFolder = parts[mainFolderIndex + 1] || 'Unknown'

            if (!grouped[mainFolder]) {
                grouped[mainFolder] = {}
                expandedFolders[mainFolder] = true
            }

            if (!grouped[mainFolder][showFolder]) {
                grouped[mainFolder][showFolder] = []
                expandedShows[`${mainFolder}/${showFolder}`] = false
            }

            grouped[mainFolder][showFolder].push(item)
        }

        return grouped
    }

    const updateMedia = async (): Promise<void> => {
        isSaving = true
        try {
            const response = await invokeFunction('update_local_media', {
                media: localMedia,
            })

            if (!response.success) throw response.error

            originalMedia = JSON.parse(JSON.stringify(localMedia))
            hasChanges = false
        } catch (error) {
            handleError(error)
        } finally {
            isSaving = false
        }
    }

    const fetchLocalMedia = async (): Promise<void> => {
        loading = true

        try {
            const response = await invokeFunction('get_editable_local_media', {})

            if (!response.success) throw response.error

            const sorted = response.data.sort((a: Sql.EditableMediaFields, b: Sql.EditableMediaFields) => {
                const partsA = a.filePath.split('\\')
                const partsB = b.filePath.split('\\')

                const maxLength = Math.max(partsA.length, partsB.length)

                for (let i = 0; i < maxLength; i++) {
                    if (!partsA[i] && partsB[i]) return -1
                    if (partsA[i] && !partsB[i]) return 1
                    if (!partsA[i] && !partsB[i]) return 0

                    const numA = Number(partsA[i])
                    const numB = Number(partsB[i])

                    let comparison: number
                    if (!isNaN(numA) && !isNaN(numB)) {
                        comparison = numA - numB
                    } else {
                        comparison = partsA[i].localeCompare(partsB[i], undefined, {
                            numeric: true,
                            sensitivity: 'base',
                        })
                    }

                    if (comparison !== 0) return comparison
                }

                return 0
            })

            localMedia = sorted
            originalMedia = JSON.parse(JSON.stringify(sorted))
            groupedMedia = groupMedia(sorted)
        } catch (error) {
            handleError(error)
        } finally {
            loading = false
        }
    }

    onMount(async () => {
        await fetchLocalMedia()
    })

    const handleKeypress = (e: KeyboardEvent): void => {
        if (isMediaManagerOpen) if (e.key === 'Escape') onCancel()
    }

    const onCancel = (): void => {
        isMediaManagerOpen = false
    }

    const resetChanges = (): void => {
        localMedia = originalMedia
        groupedMedia = groupMedia(localMedia)
        hasChanges = false
    }
</script>

<svelte:window onkeydown={handleKeypress} />

<BaseModal onClose={onCancel}>
    <div class="relative mt-2 flex min-w-[80vw] flex-col rounded">
        <div class="w-full transform rounded-xl p-6" tabindex="-1">
            <div class="mb-4 flex items-center justify-between">
                <h2 id="modal-title" class="text-xl font-semibold text-textColor">Local Media Manager</h2>
                <div class="flex items-center gap-2 pr-7.5">
                    {#if hasChanges}
                        <span class="text-sm text-yellow-400">Unsaved changes</span>
                    {/if}
                    <button
                        class="rounded bg-slate-600 px-4 py-1 text-textColor disabled:cursor-default disabled:opacity-50"
                        onclick={resetChanges}
                        disabled={!hasChanges || isSaving}>
                        Reset
                    </button>
                    <button
                        class="rounded bg-slate-600 px-4 py-1 text-textColor hover:bg-slate-500 disabled:cursor-default disabled:opacity-50 disabled:hover:bg-slate-600"
                        onclick={onCancel}
                        disabled={isSaving}>
                        Cancel
                    </button>
                    <button
                        class="rounded bg-slate-600 px-4 py-1 text-textColor hover:bg-slate-500 hover:text-primaryColor disabled:cursor-default disabled:opacity-50 disabled:hover:bg-slate-600 disabled:hover:text-textColor"
                        onclick={updateMedia}
                        disabled={!hasChanges || isSaving}>
                        {isSaving ? 'Saving...' : 'Save All Changes'}
                    </button>
                </div>
            </div>

            {#if loading}
                <div class="mt-5 flex min-h-48 items-center justify-center">
                    <LoadingSpinner />
                </div>
            {:else}
                <div class="mt-5 flex max-h-[calc(80vh-6rem)] min-h-48 flex-col space-y-4 overflow-auto px-5">
                    {#if localMedia.length > 0}
                        <div class="">
                            {#each Object.entries(groupedMedia) as [mainFolder, shows] (mainFolder)}
                                <div class="mb-4">
                                    <button
                                        class="flex w-full items-center rounded bg-slate-700 p-2 text-left font-semibold text-textColor hover:bg-slate-600"
                                        onclick={() => toggleFolder(mainFolder)}>
                                        <span class="mr-2">{expandedFolders[mainFolder] ? '▼' : '▶'}</span>
                                        {mainFolder} ({Object.keys(shows).length} folders)
                                    </button>

                                    {#if expandedFolders[mainFolder]}
                                        <div class="mt-2 ml-4">
                                            {#each Object.entries(shows) as [show, episodes] (show)}
                                                <div class="mb-2">
                                                    <div
                                                        class="flex items-center rounded bg-slate-800 hover:bg-slate-700">
                                                        <button
                                                            class="flex flex-1 items-center p-2 text-left text-textColor"
                                                            onclick={() => toggleShow(`${mainFolder}/${show}`)}>
                                                            <span class="mr-2"
                                                                >{expandedShows[`${mainFolder}/${show}`]
                                                                    ? '▼'
                                                                    : '▶'}</span>
                                                            {show}
                                                            <span class="ml-2 text-sm text-slate-400"
                                                                >({episodes.length} files)</span>
                                                        </button>
                                                        <div class="flex items-center px-2">
                                                            <span class="text-sm text-slate-400">IMDb:</span>
                                                            <input
                                                                type="text"
                                                                class="ml-2 h-6 w-24 rounded bg-slate-600 px-2 text-sm text-textColor"
                                                                value={episodes[0]?.imdbId || ''}
                                                                oninput={(e) => {
                                                                    for (const episode of episodes) {
                                                                        const index = localMedia.findIndex(
                                                                            (m) => m.filepathId === episode.filepathId,
                                                                        )
                                                                        updateLocalMedia(index, {
                                                                            imdbId: e.currentTarget.value,
                                                                        })
                                                                    }
                                                                }} />
                                                        </div>
                                                    </div>

                                                    {#if expandedShows[`${mainFolder}/${show}`]}
                                                        <div class="mt-1 ml-4">
                                                            <table class="w-full border-collapse">
                                                                <thead
                                                                    class="sticky top-0 z-10 bg-backgroundColor text-slate-300">
                                                                    <tr>
                                                                        <th class="w-14 p-2 text-center">Default</th>
                                                                        <th class="w-20 p-2 text-center">Season</th>
                                                                        <th class="w-20 p-2 text-center">Episode</th>
                                                                        <th class="p-2 text-left">File Path</th>
                                                                    </tr>
                                                                </thead>
                                                                <tbody>
                                                                    {#each episodes as media (media.filepathId)}
                                                                        <tr
                                                                            class="h-16.5 border-b border-slate-500 text-textColor">
                                                                            <td class="p-2">
                                                                                <div
                                                                                    class="flex items-center justify-center">
                                                                                    <Checkbox
                                                                                        checked={media.isDefault}
                                                                                        label=""
                                                                                        func={() => {
                                                                                            const index =
                                                                                                localMedia.findIndex(
                                                                                                    (m) =>
                                                                                                        m.filepathId ===
                                                                                                        media.filepathId,
                                                                                                )
                                                                                            if (index !== -1) {
                                                                                                const savedExpandedFolders =
                                                                                                    {
                                                                                                        ...expandedFolders,
                                                                                                    }
                                                                                                const savedExpandedShows =
                                                                                                    { ...expandedShows }

                                                                                                localMedia[index] = {
                                                                                                    ...localMedia[
                                                                                                        index
                                                                                                    ],
                                                                                                    isDefault:
                                                                                                        !media.isDefault,
                                                                                                }
                                                                                                localMedia = [
                                                                                                    ...localMedia,
                                                                                                ]
                                                                                                groupedMedia =
                                                                                                    groupMedia(
                                                                                                        localMedia,
                                                                                                    )

                                                                                                expandedFolders =
                                                                                                    savedExpandedFolders
                                                                                                expandedShows =
                                                                                                    savedExpandedShows
                                                                                            }
                                                                                        }} />
                                                                                </div>
                                                                            </td>
                                                                            <td class="p-2">
                                                                                <input
                                                                                    type="number"
                                                                                    class="h-6 w-16 rounded bg-slate-600 px-2 text-center text-sm text-textColor"
                                                                                    value={media.season || ''}
                                                                                    oninput={(e) => {
                                                                                        const index =
                                                                                            localMedia.findIndex(
                                                                                                (m) =>
                                                                                                    m.filepathId ===
                                                                                                    media.filepathId,
                                                                                            )
                                                                                        updateLocalMedia(index, {
                                                                                            season: e.currentTarget
                                                                                                .value
                                                                                                ? Number(
                                                                                                      e.currentTarget
                                                                                                          .value,
                                                                                                  )
                                                                                                : null,
                                                                                        })
                                                                                    }} />
                                                                            </td>
                                                                            <td class="p-2">
                                                                                <input
                                                                                    type="number"
                                                                                    class="h-6 w-16 rounded bg-slate-600 px-2 text-center text-sm text-textColor"
                                                                                    value={media.episode || ''}
                                                                                    oninput={(e) => {
                                                                                        const index =
                                                                                            localMedia.findIndex(
                                                                                                (m) =>
                                                                                                    m.filepathId ===
                                                                                                    media.filepathId,
                                                                                            )
                                                                                        updateLocalMedia(index, {
                                                                                            episode: e.currentTarget
                                                                                                .value
                                                                                                ? Number(
                                                                                                      e.currentTarget
                                                                                                          .value,
                                                                                                  )
                                                                                                : null,
                                                                                        })
                                                                                    }} />
                                                                            </td>
                                                                            <td
                                                                                class="max-w-xs overflow-auto p-2 text-sm"
                                                                                >{media.filePath.split('\\').pop()}</td>
                                                                        </tr>
                                                                    {/each}
                                                                </tbody>
                                                            </table>
                                                        </div>
                                                    {/if}
                                                </div>
                                            {/each}
                                        </div>
                                    {/if}
                                </div>
                            {/each}
                        </div>
                    {:else}
                        <p class="text-text ml-2 cursor-pointer text-sm">No Local Media Found.</p>
                    {/if}
                </div>
            {/if}
        </div>
    </div>
</BaseModal>
