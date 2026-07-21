<script lang="ts">
    import { afterNavigate, beforeNavigate, goto } from '$app/navigation'
    import { navigating, page } from '$app/state'
    import { setScrollPosition } from '$lib/functions/ui/setScrollPosition'
    import { isValidRoute } from '$lib/functions/utility/isValidRoute'
    import { loadingStates, scrollPositions } from '$lib/stores/app'
    import { activeGridViewSideBarItem } from '$lib/stores/pages'
    import type { Api } from '$lib/types/api'
    import GridViewMediaCard from '$lib/components/cards/GridViewMediaCard.svelte'

    let { mediaData = $bindable() } = $props()
    const hoverTime = 2000
    const itemRefs: HTMLElement[] = $state([])

    let container = $state<HTMLElement>()
    let currentIndex = $state(0)
    let focusedMediaItem: Api.MediaItem | null = $state(null)
    let hoverTimer: ReturnType<typeof setTimeout> | null = $state(null)
    let isAtBottom = $state(false)

    const handleScroll = (): void => {
        if (!container) return

        const scrollBottom = container.scrollHeight - container.scrollTop - container.clientHeight

        isAtBottom = scrollBottom < 20
    }

    const handleKeydown = (event: KeyboardEvent): void => {
        if (!container?.contains(document.activeElement)) return

        if (!itemRefs.some((ref) => ref === document.activeElement)) {
            event.preventDefault()
            currentIndex = 0
            itemRefs[0]?.focus()
            return
        }

        switch (event.key) {
            case 'ArrowRight':
                event.preventDefault()
                currentIndex = (currentIndex + 1) % mediaData.length
                itemRefs[currentIndex]?.focus()
                break
            case 'ArrowLeft':
                event.preventDefault()
                currentIndex = (currentIndex - 1 + mediaData.length) % mediaData.length
                itemRefs[currentIndex]?.focus()
                break
            case 'Enter':
                event.preventDefault()
                if (focusedMediaItem) {
                    goto(`/details/?id=${focusedMediaItem.id}&type=${focusedMediaItem.type}`)
                }
                break
        }
    }

    beforeNavigate(() => {
        if (container && page.route.id) setScrollPosition(container, page.route.id)
    })

    afterNavigate(() => {
        if ((navigating.type === 'popstate' || navigating.from?.route.id === '/video') && container) {
            const route = page.url.pathname?.replace('/', '')

            if (!isValidRoute(route)) return

            container.scrollTop = $scrollPositions[route].y
        } else {
            if (container && page.route.id) setScrollPosition(container, page.route.id)
        }
        if (hoverTimer !== null) {
            clearTimeout(hoverTimer)
            hoverTimer = null
        }
    })

    const handleFocus = (media: Api.MediaItem, index: number): void => {
        focusedMediaItem = media
        currentIndex = index

        if (hoverTimer !== null) clearTimeout(hoverTimer)

        hoverTimer = setTimeout(() => {
            $activeGridViewSideBarItem = media
            hoverTimer = null
        }, hoverTime)
    }

    const handleBlur = (): void => {
        if (hoverTimer !== null) {
            clearTimeout(hoverTimer)
            hoverTimer = null
        }
    }
</script>

<div class="relative ml-5 flex flex-col">
    <section
        id="gridViewSection"
        role="presentation"
        bind:this={container}
        class="flex h-screen flex-wrap gap-4 overflow-y-auto p-5"
        onkeydown={handleKeydown}
        onscroll={handleScroll}>
        {#each mediaData as media, index (`${media.id}-${index}`)}
            <div
                tabindex="-1"
                bind:this={itemRefs[index]}
                onfocus={() => handleFocus(media, index)}
                onblur={handleBlur}>
                <GridViewMediaCard bind:mediaData {media} {index} dataLength={mediaData.length} />
            </div>
        {/each}
    </section>

    {#if isAtBottom && $loadingStates.isGridViewLoadingMore}
        <div class="pointer-events-none absolute right-0 bottom-0 left-0 flex justify-center pb-4">
            <p class="rounded-full bg-backgroundColor/50 px-4 py-2 text-sm text-textColor shadow-lg backdrop-blur-sm">
                Loading more...
            </p>
        </div>
    {/if}
</div>
