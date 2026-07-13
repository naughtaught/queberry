<script lang="ts">
    import { clickOutside } from '$lib/functions/utility/useClickOutside'
    import DragBar from '$lib/components/bars/DragBar.svelte'

    type PositionType = 'center' | 'top' | 'top-left'

    const {
        onClose,
        roundedBottom = true,
        children,
        background = 'bg-backgroundColor',
        position = 'center' as PositionType,
    } = $props()

    const positionClasses = {
        center: 'items-center justify-center',
        top: 'items-start justify-center pt-12',
        'top-left': 'items-start justify-start pt-16 pl-16',
    }
</script>

<DragBar />
<section
    role="presentation"
    id="modalOverlay"
    class="fixed inset-0 z-40 flex h-full w-full {positionClasses[position]} bg-black/50 backdrop-blur-sm">
    <section
        use:clickOutside
        onclickOutside={onClose}
        class="{background} pointer-events-auto relative max-w-[80vw] rounded-md shadow-2xl"
        class:rounded-bl-none={!roundedBottom}>
        {@render children()}
    </section>
</section>
