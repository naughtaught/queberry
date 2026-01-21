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
    } = $props()

    let isDragging = $state(false)
    let sliderPosition = $state(0)
    let sliderElement: HTMLInputElement

    const thumbWidth = 20

    const handleChange = () => {
        const constrainedValue = Math.max(min, Math.min(max, value))

        updatePosition(constrainedValue)

        const percentage = ((constrainedValue - min) / (max - min)) * 100
        sliderElement.style.setProperty('--slider-progress', `${percentage}%`)

        func()
    }

    const handleBlur = () => {
        if (value < min) {
            value = min
        } else if (value > max) {
            value = max
        }
        updatePosition(value)

        func()
    }

    const handlePointerDown = () => {
        isDragging = true
        updatePosition(value)
    }

    const handlePointerUp = () => (isDragging = false)

    const updatePosition = (val: number) => {
        if (!sliderElement) return

        const sliderRect = sliderElement.getBoundingClientRect()
        const sliderWidth = sliderRect.width

        const effectiveWidth = sliderWidth - thumbWidth
        const effectiveStart = thumbWidth / 2

        const percentage = (val - min) / (max - min)

        const adjustedPosition = effectiveStart + percentage * effectiveWidth

        sliderPosition = (adjustedPosition / sliderWidth) * 100
    }

    $effect(() => {
        const percentage = ((value - min) / (max - min)) * 100
        sliderElement.style.setProperty('--slider-progress', `${percentage}%`)
    })
</script>

<div class="relative w-full">
    {#if label}
        <div class="text-text flex items-center text-sm">
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
                    class="text-text ml-1 w-9 rounded border border-neutral-300 text-center text-xs" />
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
        class="h-1 w-full cursor-pointer appearance-none rounded-full bg-neutral-400 bg-linear-to-r from-primary to-primary bg-no-repeat [&::-webkit-slider-thumb]:h-0 [&::-webkit-slider-thumb]:w-0 [&::-webkit-slider-thumb]:appearance-none"
        style="background-size: var(--slider-progress) 100%;" />
    {#if tooltip}
        <div
            class={`pointer-events-none absolute -translate-x-1/2 transform rounded bg-primary px-2 py-1 text-xs text-white transition-opacity duration-200 ${isDragging ? 'opacity-100' : 'opacity-0'}`}
            style:left={`${sliderPosition}%`}
            style:top={tooltipPosition ? '-1.5rem' : 'auto'}
            style:bottom={tooltipPosition ? 'auto' : '-1.5rem'}>
            {value}
            <div
                class:border-t-4={tooltipPosition}
                class:border-b-4={!tooltipPosition}
                class:bottom-[-4px]={tooltipPosition}
                class:top-[-4px]={!tooltipPosition}
                class="absolute left-1/2 -ml-1 border-r-4 border-l-4 border-solid border-primary border-r-transparent border-l-transparent">
            </div>
        </div>
    {/if}
</div>
