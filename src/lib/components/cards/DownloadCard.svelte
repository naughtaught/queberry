<script lang="ts">
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import CloseIcon from 'virtual:icons/material-symbols/close'
    import RetryIcon from 'virtual:icons/pajamas/retry'

    import { downloadsInProgress, enabledResolverPlugins } from '$lib/stores/plugins'
    import { createError, handleError } from '$lib/functions/errors/errorHandling'
    import { getImagePath } from '$lib/functions/ui/getImagePath'
    import { onMount } from 'svelte'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import { user } from '$lib/stores/user'
    import { checkMethodApi } from '$lib/functions/plugins/checkMethodApi'

    const { download } = $props()

    let posterPath = $state<string | null>(null)
    let loading = $state(false)

    onMount(() => {
        if (download.type === 'movie') {
            posterPath = getImagePath(download.link.mediaPoster, 'original')
        } else {
            posterPath =
                getImagePath(download.link.seasonPoster, 'original') ??
                getImagePath(download.link.mediaPoster, 'original')
        }
    })

    const formattedETA = $derived.by(() => {
        if (!download.eta || download.eta < 0 || !isFinite(download.eta)) return '0:00:00:00'

        const days = Math.floor(download.eta / 86400)
        const hours = Math.floor((download.eta % 86400) / 3600)
        const minutes = Math.floor((download.eta % 3600) / 60)
        const secs = Math.floor(download.eta % 60)

        return `${days}:${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}:${String(secs).padStart(2, '0')}`
    })

    const cancelDownload = async (): Promise<void> => {
        try {
            if (!$user) throw createError('Missing User', 401, { log: false })

            if (download.status === 'completed') {
                await invokeFunction('clear_completed_download', {
                    userId: $user.id,
                    uuid: download.link.uuid,
                })
            } else {
                await invokeFunction('cancel_download', {
                    uuid: download.link.uuid,
                    userId: $user.id,
                })
            }

            downloadsInProgress.update((x) => x.filter((entry) => entry.link.uuid !== download.link.uuid))
        } catch (error) {
            handleError(error)
        }
    }

    const retryDownload = async (): Promise<void> => {
        loading = true
        try {
            const plugin = $enabledResolverPlugins.find((x) => x.id === download.link.resolverId)

            if (!plugin) {
                throw createError('Missing valid resolver plugin', 404, {
                    log: false,
                })
            }

            checkMethodApi(plugin, 'UnrestrictLink')

            const resp = await invokeFunction('call_plugin_method', {
                pluginName: download.link.resolverId,
                methodName: 'UnrestrictLink',
                args: [plugin.apikey ?? null, download.link.fileLink],
            })

            if (!resp.success) throw resp.error

            if (resp.data?.link) {
                if (!$user) throw createError('Missing User', 401, { log: false })
                await invokeFunction('retry_download', {
                    uuid: download.link.uuid,
                    newFileUrl: resp.data.link,
                    userId: $user.id,
                })
            }
        } catch (error) {
            handleError(error)
        } finally {
            loading = false
        }
    }
</script>

<div
    class="glass-panel group sidebar-gradient relative w-full overflow-hidden rounded-lg p-8 shadow-2xl transition-all duration-500">
    <div
        class="absolute top-0 left-0 h-full w-1 origin-top scale-y-0 bg-primaryColor transition-transform group-hover:scale-y-100">
    </div>
    <div class="flex items-center gap-8">
        <div class="grow">
            <div class="flex gap-6">
                <div class="h-44 w-32 shrink-0 overflow-hidden rounded-md shadow-lg">
                    <img
                        alt="Search Result Poster"
                        class="h-full w-full object-cover"
                        src={posterPath ? posterPath : '/images/poster-placeholder.png'} />
                </div>

                <div class="flex grow flex-col">
                    <div class="flex items-start justify-between">
                        <div class="min-w-0 flex-1">
                            <h3 class="mb-1 space-x-4 text-xl font-bold">
                                {download.link.title}
                                {#if download.link.released}({download.link.released}){/if}
                                <span>
                                    {#if download.link.episode}S{download.link.season
                                            .toString()
                                            .padStart(2, '0')}E{download.link.episode.toString().padStart(2, '0')}{/if}
                                </span>
                            </h3>
                            <h4 class="mb-3 max-w-[75%] truncate text-xs">
                                {download.link.filename}
                            </h4>
                        </div>
                        <span class="ml-4 shrink-0 text-2xl font-black tracking-tighter"
                            >{download.progress.toFixed(0)}%</span>
                    </div>

                    <div class="mt-auto flex items-center justify-between">
                        <span class="flex items-center gap-1 text-[10px] tracking-[0.2em] uppercase">
                            <span class="text-[14px]">{download.status}</span>
                        </span>
                        <span class="flex items-center gap-1 text-[10px] tracking-[0.2em] text-textColor uppercase">
                            {download.speed.toFixed(2)} Mbps
                        </span>
                        <span class="flex items-center gap-1 text-[10px] tracking-[0.2em] text-textColor uppercase">
                            ETA: {formattedETA}
                        </span>
                    </div>

                    <div class="mt-3 h-2 w-full overflow-hidden rounded-full bg-white/5">
                        <div
                            class="h-full bg-primaryColor transition-all duration-1000"
                            style="width: {download.progress.toFixed(0)}%">
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div class="flex shrink-0 flex-col gap-10">
            {#if !loading}
                {#if download.status === 'failed'}
                    <button
                        onclick={retryDownload}
                        class="group/retryButton flex h-12 w-12 items-center justify-center rounded-full bg-white/5 transition-colors hover:bg-primaryColor/20">
                        <RetryIcon class="text-textColor transition-colors group-hover/retryButton:text-primaryColor" />
                    </button>
                {/if}
                <button
                    onclick={cancelDownload}
                    class="group/closebutton flex h-12 w-12 items-center justify-center rounded-full bg-white/5 transition-colors hover:bg-red-500/20">
                    <CloseIcon class="text-textColor transition-colors group-hover/closebutton:text-red-500" />
                </button>
            {:else}
                <div class="flex h-12 w-12 items-center justify-center">
                    <LoadingSpinner />
                </div>
            {/if}
        </div>
    </div>
</div>
