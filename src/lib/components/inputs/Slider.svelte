<script lang="ts">
    let {
        min,
        max,
        value = $bindable(),
        step = 1,
        func,
        label,
        tooltip = true,
        tooltipPosition = true,
        zeroPoint = false,
    } = $props()

    let isDragging = $state(false)
    let sliderPosition = $state(0)
    let sliderElement: HTMLInputElement

    const thumbWidth = 8

    const handleChange = (): void => {
        const constrainedValue = Math.max(min, Math.min(max, value))
        updatePosition(constrainedValue)
        updateGradient(constrainedValue)
        func()
    }

    const handleBlur = (): void => {
        if (value < min) {
            value = min
        } else if (value > max) {
            value = max
        }
        updatePosition(value)
        updateGradient(value)
        func()
    }

    const handlePointerDown = (): void => {
        isDragging = true
        updatePosition(value)
        updateGradient(value)
    }

    const handlePointerUp = (): boolean => (isDragging = false)

    const updatePosition = (val: number): void => {
        if (!sliderElement) return

        const sliderRect = sliderElement.getBoundingClientRect()
        const sliderWidth = sliderRect.width

        const effectiveWidth = sliderWidth - thumbWidth
        const effectiveStart = thumbWidth / 2

        const percentage = (val - min) / (max - min)

        const adjustedPosition = effectiveStart + percentage * effectiveWidth

        sliderPosition = (adjustedPosition / sliderWidth) * 100
    }

    const updateGradient = (val: number): void => {
        if (!sliderElement) return

        if (zeroPoint) {
            const zeroPercent = ((0 - min) / (max - min)) * 100

            const valuePercent = ((val - min) / (max - min)) * 100

            if (val >= 0) {
                const startPercent = Math.max(0, Math.min(100, zeroPercent))
                const endPercent = Math.max(0, Math.min(100, valuePercent))
                sliderElement.style.setProperty('--gradient-start', `${startPercent}%`)
                sliderElement.style.setProperty('--gradient-end', `${endPercent}%`)
            } else {
                const startPercent = Math.max(0, Math.min(100, valuePercent))
                const endPercent = Math.max(0, Math.min(100, zeroPercent))
                sliderElement.style.setProperty('--gradient-start', `${startPercent}%`)
                sliderElement.style.setProperty('--gradient-end', `${endPercent}%`)
            }
        } else {
            const percentage = ((val - min) / (max - min)) * 100
            sliderElement.style.setProperty('--gradient-start', '0%')
            sliderElement.style.setProperty('--gradient-end', `${Math.max(0, Math.min(100, percentage))}%`)
        }
    }

    $effect(() => {
        if (sliderElement) {
            updateGradient(value)
        }
    })
</script>

<div class="relative w-full">
    {#if label}
        <div class="flex items-center text-sm text-textColor">
            <span>{label}</span>
            <div class="flex items-center">
                <input
                    type="number"
                    bind:value
                    oninput={handleChange}
                    onblur={handleBlur}
                    {min}
                    {max}
                    {step}
                    class="ml-1 w-9 rounded border border-neutral-300 text-center text-xs text-textColor" />
                <span class="ml-1">%</span>
            </div>
        </div>
    {/if}
    <input
        type="range"
        {min}
        {max}
        {step}
        bind:value
        bind:this={sliderElement}
        oninput={handleChange}
        onpointerdown={handlePointerDown}
        onpointerup={handlePointerUp}
        aria-label={label || 'Slider'}
        class="h-1 w-full cursor-pointer appearance-none rounded-full bg-neutral-400 [&::-webkit-slider-thumb]:h-2 [&::-webkit-slider-thumb]:w-2 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-primaryShade-200 [&::-webkit-slider-thumb]:shadow-md"
        style="background: linear-gradient(to right, 
            var(--color-neutral-400) 0%,
            var(--color-neutral-400) var(--gradient-start, 0%),
            var(--color-primaryColor) var(--gradient-start, 0%),
            var(--color-primaryColor) var(--gradient-end, 0%),
            var(--color-neutral-400) var(--gradient-end, 0%),
            var(--color-neutral-400) 100%
        );" />
    {#if tooltip}
        <div
            class={`pointer-events-none absolute -translate-x-1/2 transform rounded bg-primaryColor px-2 py-1 text-xs text-textColor transition-opacity duration-200 ${isDragging ? 'opacity-100' : 'opacity-0'}`}
            style:left={`${sliderPosition}%`}
            style:top={tooltipPosition ? '-1.5rem' : 'auto'}
            style:bottom={tooltipPosition ? 'auto' : '-1.5rem'}>
            {value}
            <div
                class:border-t-4={tooltipPosition}
                class:border-b-4={!tooltipPosition}
                class:bottom-[-4px]={tooltipPosition}
                class:top-[-4px]={!tooltipPosition}
                class="absolute left-1/2 -ml-1 border-r-4 border-l-4 border-solid border-primaryColor border-r-transparent border-l-transparent">
            </div>
        </div>
    {/if}
</div>
