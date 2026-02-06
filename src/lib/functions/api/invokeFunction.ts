import type { Api } from '$lib/types/api'
import { invoke } from '@tauri-apps/api/core'

export const invokeFunction = async (emit: string, args: Record<string, unknown>): Promise<Api.ApiResponse> => {
    const response: Api.ApiResponse = await invoke(emit, args)

    if (!response.success) throw response.error

    return response
}
