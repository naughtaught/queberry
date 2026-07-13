import type { App } from '$lib/types/app'
import { invoke } from '@tauri-apps/api/core'

export const invokeFunction = async (emit: string, args: Record<string, unknown>): Promise<App.Response> => {
    const response: App.Response = await invoke(emit, args)

    return response
}
