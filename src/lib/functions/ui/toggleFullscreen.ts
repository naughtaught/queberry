import { isAppFullscreen } from '$lib/stores/app'
import { getCurrentWindow, LogicalSize, PhysicalPosition } from '@tauri-apps/api/window'
import { handleError } from '$lib/functions/errors/errorHandling'

const WINDOWED_STATE_KEY = 'windowedState'

interface WindowedState {
    width: number
    height: number
    x: number
    y: number
}

const getStoredWindowedState = (): WindowedState => {
    const stored = localStorage.getItem(WINDOWED_STATE_KEY)
    if (stored) return JSON.parse(stored)
    return { width: 1280, height: 720, x: -1, y: -1 }
}

const persistWindowedState = (state: WindowedState): void => {
    localStorage.setItem(WINDOWED_STATE_KEY, JSON.stringify(state))
}

let windowedState = getStoredWindowedState()

export const toggleFullscreen = async (): Promise<void> => {
    const window = getCurrentWindow()
    try {
        const isFullscreen = await window.isFullscreen()

        if (!isFullscreen) {
            const position = await window.outerPosition()
            const size = await window.outerSize()
            windowedState = {
                width: size.width,
                height: size.height,
                x: position.x,
                y: position.y,
            }
            persistWindowedState(windowedState)
        }

        await window.setFullscreen(!isFullscreen)
        isAppFullscreen.set(!isFullscreen)

        if (isFullscreen) {
            await window.setSize(new LogicalSize(windowedState.width, windowedState.height))

            if (windowedState.x === -1 && windowedState.y === -1) {
                await window.center()
            } else {
                await window.setPosition(new PhysicalPosition(windowedState.x, windowedState.y))
            }
        }
    } catch (error) {
        handleError(error, { context: 'toggle fullscreen failed' })
    }
}
