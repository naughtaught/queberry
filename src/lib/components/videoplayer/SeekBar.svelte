<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { handleError, settings, videoMetadata, videoState, type Api } from '$lib'

    let cacheTime = $derived($videoState.cacheTime)
    let isDragging = $state(false)
    let sliderValue = $state(0)
    let sliderPosition = $state(0)
    let sliderElement: HTMLInputElement
    let min = 0
    let displayDurationMode = $state($settings.durationDisplay)

    let displayCurrent = $derived.by(() => {
        if (displayDurationMode === 'Time Remaining') {
            const remaining = $videoMetadata.duration - $videoState.currentTime
            return '-' + formatTime(remaining)
        }
        return formatTime($videoState.currentTime)
    })
    let displayDuration = $derived.by(() => {
        return formatTime($videoMetadata.duration)
    })
    let displayValue = $derived.by(() => {
        return formatTime(sliderValue)
    })
    let cachedEndPosition = $derived.by(() => {
        const cacheTimeNum = Number(cacheTime) || 0
        return Math.min(cacheTimeNum, $videoMetadata.duration)
    })

    const formatTime = (seconds: number) => {
        const sec = Math.floor(seconds % 60)
        const min = Math.floor((seconds / 60) % 60)
        const hr = Math.floor(seconds / 3600)
        const pad = (n: number) => n.toString().padStart(2, '0')
        return hr > 0 ? `${pad(hr)}:${pad(min)}:${pad(sec)}` : `${pad(min)}:${pad(sec)}`
    }

    const handlePointerDown = () => {
        isDragging = true
        updatePosition(sliderValue)
    }

    const handlePointerUp = async (time: number) => {
        try {
            const response: Api.ApiResponse = await invoke('set_time', {
                time: time,
            })
            if (response.success) {
                $videoState.currentTime = response.data.time
            } else {
                handleError(response.error!)
            }
        } catch (error) {
            const errorDetail: Api.ErrorDetail = {
                code: 500,
                message: error instanceof Error ? error.message : String(error),
                stack: error instanceof Error ? error.stack : undefined,
            }
            handleError(errorDetail)
        }
        isDragging = false
    }

    const handleInput = (e: Event) => {
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

    const updatePosition = (val: number) => {
        if (!sliderElement) return

        const sliderRect = sliderElement.getBoundingClientRect()
        const sliderWidth = sliderRect.width

        const percentage = (val - min) / ($videoMetadata.duration - min)

        const adjustedPosition = percentage * sliderWidth

        sliderPosition = (adjustedPosition / sliderWidth) * 100
    }

    $effect(() => {
        if (!sliderElement) return

        let playedPercent = 0
        let cachedEndPercent = 0

        if ($videoMetadata.duration && $videoMetadata.duration > 0) {
            playedPercent = ($videoState.currentTime / $videoMetadata.duration) * 100
            cachedEndPercent = (cachedEndPosition / $videoMetadata.duration) * 100
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
</script>

<div class="w-full">
    <div class="flex w-full items-center justify-between">
        <div class="relative w-full">
            <input
                type="range"
                {min}
                max={$videoMetadata.duration}
                step="0.01"
                bind:value={sliderValue}
                bind:this={sliderElement}
                onpointerdown={handlePointerDown}
                onpointerup={() => handlePointerUp(sliderValue)}
                oninput={handleInput}
                class="from-bg-primaryColor to-bg-primaryColor h-1 w-full cursor-pointer appearance-none rounded-full [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:appearance-none" />
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
                displayDurationMode =
                    displayDurationMode === 'Duration' ? 'Time Remaining' : 'Duration'
            }}>{displayDuration}</button>
    </div>
</div>
