<script lang="ts">
    import { settings } from '$lib/stores/user'

    const { onUpdate } = $props()

    let draggedIndex: number | null = $state(null)
    let dragOverIndex: number | null = $state(null)
    let dragElement: HTMLElement | null = $state(null)

    const handleDragStart = (e: MouseEvent, index: number): void => {
        e.preventDefault()
        draggedIndex = index
        dragElement = e.currentTarget as HTMLElement

        if (dragElement) {
            dragElement.style.opacity = '0.5'
            dragElement.style.cursor = 'grabbing'
        }

        document.addEventListener('mousemove', handleDragMove)
        document.addEventListener('mouseup', handleDragEnd)
    }

    const handleDragMove = (e: MouseEvent): void => {
        if (draggedIndex === null) return

        const elements = document.querySelectorAll('[data-draggable-item]')
        let targetIndex: number | null = null

        elements.forEach((el, idx) => {
            const rect = el.getBoundingClientRect()

            if (e.clientY >= rect.top && e.clientY <= rect.bottom) {
                targetIndex = idx
            }
        })

        if (targetIndex !== null && targetIndex !== dragOverIndex) {
            dragOverIndex = targetIndex
            reorderItems(draggedIndex, dragOverIndex)
            draggedIndex = dragOverIndex
        }
    }

    const handleDragEnd = (): void => {
        if (dragElement) {
            dragElement.style.opacity = ''
            dragElement.style.cursor = ''
        }

        draggedIndex = null
        dragOverIndex = null
        dragElement = null

        document.removeEventListener('mousemove', handleDragMove)
        document.removeEventListener('mouseup', handleDragEnd)
    }

    const reorderItems = (from: number, to: number): void => {
        if (from === to) return

        const items = $settings.indexerSortCriteria.slice()
        const [moved] = items.splice(from, 1)
        items.splice(to, 0, moved)

        settings.update((s) => ({
            ...s,
            indexerSortCriteria: items,
        }))

        onUpdate()
    }

    const toggleDirection = (index: number): void => {
        const items = $settings.indexerSortCriteria.slice()
        items[index] = {
            ...items[index],
            order: items[index].order === 'asc' ? 'desc' : 'asc',
        }

        settings.update((s) => ({
            ...s,
            indexerSortCriteria: items,
        }))
    }
</script>

<div class="flex flex-col gap-2">
    {#each $settings.indexerSortCriteria as option, index (index)}
        <div
            role="button"
            tabindex="0"
            data-draggable-item
            class="group flex w-full cursor-grab items-center justify-between rounded-lg border border-white/5 bg-white/5 p-3 transition-all hover:bg-white/10 active:cursor-grabbing"
            class:bg-white={dragOverIndex === index}
            class:opacity-50={draggedIndex === index}
            onmousedown={(e) => handleDragStart(e, index)}>
            <div class="pointer-events-none flex items-center space-x-3">
                <span class="text-[10px] font-black text-slate-500">{index + 1}</span>
                <span class="text-xs font-bold">{option.key}</span>
            </div>
            <button
                type="button"
                onclick={() => toggleDirection(index)}
                class="pointer-events-auto ml-2 text-primaryColor transition-opacity hover:opacity-80"
                title={option.order === 'asc' ? 'Ascending' : 'Descending'}>
                {option.order === 'asc' ? '↑' : '↓'}
            </button>
        </div>
    {/each}
</div>
