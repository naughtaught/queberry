<script lang="ts">
    let { min, max, minValue = $bindable(min), maxValue = $bindable(max), step = 1, label, reset } = $props()

    let isDraggingMin = $state(false)
    let isDraggingMax = $state(false)
    let minSliderPosition = $state(0)
    let maxSliderPosition = $state(0)
    let maxOnTop = $state(false)
    let sliderContainer: HTMLDivElement

    $effect(() => {
        if (reset) {
            minValue = min
            maxValue = max
        }
    })

    $effect(() => {
        const minVal = Number(minValue)
        const maxVal = Number(maxValue)
        const midpoint = min + (max - min) / 2
        const areClose = Math.abs(maxVal - minVal) <= 4

        if (minVal < midpoint && maxVal < midpoint && areClose) {
            maxOnTop = true
        } else if (minVal >= midpoint && maxVal >= midpoint && areClose) {
            maxOnTop = false
        }
    })

    const thumbWidth = 8

    const minSliderId = `slider-min-${Math.floor(Math.random() * 10000)}`
    const maxSliderId = `slider-max-${Math.floor(Math.random() * 10000)}`

    const handleMinChange = (): void => {
        minValue = Math.min(maxValue ?? max, Number(minValue))
        updatePositions()
        updateTrackHighlight()
    }

    const handleMaxChange = (): void => {
        if (isDraggingMax && Number(maxValue) < Number(minValue)) maxValue = minValue
        updatePositions()
        updateTrackHighlight()
    }

    const handleMinPointerDown = (): void => {
        isDraggingMin = true
        updatePositions()
    }

    const handleMaxPointerDown = (): void => {
        isDraggingMax = true
        updatePositions()
    }

    const handlePointerUp = (): void => {
        isDraggingMin = false
        isDraggingMax = false
    }

    const updateTrackHighlight = (): void => {
        if (!sliderContainer) return

        const minPercent =
            minValue === null || minValue === undefined || minValue === ''
                ? 0
                : ((Number(minValue) - min) / (max - min)) * 100
        const maxPercent =
            maxValue === null || maxValue === undefined || maxValue === ''
                ? 100
                : ((Number(maxValue) - min) / (max - min)) * 100

        sliderContainer.style.background = `linear-gradient(to right, #e5e7eb ${minPercent}%, var(--color-primaryColor, #3b82f6) ${minPercent}%, var(--color-primaryColor, #3b82f6) ${maxPercent}%, #e5e7eb ${maxPercent}%)`
    }

    const updatePositions = (): void => {
        if (!sliderContainer) return

        const containerRect = sliderContainer.getBoundingClientRect()
        const containerWidth = containerRect.width

        const minEffectiveWidth = containerWidth - thumbWidth
        const minEffectiveStart = thumbWidth / 2
        const minPercentage =
            minValue === null || minValue === undefined || minValue === '' ? 0 : (Number(minValue) - min) / (max - min)
        const minAdjustedPosition = minEffectiveStart + minPercentage * minEffectiveWidth
        minSliderPosition = (minAdjustedPosition / containerWidth) * 100

        const maxEffectiveWidth = containerWidth - thumbWidth
        const maxEffectiveStart = thumbWidth / 2
        const maxPercentage =
            maxValue === null || maxValue === undefined || maxValue === '' ? 1 : (Number(maxValue) - min) / (max - min)
        const maxAdjustedPosition = maxEffectiveStart + maxPercentage * maxEffectiveWidth
        maxSliderPosition = (maxAdjustedPosition / containerWidth) * 100
    }

    $effect(() => {
        updatePositions()
        updateTrackHighlight()
    })
</script>

<div class="relative w-full">
    {#if label}
        <div class="mb-1 flex w-full items-center justify-between text-sm tracking-wide text-slate-500">
            <span>{label}</span>
            <span class="flex rounded bg-slate-800 p-1 text-xs">{minValue} - {maxValue}</span>
        </div>
    {/if}
    <div class="flex items-center gap-2">
        <div class="relative h-1 grow rounded-md" bind:this={sliderContainer}>
            <div class="relative w-full">
                <div>
                    <input
                        type="range"
                        bind:value={minValue}
                        {min}
                        {max}
                        {step}
                        oninput={handleMinChange}
                        onpointerdown={handleMinPointerDown}
                        onpointerup={handlePointerUp}
                        id={minSliderId}
                        tabindex="-1"
                        class="pointer-events-none absolute top-1 bottom-0 m-auto h-full w-full cursor-pointer appearance-none bg-transparent outline-none [&::-webkit-slider-runnable-track]:h-0.5 [&::-webkit-slider-thumb]:pointer-events-auto [&::-webkit-slider-thumb]:relative [&::-webkit-slider-thumb]:z-20 [&::-webkit-slider-thumb]:-mt-1.25 [&::-webkit-slider-thumb]:h-2 [&::-webkit-slider-thumb]:w-2 [&::-webkit-slider-thumb]:cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-(--color-primaryColor,#3b82f6) [&::-webkit-slider-thumb]:shadow-md"
                        class:z-20={!maxOnTop}
                        class:z-10={maxOnTop}
                        aria-label={label ? `${label} minimum` : 'Minimum value'} />

                    <input
                        type="range"
                        bind:value={maxValue}
                        {min}
                        {max}
                        {step}
                        oninput={handleMaxChange}
                        onpointerdown={handleMaxPointerDown}
                        onpointerup={handlePointerUp}
                        id={maxSliderId}
                        tabindex="-1"
                        class="pointer-events-none absolute top-1 bottom-0 m-auto h-full w-full cursor-pointer appearance-none bg-transparent outline-none [&::-webkit-slider-runnable-track]:h-0.5 [&::-webkit-slider-thumb]:pointer-events-auto [&::-webkit-slider-thumb]:relative [&::-webkit-slider-thumb]:z-10 [&::-webkit-slider-thumb]:-mt-1.25 [&::-webkit-slider-thumb]:h-2 [&::-webkit-slider-thumb]:w-2 [&::-webkit-slider-thumb]:cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-(--color-primaryColor,#3b82f6) [&::-webkit-slider-thumb]:shadow-md"
                        class:z-20={maxOnTop}
                        class:z-10={!maxOnTop}
                        aria-label={label ? `${label} maximum` : 'Maximum value'} />
                </div>
            </div>

            <div
                class={`pointer-events-none absolute -translate-x-1/2 transform rounded bg-primaryColor px-2 py-1 text-xs text-white transition-opacity duration-200 ${isDraggingMin ? 'opacity-100' : 'opacity-0'} -top-8.5 left-[${minSliderPosition}%]`}
                style="top: -34px; left: {minSliderPosition}%;">
                {minValue}
                <div
                    class="absolute -bottom-1 left-1/2 -ml-1 border-t-4 border-r-4 border-l-4 border-solid border-primaryColor border-r-transparent border-l-transparent">
                </div>
            </div>

            <div
                class={`pointer-events-none absolute -translate-x-1/2 transform rounded bg-primaryColor px-2 py-1 text-xs text-white transition-opacity duration-200 ${isDraggingMax ? 'opacity-100' : 'opacity-0'}`}
                style="top: -34px; left: {maxSliderPosition}%;">
                {maxValue}
                <div
                    class="absolute -bottom-1 left-1/2 -ml-1 border-t-4 border-r-4 border-l-4 border-solid border-primaryColor border-r-transparent border-l-transparent">
                </div>
            </div>
        </div>
    </div>
</div>
