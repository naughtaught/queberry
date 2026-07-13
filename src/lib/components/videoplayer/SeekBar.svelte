<script lang="ts">
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { settings } from '$lib/stores/user'
    import { sessionSettings, videoMetadata, videoProperties, videoState } from '$lib/stores/video'
    import { SvelteSet } from 'svelte/reactivity'

    const cacheTime = $derived($videoState.cacheTime)
    const thumbWidth = 8
    let isDragging = $state(false)
    let sliderValue = $state(0)
    let sliderPosition = $state(0)
    let sliderElement: HTMLInputElement
    const min = 0
    let displayDurationMode = $state($settings.durationDisplay)
    let currentEpisodeId: number | undefined

    const displayCurrent = $derived.by(() => {
        if (displayDurationMode === 'Time Remaining') {
            const remaining = $videoProperties.duration - $videoState.currentTime
            return `-${formatTime(remaining)}`
        }
        return formatTime($videoState.currentTime)
    })
    const displayDuration = $derived.by(() => formatTime($videoProperties.duration))
    const displayValue = $derived.by(() => formatTime(sliderValue))
    const cachedEndPosition = $derived.by(() => {
        const cacheTimeNum = Number(cacheTime) || 0
        return Math.min(cacheTimeNum, $videoProperties.duration)
    })
    const skippedSegments = new SvelteSet<string>()

    const formatTime = (seconds: number): string => {
        const second = Math.floor(seconds % 60)
        const minute = Math.floor((seconds / 60) % 60)
        const hour = Math.floor(seconds / 3600)
        const pad = (n: number): string => n.toString().padStart(2, '0')
        return hour > 0 ? `${pad(hour)}:${pad(minute)}:${pad(second)}` : `${pad(minute)}:${pad(second)}`
    }

    const handlePointerDown = (): void => {
        isDragging = true
        updatePosition(sliderValue)
    }

    const handlePointerUp = async (time: number): Promise<void> => {
        try {
            const resp = await invokeFunction('set_time', {
                value: time,
            })
            if (!resp.success) throw resp.error

            $videoState.currentTime = resp.data.value
            isDragging = false
        } catch (error) {
            handleError(error, {
                context: 'setting the video time failed',
            })
        }
    }

    const handleInput = (e: Event): void => {
        if (isDragging) {
            const target = e.target as HTMLInputElement
            updatePosition(target.valueAsNumber)
        }
    }

    $effect(() => {
        if (!isDragging) sliderValue = $videoState.currentTime
    })

    $effect(() => {
        if (isDragging) updatePosition(sliderValue)
    })

    const updatePosition = (val: number): void => {
        if (!sliderElement) return

        const sliderRect = sliderElement.getBoundingClientRect()
        const sliderWidth = sliderRect.width

        const effectiveWidth = sliderWidth - thumbWidth
        const effectiveStart = thumbWidth / 2

        const percentage = (val - min) / ($videoProperties.duration - min)

        const adjustedPosition = effectiveStart + percentage * effectiveWidth

        sliderPosition = (adjustedPosition / sliderWidth) * 100
    }

    $effect(() => {
        if (!sliderElement) return

        let playedPercent = 0
        let cachedEndPercent = 0

        if ($videoProperties.duration && $videoProperties.duration > 0) {
            playedPercent = ($videoState.currentTime / $videoProperties.duration) * 100
            cachedEndPercent = (cachedEndPosition / $videoProperties.duration) * 100
        }

        playedPercent = Math.max(0, Math.min(100, playedPercent))
        cachedEndPercent = Math.max(0, Math.min(100, cachedEndPercent))

        sliderElement.style.background = `linear-gradient(
            to right,
            var(--color-primaryColor) 0%,
            var(--color-primaryColor) ${playedPercent}%,
            rgb(from var(--color-primaryColor) r g b / 0.3)  ${playedPercent}%,
            rgb(from var(--color-primaryColor) r g b / 0.3)  ${cachedEndPercent}%,
            var(--color-neutral-400) ${cachedEndPercent}%,
            var(--color-neutral-400) 100%
        )`
    })

    $effect(() => {
        const episode = $videoMetadata.episode
        const currentTime = $videoState.currentTime
        const duration = $videoProperties.duration

        if (!episode || $sessionSettings.disableAllTimings || currentTime < 0.5) return

        if (episode.episode_id !== currentEpisodeId) {
            currentEpisodeId = episode.episode_id
            skippedSegments.clear()
        }

        const isFirstEpisode = $videoMetadata.seasonNumber === 1 && episode.episode_num === 1

        const skipConfigs = [
            {
                timings: episode.intro_timings,
                shouldSkip: $settings.skipIntro && !$sessionSettings.disableIntroTiming && !isFirstEpisode,
                name: 'intro',
            },
            {
                timings: episode.recap_timings,
                shouldSkip: $settings.skipRecap && !$sessionSettings.disableRecapTiming,
                name: 'recap',
            },
            {
                timings: episode.preview_timings,
                shouldSkip: $settings.skipPreview && !$sessionSettings.disablePreviewTiming,
                name: 'preview',
            },
            {
                timings: episode.credits_timings,
                shouldSkip: $settings.skipCredits && !$sessionSettings.disableCreditTiming,
                name: 'credits',
            },
        ]

        let seekTo: number | null = null

        for (const { timings, shouldSkip, name } of skipConfigs) {
            if (!timings || !shouldSkip) continue

            for (const timing of timings) {
                const startSec = timing.start ?? 0
                const endSec = timing.end
                const segmentKey = `${name}-${startSec}-${endSec}`

                if (startSec >= duration) continue
                if (skippedSegments.has(segmentKey)) continue

                if (currentTime >= startSec && (!endSec || currentTime < endSec)) {
                    let target = endSec ?? duration

                    if (name === 'preview') {
                        const creditsTiming = episode.credits_timings?.[0]
                        const creditsStart = creditsTiming?.start

                        if (
                            creditsStart !== undefined &&
                            creditsStart !== null &&
                            Math.abs(target - creditsStart) <= 1
                        ) {
                            target = creditsTiming?.end ?? Math.max(0, duration - 5)
                        }
                    }

                    if (name === 'preview' || name === 'credits') {
                        const creditsTiming = episode.credits_timings?.[0]
                        const previewTiming = episode.preview_timings?.[0]

                        const creditsEnd = creditsTiming?.end
                        const previewEnd = previewTiming?.end

                        const isAtDuration =
                            endSec === null ||
                            endSec === undefined ||
                            Math.abs(endSec - duration) <= 1 ||
                            (creditsEnd !== null && creditsEnd !== undefined && Math.abs(creditsEnd - duration) <= 1) ||
                            (previewEnd !== null && previewEnd !== undefined && Math.abs(previewEnd - duration) <= 1)

                        if (isAtDuration) target = Math.max(0, duration - 5)
                    }

                    if (seekTo === null || target > seekTo) seekTo = Math.min(target, duration)

                    skippedSegments.add(segmentKey)
                }
            }
        }

        if (seekTo !== null && seekTo > currentTime) {
            handlePointerUp(Math.min(seekTo + 0.1, duration))
        }
    })
</script>

<div class="w-full">
    <div class="flex w-full items-center justify-between">
        <div class="relative w-full">
            <input
                type="range"
                {min}
                max={$videoProperties.duration}
                step="0.01"
                bind:value={sliderValue}
                bind:this={sliderElement}
                onpointerdown={handlePointerDown}
                onpointerup={() => handlePointerUp(sliderValue)}
                oninput={handleInput}
                class="from-bg-primaryColor to-bg-primaryColor h-1 w-full cursor-pointer appearance-none rounded-full [&::-webkit-slider-thumb]:h-2 [&::-webkit-slider-thumb]:w-2 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-primaryShade-200 [&::-webkit-slider-thumb]:shadow-md hover:[&::-webkit-slider-thumb]:bg-primaryShade-300" />
            <div
                class={`pointer-events-none absolute -translate-x-1/2 transform rounded bg-primaryColor px-2 py-1 text-xs text-textColor transition-opacity duration-200 ${isDragging ? 'opacity-100' : 'opacity-0'}`}
                style="left: {sliderPosition}%; top:-20px; z-index: 10;">
                {displayValue}
                <div
                    class="absolute left-1/2 -translate-x-1/2 border-t-4 border-r-4 border-l-4 border-solid border-primaryColor border-r-transparent border-l-transparent"
                    style="top: 100%;">
                </div>
            </div>
        </div>
    </div>
    <div class="flex justify-between text-sm text-textColor">
        <p>{displayCurrent}</p>
        <button
            onclick={() => {
                displayDurationMode = displayDurationMode === 'Duration' ? 'Time Remaining' : 'Duration'
            }}>{displayDuration}</button>
    </div>
</div>
