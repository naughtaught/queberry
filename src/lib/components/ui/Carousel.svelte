<script lang="ts">
    import { loadingStates } from '$lib/stores/app'
    import { onMount } from 'svelte'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'

    const { children, title = 'Carousel', loadingState } = $props()

    let element: HTMLElement | null = $state(null)

    export const scrollTo = (options: { left?: number; top?: number; behavior?: 'smooth' | 'auto' }): void => {
        if (element) element.scrollTo(options)
    }

    const saveScrollPosition = (): void => {
        if (!element || !title) return
        const position = element.scrollLeft

        sessionStorage.setItem(`carousel-scroll-${title.replace(' ', '-')}`, position.toString())
    }

    onMount(() => {
        if (element && title) {
            const scrollPosition = sessionStorage.getItem(`carousel-scroll-${title.replace(' ', '-')}`)
            if (scrollPosition) {
                element.classList.add('scroll-auto')
                element.classList.remove('scroll-smooth')
                element.scrollLeft = Number(scrollPosition)
                setTimeout(() => {
                    element?.classList.remove('scroll-auto')
                    element?.classList.add('scroll-smooth')
                }, 0)
            }
        }
    })
</script>

<div class="w-full">
    <h2 class="mb-3 font-black tracking-wide text-slate-300">{title}</h2>
    <div
        class="w-full snap-x overflow-x-auto overflow-y-hidden scroll-smooth"
        bind:this={element}
        onscroll={saveScrollPosition}
        data-carousel-id={title.replace(' ', '-')}>
        {#if $loadingStates[loadingState as keyof typeof $loadingStates]}
            <div class="h-75 w-full"><LoadingSpinner /></div>
        {:else}
            <div class="flex gap-x-3">
                {@render children()}
            </div>
        {/if}
    </div>
</div>
