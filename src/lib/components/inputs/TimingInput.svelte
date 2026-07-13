<script lang="ts">
    import { sessionSettings, videoMetadata, videoProperties } from '$lib/stores/video'
    import Checkbox from '$lib/components/inputs/Checkbox.svelte'
    import type { Api } from '$lib/types/api'
    import UpdateIcon from 'virtual:icons/radix-icons/update'
    import CheckIcon from 'virtual:icons/material-symbols/check'
    import { createError, handleError } from '$lib/functions/errors/errorHandling'
    import { enabledUtilityPlugins } from '$lib/stores/plugins'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { updateLocalTimings } from '$lib/functions/video/updateLocalTimings'

    const {
        timings,
        label,
        disableKey,
        updateKey,
    }: {
        timings: Api.TimingRange[] | undefined
        label: string
        disableKey: 'disableIntroTiming' | 'disableRecapTiming' | 'disablePreviewTiming' | 'disableCreditTiming'
        updateKey: 'isIntroTimingUpdated' | 'isRecapTimingUpdated' | 'isPreviewTimingUpdated' | 'isCreditTimingUpdated'
    } = $props()

    let editableTimings = $state<Api.TimingRange[]>([])
    let startTime = $state<number | null>(null)
    let endTime = $state<number | null>(null)
    let startHours = $state('')
    let startMinutes = $state('')
    let startSeconds = $state('')
    let endHours = $state('')
    let endMinutes = $state('')
    let endSeconds = $state('')
    let initialized = $state(false)
    const hasApiKey = $derived(
        $enabledUtilityPlugins.some(
            (key) => key.apikey && key.methods?.some((method) => method.interfaceMethod === 'GetIntroTimings'),
        ),
    )
    const isInputDisabled = $derived($sessionSettings[disableKey] || $sessionSettings[updateKey] || !hasApiKey)

    const parseTimeToSeconds = (h: string, m: string, s: string): number | null => {
        const hours = parseInt(h) || 0
        const minutes = parseInt(m) || 0
        const seconds = parseInt(s) || 0
        return hours * 3600 + minutes * 60 + seconds
    }

    $effect(() => {
        if (!initialized && timings) {
            editableTimings = JSON.parse(JSON.stringify(timings))
            startTime = editableTimings[0]?.start ?? null
            endTime = editableTimings[0]?.end ?? null

            if (startTime !== null) {
                startHours = Math.floor(startTime / 3600)
                    .toString()
                    .padStart(2, '0')
                startMinutes = Math.floor((startTime % 3600) / 60)
                    .toString()
                    .padStart(2, '0')
                startSeconds = Math.floor(startTime % 60)
                    .toString()
                    .padStart(2, '0')
            }
            if (endTime !== null) {
                endHours = Math.floor(endTime / 3600)
                    .toString()
                    .padStart(2, '0')
                endMinutes = Math.floor((endTime % 3600) / 60)
                    .toString()
                    .padStart(2, '0')
                endSeconds = Math.floor(endTime % 60)
                    .toString()
                    .padStart(2, '0')
            }

            initialized = true
        }
    })

    const updateStartTime = (): void => {
        startTime = parseTimeToSeconds(startHours, startMinutes, startSeconds)
        updateTimingsArray()
    }

    const updateEndTime = (): void => {
        endTime = parseTimeToSeconds(endHours, endMinutes, endSeconds)
        updateTimingsArray()
    }

    const updateTimingsArray = (): void => {
        if (editableTimings.length === 0) {
            editableTimings = [{ start: null, end: null }]
        }
        editableTimings[0] = {
            ...editableTimings[0],
            start: startTime,
            end: endTime,
        }
    }

    const updateExternalTimings = async (): Promise<void> => {
        try {
            $sessionSettings[updateKey] = true

            if (!startTime && !endTime) {
                throw createError('Missing a valid timestamp.', 400, { log: false })
            }

            const TimingPlugins = $enabledUtilityPlugins.filter((plugin) => {
                const method = plugin.methods?.find((m) => m.interfaceMethod === 'GetIntroTimings')
                return method && (!method.requiresApiKey || !!plugin.apikey)
            })

            if (TimingPlugins.length === 0) throw createError('Valid Plugin not found.', 404, { log: false })

            let anyPluginSucceeded = false
            const segmentMap: Record<string, string> = {
                Intro: 'intro',
                Recap: 'recap',
                Preview: 'preview',
                Credits: 'credits',
            }
            const segmentType = segmentMap[label] || label.toLowerCase()

            for (const plugin of TimingPlugins) {
                try {
                    const resp = await invokeFunction('call_plugin_method', {
                        pluginName: plugin.id,
                        methodName: 'UpdateIntroTimings',
                        args: [
                            plugin.apikey ?? null,
                            $videoMetadata.media?.imdb_id,
                            $videoMetadata.media?.tmdb_id,
                            $videoMetadata.media?.tvdb_id,
                            segmentType,
                            $videoMetadata.seasonNumber ?? null,
                            $videoMetadata.episode?.episode_num ?? null,
                            Math.round($videoProperties.duration),
                            startTime,
                            endTime,
                            $videoMetadata.media?.type,
                        ],
                    })

                    if (resp.success) anyPluginSucceeded = true
                } catch (error) {
                    handleError(error)
                }
            }

            if (anyPluginSucceeded) {
                const metadata = $videoMetadata
                let currentTimings: Record<string, Api.TimingRange[]> = {
                    intro: [],
                    recap: [],
                    preview: [],
                    credits: [],
                }

                if (metadata.media?.type === 'tv' && metadata.episode) {
                    currentTimings = {
                        intro: metadata.episode.intro_timings || [],
                        recap: metadata.episode.recap_timings || [],
                        preview: metadata.episode.preview_timings || [],
                        credits: metadata.episode.credits_timings || [],
                    }
                } else if (metadata.media?.type === 'movie' && metadata.media) {
                    currentTimings = {
                        intro: metadata.media.intro_timings || [],
                        recap: metadata.media.recap_timings || [],
                        preview: metadata.media.preview_timings || [],
                        credits: metadata.media.credits_timings || [],
                    }
                }

                currentTimings[segmentType] = [{ start: startTime, end: endTime }]

                updateLocalTimings({
                    intro: currentTimings.intro,
                    recap: currentTimings.recap,
                    credits: currentTimings.credits,
                    preview: currentTimings.preview,
                })
            } else {
                $sessionSettings[updateKey] = false
            }
        } catch (error) {
            $sessionSettings[updateKey] = false
            handleError(error)
        }
    }
</script>

<div class="w-full text-left">
    <div class="mt-1 grid grid-cols-[auto_1fr_auto_auto] items-center gap-2">
        <Checkbox
            label=""
            checked={$sessionSettings[disableKey]}
            func={() => {
                $sessionSettings[disableKey] = !$sessionSettings[disableKey]
            }} />
        <span class="w-14 cursor-default text-sm text-textColor" class:opacity-50={$sessionSettings[disableKey]}
            >{label}</span>
        <div class="flex cursor-default gap-1">
            <!-- Start Time -->
            <div class="flex items-center gap-0.5">
                <input
                    type="text"
                    maxlength="2"
                    placeholder="hh"
                    value={startHours}
                    oninput={(e) => {
                        if (e.target) {
                            startHours = (e.target as HTMLInputElement).value
                            updateStartTime()
                        }
                    }}
                    disabled={isInputDisabled}
                    class="w-10 rounded border border-slate-700 bg-slate-800/50 px-1 text-center text-xs text-textColor disabled:cursor-default disabled:opacity-50"
                    aria-label="{label} start hours" />
                <span class="text-xs text-textColor">:</span>
                <input
                    type="text"
                    maxlength="2"
                    placeholder="mm"
                    value={startMinutes}
                    oninput={(e) => {
                        if (e.target) {
                            startMinutes = (e.target as HTMLInputElement).value
                            updateStartTime()
                        }
                    }}
                    disabled={isInputDisabled}
                    class="w-10 rounded border border-slate-700 bg-slate-800/50 px-1 text-center text-xs text-textColor disabled:cursor-default disabled:opacity-50"
                    aria-label="{label} start minutes" />
                <span class="text-xs text-textColor">:</span>
                <input
                    type="text"
                    maxlength="2"
                    placeholder="ss"
                    value={startSeconds}
                    oninput={(e) => {
                        if (e.target) {
                            startSeconds = (e.target as HTMLInputElement).value
                            updateStartTime()
                        }
                    }}
                    disabled={isInputDisabled}
                    class="w-10 rounded border border-slate-700 bg-slate-800/50 px-1 text-center text-xs text-textColor disabled:cursor-default disabled:opacity-50"
                    aria-label="{label} start seconds" />
            </div>
            <span class="mx-1 flex items-center text-xs text-textColor">-</span>
            <div class="flex items-center gap-0.5">
                <input
                    type="text"
                    maxlength="2"
                    placeholder="hh"
                    value={endHours}
                    oninput={(e) => {
                        if (e.target) {
                            endHours = (e.target as HTMLInputElement).value
                            updateEndTime()
                        }
                    }}
                    disabled={isInputDisabled}
                    class="w-10 rounded border border-slate-700 bg-slate-800/50 px-1 text-center text-xs text-textColor disabled:cursor-default disabled:opacity-50"
                    aria-label="{label} end hours" />
                <span class="text-xs text-textColor">:</span>
                <input
                    type="text"
                    maxlength="2"
                    placeholder="mm"
                    value={endMinutes}
                    oninput={(e) => {
                        if (e.target) {
                            endMinutes = (e.target as HTMLInputElement).value
                            updateEndTime()
                        }
                    }}
                    disabled={isInputDisabled}
                    class="w-10 rounded border border-slate-700 bg-slate-800/50 px-1 text-center text-xs text-textColor disabled:cursor-default disabled:opacity-50"
                    aria-label="{label} end minutes" />
                <span class="text-xs text-textColor">:</span>
                <input
                    type="text"
                    maxlength="2"
                    placeholder="ss"
                    value={endSeconds}
                    oninput={(e) => {
                        if (e.target) {
                            endSeconds = (e.target as HTMLInputElement).value
                            updateEndTime()
                        }
                    }}
                    disabled={isInputDisabled}
                    class="w-10 rounded border border-slate-700 bg-slate-800/50 px-1 text-center text-xs text-textColor disabled:cursor-default disabled:opacity-50"
                    aria-label="{label} end seconds" />
            </div>
        </div>
        <div>
            {#if hasApiKey}
                <button
                    onclick={updateExternalTimings}
                    disabled={isInputDisabled}
                    class="flex h-8 w-8 items-center justify-center rounded-lg bg-slate-800 shadow-lg transition-colors disabled:cursor-default disabled:opacity-50">
                    {#if isInputDisabled}
                        <CheckIcon class="h-4 w-4 text-green-400" />
                    {:else}
                        <UpdateIcon class="h-4 w-4" />
                    {/if}
                </button>
            {/if}
        </div>
    </div>
</div>
