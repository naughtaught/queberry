<script lang="ts">
    import SizeIcon from 'virtual:icons/material-symbols/database'
    import ResolutionIcon from 'virtual:icons/material-symbols/hd'
    import SeedsIcon from 'virtual:icons/ic/baseline-people'
    import CachedIcon from 'virtual:icons/octicon/cache-16'
    import VideoDetailsIcon from 'virtual:icons/mdi/slate-outline'
    import AudioDetailsIcon from 'virtual:icons/ri/music-fill'
    import TagsIcon from 'virtual:icons/mdi/tags'
    import PlayIcon from 'virtual:icons/material-symbols/play-circle'
    import SourceIcon from 'virtual:icons/ic/baseline-place'
    import IndexersIcon from 'virtual:icons/lucide/library'

    import BlacklistSourceButton from '$lib/components/inputs/BlacklistSourceButton.svelte'
    import { loadVideo } from '$lib/functions/video/loadVideo'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import { downloadsInProgress, transfersInProgress } from '$lib/stores/plugins'
    import ProgressBar from '$lib/components/ui/ProgressBar.svelte'
    import { startTransferPolling, stopTransferPolling } from '$lib/functions/plugins/transferPolling'
    import { onDestroy } from 'svelte'
    import { loadingStates, modals } from '$lib/stores/app'

    let { source = $bindable(), media, selectedSeason, selectedEpisode, selectedSource = $bindable() } = $props()

    let isLoading = $state(false)
    const transfer = $derived($transfersInProgress[source.info_hash] ?? null)
    const download = $derived(
        $downloadsInProgress.find((d) => {
            if (source.info_hash && d.link.infoHash === source.info_hash) {
                if (media.type === 'tv' && d.link.season !== null && d.link.episode !== null) {
                    const sourceSeasonNum = selectedSeason?.season_num
                    const sourceEpisodeNum = selectedEpisode?.episode_num
                    return d.link.season === sourceSeasonNum && d.link.episode === sourceEpisodeNum
                }
                return true
            }
            return false
        }) ?? null,
    )
    let isPolling = $state(false)

    $effect(() => {
        if (transfer?.hash === source.info_hash) {
            if (isPolling) return
            startTransferPolling(transfer)
            isPolling = true
        }
    })

    onDestroy(() => {
        if (transfer && isPolling) stopTransferPolling(transfer)
    })
</script>

<div class="glass-panel relative mb-8 w-full overflow-hidden rounded-xl border border-slate-200/10 shadow-2xl">
    <div class="flex flex-col">
        <div class="absolute top-0.5 right-1">
            <BlacklistSourceButton infohash={source.info_hash} size="text-xs" />
        </div>
        <div class="flex flex-col gap-5 p-5">
            <div class="flex items-start justify-between gap-2">
                <h2 class="line-clamp-3 text-base leading-snug font-bold tracking-tight text-textColor">
                    {source.filename}
                </h2>
            </div>
            <div class="grid grid-cols-2 gap-x-2 gap-y-3 border-b border-slate-200/10 pb-5 text-[11px]">
                <div class="flex items-center gap-1.5 text-slate-500">
                    <SizeIcon class="text-[16px]" />
                    <span class="font-medium">{source.size.toFixed(2)} GB</span>
                </div>
                <div class="flex items-center gap-1.5 text-slate-500">
                    <ResolutionIcon class="text-[16px]" />
                    <span class="font-medium">{source.resolution}</span>
                </div>
                <div class="flex items-center gap-1.5 text-slate-500">
                    <SeedsIcon class="text-[16px]" />
                    <span class="font-medium">{source.seeds} Seeds</span>
                </div>
                <div class="flex items-center gap-1.5 text-slate-500">
                    <SourceIcon class="text-[16px]" />
                    <span class="font-medium">{source.source ?? 'Unknown'}</span>
                </div>
            </div>
            <div class="flex flex-col gap-y-3 border-b border-slate-200/10 pb-5 text-[11px]">
                <div class="flex items-center gap-1.5 text-slate-500">
                    <CachedIcon class="text-[16px]" />
                    <span class="font-medium">{source.cached}</span>
                </div>
                <div class="flex items-center gap-1.5 text-slate-500">
                    <IndexersIcon class="text-[16px]" />
                    <span class="font-medium">{source.indexer}</span>
                </div>
            </div>
            <div class="space-y-5">
                <div class="flex flex-col gap-2">
                    <div class="flex items-center gap-2">
                        <VideoDetailsIcon class="text-[18px] text-slate-500" />
                        <span class="text-[10px] font-bold tracking-widest uppercase">Video</span>
                    </div>
                    <div class="grid grid-cols-1 gap-1 text-[11px]">
                        <div class="flex justify-between border-b border-white/5 pb-1">
                            <span class="text-slate-500">Codec</span><span class="font-medium text-slate-300"
                                >{source.video_details.codec ?? ''}</span>
                        </div>
                        <div class="flex justify-between border-b border-white/5 pb-1">
                            <span class="text-slate-500">Bitrate</span><span class="font-medium text-slate-300"
                                >{source.video_details.bitrate}</span>
                        </div>
                        <div class="flex justify-between">
                            <span class="text-slate-500">Framerate</span><span class="font-medium text-slate-300"
                                >{source.video_details.framerate}</span>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col gap-2">
                    <div class="flex items-center gap-2">
                        <AudioDetailsIcon class="text-[18px] text-slate-500" />
                        <span class="text-[10px] font-bold tracking-widest uppercase">Audio</span>
                    </div>
                    <div class="grid grid-cols-1 gap-1 text-[11px]">
                        <div class="flex justify-between border-b border-white/5 pb-1">
                            <span class="text-slate-500">Codec</span><span class="font-medium text-slate-300"
                                >{source.audio_details.codec ?? ''}</span>
                        </div>
                        <div class="flex justify-between border-b border-white/5 pb-1">
                            <span class="text-slate-500">Channels</span><span class="font-medium text-slate-300"
                                >{source.audio_details.channels}</span>
                        </div>
                        <div class="flex justify-between border-b border-white/5 pb-1">
                            <span class="text-slate-500">Sampling</span><span class="font-medium text-slate-300"
                                >{source.audio_details.sampling_rate}</span>
                        </div>
                        <div class="flex justify-between">
                            <span class="text-slate-500">Languages</span><span class="font-medium text-slate-300"
                                >{source.langauge ?? 'Unknown'}</span>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col gap-2">
                    <div class="flex items-center gap-2">
                        <TagsIcon class="text-[18px] text-slate-500" />
                        <span class="text-[10px] font-bold tracking-widest uppercase">Tags</span>
                    </div>
                    <div class="mt-1 flex flex-wrap gap-1.5">
                        {#each source.tags.concat(source.video_filters) as tag (tag)}
                            <span
                                class="rounded bg-slate-800 px-1.5 py-0.5 text-[8px] font-bold text-slate-300 uppercase"
                                >{tag}</span>
                        {/each}
                    </div>
                </div>
            </div>
            <div class="flex flex-col gap-3 border-t border-slate-200/10 pt-5">
                <div class="flex flex-col gap-2">
                    {#if transfer}
                        <ProgressBar progressData={transfer} type="transfer" />
                    {:else if download}
                        <ProgressBar progressData={download} type="download" />
                    {:else if source.source !== 'Local Media' && source.cached.length > 0}
                        <button
                            disabled={$loadingStates.isPlayButtonLoading}
                            onclick={() => {
                                selectedSource = source
                                $modals.download = true
                            }}
                            class="w-full rounded-lg border py-2.5 text-xs font-bold text-slate-600 transition-colors disabled:cursor-default">
                            Download
                        </button>
                    {/if}
                    <button
                        disabled={transfer !== null || $loadingStates.isPlayButtonLoading}
                        onclick={() => {
                            $loadingStates.isPlayButtonLoading = true
                            isLoading = true
                            selectedSource = source
                            loadVideo(
                                media,
                                [source],
                                selectedSeason?.season_num ?? null,
                                selectedEpisode ?? null,
                                true,
                            ).finally(() => {
                                isLoading = false
                            })
                        }}
                        class="{transfer !== null
                            ? 'cursor-default! text-slate-500'
                            : 'group text-textColor hover:text-primaryColor'}  flex min-h-12 flex-1 items-center justify-center gap-2 rounded-xl bg-slate-800 py-3 font-bold shadow-lg shadow-backgroundColor disabled:cursor-default">
                        {#if !isLoading && !$loadingStates.isPlayButtonLoading}
                            <PlayIcon class="text-xl group-hover:text-primaryColor" />
                            Play
                        {:else}
                            <LoadingSpinner />
                        {/if}
                    </button>
                </div>
            </div>
        </div>
    </div>
</div>
