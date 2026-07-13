import { scrollPositions } from '$lib/stores/app'
import { get } from 'svelte/store'

export const setScrollPosition = (scrollContainer: HTMLElement, scrollPage: string): void => {
    const currentScrollPositions = get(scrollPositions)

    scrollPositions.set({
        ...currentScrollPositions,
        [scrollPage.replace('/', '')]: {
            x: scrollContainer.scrollLeft,
            y: scrollContainer.scrollTop,
        },
    })
}
