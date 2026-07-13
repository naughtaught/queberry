let intersectionObserver: IntersectionObserver

const ensureIntersectionObserver = (): void => {
    if (intersectionObserver) return

    intersectionObserver = new IntersectionObserver((entries) => {
        entries.forEach((entry) => {
            const eventName = entry.isIntersecting ? 'enterViewport' : 'exitViewport'
            entry.target.dispatchEvent(new CustomEvent(eventName))
        })
    })
}

export const viewport = (element: Element): { destroy(): void } => {
    ensureIntersectionObserver()

    intersectionObserver.observe(element)

    return {
        destroy() {
            intersectionObserver.unobserve(element)
        },
    }
}
