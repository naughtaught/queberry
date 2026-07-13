<script lang="ts">
    import { getRatingColor } from "$lib/functions/ui/getRatingColor"

    let {
        index = $bindable(0),
        isActive = false,
        baseOpacity = 30,
        text,
        onClick = $bindable(() => {}),
        onHover = $bindable(() => {}),
    } = $props()

    const getRatingColorValue = (ratingIndex: number): string => {
        return `var(--color-${getRatingColor(ratingIndex, 'bg').replace('bg-', '')})`
    }

    const getOpacityStyle = (): string => {
        return isActive ? '' : `opacity: ${baseOpacity / 100};`
    }
</script>

<button
    type="button"
    class="h-full flex-1 border-0 bg-transparent px-0"
    onmouseenter={() => onHover(index)}
    onclick={(e) => {
        e.preventDefault()
        e.stopPropagation()
        onClick(index)
    }}
    onkeydown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault()
            e.stopPropagation()
            onClick(index)
        }
    }}
    aria-label="Rating segment {index + 1}">
    <div
        class="flex h-full w-full items-center justify-center transition-opacity hover:opacity-100"
        style={`background: ${getRatingColorValue((index + 1) * 10)}; ${getOpacityStyle()};`}>
        {#if isActive && text > 0}
            <span class="pointer-events-none text-xs font-bold text-black select-none">{Number(text)}</span>
        {/if}
    </div>
</button>
