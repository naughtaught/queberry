export const clickOutside = (element: HTMLElement, options?: { ignore?: string }  ): { destroy(): void } => {
    const handleClick = (event: Event): void => {
        const target = event.target as HTMLElement
        const ignore = options?.ignore
        
        if (!event.target || (ignore && target.closest(ignore))) {
            return
        }
        if (element && !element.contains(target) && !event.defaultPrevented) {
            element.dispatchEvent(new CustomEvent('clickOutside'))
        }
    }

    document.addEventListener('click', handleClick, true)

    return {
        destroy() {
            document.removeEventListener('click', handleClick, true)
        },
    }
}
