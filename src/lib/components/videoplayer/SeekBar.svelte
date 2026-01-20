<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { videoMetadata, videoState } from '$lib/stores/video'
    import { handleError, type Api } from '$lib'

    let cacheTime = $derived($videoState.cache_time)
    let isDragging = $state(false)
    let sliderValue = $state(0)
    let sliderPosition = $state(0)
    let sliderElement: HTMLInputElement
    let min = 0

    let displayCurrent = $derived.by(() => {
        return formatTime($videoState.current_time)
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

    const thumbWidth = 16

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
                $videoState.current_time = response.data!.time
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
        if (!isDragging) sliderValue = $videoState.current_time
    })

    $effect(() => {
        if (isDragging) updatePosition(sliderValue)
    })

    const updatePosition = (val: number) => {
        if (!sliderElement) return

        const sliderRect = sliderElement.getBoundingClientRect()
        const sliderWidth = sliderRect.width

        const effectiveWidth = sliderWidth - thumbWidth
        const effectiveStart = thumbWidth / 2

        const percentage = (val - min) / ($videoMetadata.duration - min)

        const adjustedPosition = effectiveStart + percentage * effectiveWidth

        sliderPosition = (adjustedPosition / sliderWidth) * 100
    }

    $effect(() => {
        if (!$videoMetadata.duration) return

        const playedPercent = ($videoState.current_time / $videoMetadata.duration) * 100
        const cachedEndPercent = (cachedEndPosition / $videoMetadata.duration) * 100

        const primary = '#ff6600'
        const primaryTransparent = 'rgba(255,102,0,0.3)'

        const bg = `linear-gradient(
            to right,
            ${primary} 0%,
            ${primary} ${playedPercent}%,
            ${primaryTransparent} ${playedPercent}%,
            ${primaryTransparent} ${cachedEndPercent}%,
            #a3a3a3 ${cachedEndPercent}%,
            #a3a3a3 100%
        )`

        sliderElement.style.background = bg
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
                class="h-1 w-full cursor-pointer appearance-none rounded-full from-[#ff6600] to-[#ff6600] [&::-webkit-slider-thumb]:h-2 [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-[#ff6600] [&::-webkit-slider-thumb]:shadow-md" />
            <div
                class={`pointer-events-none absolute -translate-x-1/2 transform rounded bg-[#ff6600] px-2 py-1 text-xs text-white transition-opacity duration-200 ${isDragging ? 'opacity-100' : 'opacity-0'}`}
                style="left: {sliderPosition}%; top:-20px; z-index: 10;">
                {displayValue}
                <div
                    class="absolute left-1/2 -translate-x-1/2 border-t-4 border-r-4 border-l-4 border-solid border-[#ff6600] border-r-transparent border-l-transparent"
                    style="top: 100%;">
                </div>
            </div>
        </div>
    </div>
    <div class="flex justify-between text-sm">
        <p>{displayCurrent}</p>
        <p>{displayDuration}</p>
    </div>
</div>
