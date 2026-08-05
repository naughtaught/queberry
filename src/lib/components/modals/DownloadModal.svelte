<script lang="ts">
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import BaseModal from '$lib/components/modals/BaseModal.svelte'
    import { directories, modals } from '$lib/stores/app'
    import { onMount } from 'svelte'
    import { createError, handleError } from '$lib/functions/errors/errorHandling'
    import { fetchVideoUrl } from '$lib/functions/video/fetchVideoUrl'
    import type { Video } from '$lib/types/video'
    import { parseFilenameForEpisode } from '$lib/functions/utility/parseFilenameForEpisode'
    import CopyIcon from 'virtual:icons/clarity/copy-solid'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import type { Sql } from '$lib/types/sql'
    import type { Plugins } from '$lib/types/plugins'
    import { open } from '@tauri-apps/plugin-dialog'
    import { downloadsInProgress } from '$lib/stores/plugins'
    import { user } from '$lib/stores/user'
    import { getSeasonPoster } from '$lib/functions/utility/getSeasonPoster'
    import { extractSeasonAndEpisode } from '$lib/functions/utility/extractSeasonAndEpisode'
    import { getImagePath } from '$lib/functions/ui/getImagePath'
    import { unrestrictLink } from '$lib/functions/video/unrestrictLink'
    import Checkbox from '../inputs/Checkbox.svelte'

    const { media, source, seasonNumber, episodeData } = $props()

    let loading = $state(true)
    let files: Video.Files[] = $state([])
    let filesToDownload: Video.Files[] = $state([])
    let resolver: Plugins.Plugin | null = $state(null)
    let downloadLinks: Sql.DownloadMetaData[] = $state([])
    let downloadDirectory = $derived(media.type === 'tv' ? $directories.tv : $directories.movies)

    const handleKeypress = (e: KeyboardEvent): void => {
        if ($modals.download) {
            if (e.key === 'Escape') onCancel()
            if (e.key === 'Enter') onConfirm()
        }
    }

    const onConfirm = (): void => {
        if (filesToDownload.length === 0) {
            onCancel()
            return
        }
        downloadFiles()
    }

    const addToDownload = (file: Video.Files): void => {
        if (!filesToDownload.some((f) => f.filename === file.filename)) {
            filesToDownload = [...filesToDownload, file]
        }
    }

    const removeFromDownload = (file: Video.Files): void => {
        filesToDownload = filesToDownload.filter((f) => f.filename !== file.filename)
    }

    const toggleFile = (file: Video.Files, checked: boolean): void => {
        if (checked) {
            addToDownload(file)
        } else {
            removeFromDownload(file)
        }
    }

    const onCancel = (): void => {
        loading = false
        $modals.download = false
    }

    const selectAll = (): void => {
        const currentFilenames = new Set(filesToDownload.map((f) => f.filename))
        const newFiles = files.filter((file) => !currentFilenames.has(file.filename))
        filesToDownload = [...filesToDownload, ...newFiles]
    }

    onMount(async () => {
        const originalSeasonNumber = episodeData?.original_season_num ? episodeData.original_season_num : seasonNumber
        const originalEpisodeNumber = episodeData?.original_episode_num
            ? episodeData.original_episode_num
            : episodeData?.episode_num

        try {
            const response = await fetchVideoUrl(source, originalSeasonNumber, originalEpisodeNumber)

            if (!response.success) throw response.error

            files = response.data.files
            resolver = response.data.resolver

            if (media.type === 'tv') {
                files.sort((a, b) => {
                    const episodeA = extractSeasonAndEpisode(a.filename)
                    const episodeB = extractSeasonAndEpisode(b.filename)

                    if (!episodeA && !episodeB) return 0
                    if (!episodeA) return 1
                    if (!episodeB) return -1

                    if (episodeA.season !== episodeB.season) {
                        return episodeA.season - episodeB.season
                    }
                    return episodeA.episode - episodeB.episode
                })
            } else {
                files.sort((a, b) => b.size - a.size)
            }

            if (originalSeasonNumber && originalEpisodeNumber) {
                const file = parseFilenameForEpisode(originalSeasonNumber, originalEpisodeNumber, files)
                if (file) addToDownload(file)
            } else {
                files.forEach((file) => {
                    addToDownload(file)
                })
            }
        } catch (error) {
            handleError(error)
        } finally {
            loading = false
        }
    })

    $effect(() => {
        if (filesToDownload.length === 0) downloadLinks = []
    })

    const addDownloadLinks = async (): Promise<void> => {
        const DELAY_MS = 500

        for (let i = 0; i < filesToDownload.length; i++) {
            try {
                const file = filesToDownload[i]

                if (!resolver) return

                const existingFilenames = downloadLinks.map((x) => x.fileUrl)
                if (existingFilenames.includes(file.filename)) continue

                const downloadLink = await unrestrictLink(resolver.apikey, file.link, resolver)

                if (downloadLink) {
                    const extracted = extractSeasonAndEpisode(file.filename)

                    if (!$user) throw createError('Missing User', 401, { log: false })

                    downloadLinks.push({
                        folderPath: downloadDirectory,
                        filename: file.filename,
                        fileLink: file.link,
                        imdbId: media.imdb_id,
                        title: media.title,
                        released: media.released,
                        season: extracted?.season ?? null,
                        episode: extracted?.episode ?? null,
                        fileUrl: downloadLink,
                        mediaPoster: getImagePath(media.poster, 'original') ?? null,
                        seasonPoster: extracted?.season
                            ? getSeasonPoster(extracted?.season, media.seasons?.seasons ?? null)
                            : null,
                        uuid: crypto.randomUUID(),
                        resolverId: resolver.id,
                        userId: $user.id,
                        infoHash: source.info_hash,
                    })
                }

                if (i < filesToDownload.length - 1) {
                    await new Promise((resolve) => setTimeout(resolve, DELAY_MS))
                }
            } catch (error) {
                handleError(error)
                continue
            }
        }
    }
    const copyToClipboard = async (): Promise<void> => {
        loading = true
        try {
            await addDownloadLinks()
            const fileUrls = downloadLinks.map((link) => link.fileUrl).join('\n')
            await navigator.clipboard.writeText(fileUrls)
            onCancel()
        } catch (error) {
            handleError(error)
        } finally {
            loading = false
        }
    }

    const downloadFiles = async (): Promise<void> => {
        try {
            if (!downloadDirectory) {
                const selected = await open({ multiple: false, directory: true })
                if (!selected) {
                    onCancel()
                    return
                }
                downloadDirectory = selected
            }
            loading = true

            await addDownloadLinks()

            if (!$user) throw createError('Missing User', 401, { log: false })

            for (const link of downloadLinks) {
                try {
                    const response = await invokeFunction('download_file', { params: link, userId: $user.id })
                    if (!response.success) throw response.error

                    downloadsInProgress.update((d) => [
                        ...d,
                        {
                            link,
                            progress: 0,
                            speed: 0,
                            eta: 0,
                            fileIndex: 1,
                            status: 'queued',
                        },
                    ])
                } catch (error) {
                    handleError(error)
                }
            }

            const response = await invokeFunction('process_download_queue', { userId: $user.id })
            if (!response.success) throw response.error

            onCancel()
        } catch (error) {
            handleError(error)
        } finally {
            loading = false
        }
    }
</script>

<svelte:window onkeydown={handleKeypress} />

<BaseModal onClose={onCancel}>
    <div class="relative mt-2 flex w-225 flex-col rounded">
        <div class="text-text sticky top-0 py-2 text-center font-medium">
            <h1>Select Files to Download</h1>
        </div>
        <div class="mb-5 flex w-full justify-between px-5 text-sm font-medium text-textColor">
            <div class="flex w-full items-center justify-between">
                {#if files.length > 0}
                    {#if files.length !== filesToDownload.length}
                        <button
                            disabled={filesToDownload.length === 0 || loading === true}
                            class="hover:text-primaryColor disabled:cursor-default disabled:text-slate-400 disabled:hover:bg-backgroundColor"
                            onclick={selectAll}>Select all</button>
                    {:else}
                        <button
                            disabled={filesToDownload.length === 0 || loading === true}
                            class="hover:text-primaryColor disabled:cursor-default disabled:text-slate-400 disabled:hover:bg-backgroundColor"
                            onclick={() => (filesToDownload = [])}>Deselect all</button>
                    {/if}
                {/if}
                {#if filesToDownload.length > 0}
                    <button
                        onclick={copyToClipboard}
                        disabled={filesToDownload.length === 0 || loading === true}
                        class="hover:text-primaryColor disabled:cursor-default disabled:text-slate-400 disabled:hover:bg-backgroundColor"
                        ><CopyIcon /></button>
                {/if}
            </div>
        </div>
        {#if loading}
            <div class="mt-5 flex h-[50vh] items-center justify-center px-5">
                <LoadingSpinner />
            </div>
        {:else}
            <div class="mt-5 flex h-[50vh] flex-col space-y-2 overflow-auto px-5">
                {#each files as file (file)}
                    <div class="inline-flex w-full items-center justify-between">
                        <div class="flex items-center">
                            <Checkbox
                                label={file.filename}
                                id={file.filename}
                                checked={filesToDownload.some((f) => f.filename === file.filename)}
                                func={(e: Event) => {
                                    const target = e.target as HTMLInputElement
                                    toggleFile(file, target.checked)
                                }} />
                        </div>
                        <span class="text-text text-sm">
                            {(file.size / (1024 * 1024 * 1024)).toFixed(2)} GB
                        </span>
                    </div>
                {/each}
            </div>
        {/if}
        <div class="mx-auto flex w-fit items-center justify-center gap-x-5 py-5">
            <button
                disabled={loading === true}
                onclick={onCancel}
                class="px-6 py-3.5 font-medium text-slate-400 transition-colors hover:text-slate-200 disabled:cursor-default disabled:hover:text-slate-400">
                Cancel
            </button>
            <button
                onclick={downloadFiles}
                disabled={filesToDownload.length === 0 || loading === true}
                class="flex-1 rounded-lg bg-slate-600 px-6 py-3.5 font-bold shadow-lg transition-all hover:text-primaryColor disabled:cursor-default disabled:text-slate-400 disabled:hover:bg-slate-600">
                Download Selected
            </button>
        </div>
    </div>
</BaseModal>
