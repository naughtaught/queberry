import { get, readable, writable } from 'svelte/store'
import { type App } from '$lib'

const appStateDefault = readable<App.State>({
    isFullscreen: false,
})

export const appState = writable<App.State>(get(appStateDefault))
