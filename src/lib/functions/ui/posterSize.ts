const CAROUSEL_WIDTHS = [154, 185, 342, 500]
const GRID_WIDTHS = [154, 185, 342, 500, 780]

export const posterSize = (scalePct: number, container: 'carousel' | 'grid'): string => {
    const availableWidths = container.toLowerCase() === 'carousel' ? CAROUSEL_WIDTHS : GRID_WIDTHS
    const targetWidth = 342 * (scalePct / 100)
    for (const w of availableWidths) {
        if (targetWidth <= w) return `w${w}`
    }

    return container.toLowerCase() === 'carousel' ? 'w500' : 'original'
}
